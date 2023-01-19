/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::error::{
    unclosed_block_comment, unclosed_byte_vector, unclosed_special, unclosed_string, Error,
};
use crate::input::indices::CharIndex;
use crate::input::iter::CharIndices;
use crate::lexer::internals::{IteratorState, State};
use crate::lexer::token::{Span, Token, TokenKind};
use crate::Sourced;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct TokenIter<'a> {
    state_stack: Vec<IteratorState>,
    chars: CharIndices<'a>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! return_token {
    ($current_state:expr, $char_index:expr, $kind:ident => $state:ident) => {
        $current_state.set_state(State::$state);
        return_token!($current_state, $char_index, $kind);
    };
    ($current_state:expr, $char_index:expr, $kind:ident) => {
        let token = Some(Ok(Token::new(
            TokenKind::$kind,
            $current_state.token_starts_at(),
            $char_index.index(),
        )));
        println!("{:?}", token);
        return token;
    };
}

macro_rules! return_error {
    ($current_state:expr, $char_index:expr, $error_fn:ident, $state:ident) => {
        $current_state.set_state(State::$state);
        let err = Some(Err($error_fn(Span::new(
            $current_state.token_starts_at().character(),
            $char_index.index().character(),
        ))));
        println!("{:?}", err);
        return err;
    };
    ($current_state:expr, $char_index:expr, $error_fn:ident) => {
        return_error!($current_state, $char_index, $error_fn, Nothing);
    };
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<'a> From<CharIndices<'a>> for TokenIter<'a> {
    fn from(chars: CharIndices<'a>) -> Self {
        Self {
            state_stack: Vec::default(),
            chars,
        }
    }
}

impl Sourced for TokenIter<'_> {
    #[inline(always)]
    fn source_str(&self) -> &str {
        self.chars.source_str()
    }
}

impl Iterator for TokenIter<'_> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current_state = IteratorState::default();
        let mut last_char_index = CharIndex::new(0, 0, '\u{00}');
        while let Some(char_index) = self.next_char() {
            println!("match ({:?}, {:?}) ...", current_state, char_index);
            last_char_index = char_index;
            match (current_state.state(), char_index.character()) {
                // --------------------------------------------------------------------------------
                // White space handling
                (State::Nothing, c) if c.is_whitespace() => {
                    current_state.set_state(State::InWhitespace);
                }
                (State::InWhitespace, c) if c.is_whitespace() => {}
                (State::Nothing | State::InWhitespace, '\n') => {
                    current_state.set_state(State::InWhitespace);
                }
                // --------------------------------------------------------------------------------
                // Single character (mostly) tokens
                (State::Nothing | State::InWhitespace, '(') => {
                    current_state.set_token_start(&char_index);
                    return_token!(current_state, char_index, OpenParenthesis => Nothing);
                }
                (State::Nothing | State::InWhitespace, ')') => {
                    current_state.set_token_start(&char_index);
                    return_token!(current_state, char_index, CloseParenthesis => Nothing);
                }
                (State::Nothing | State::InWhitespace, '\'') => {
                    current_state.set_token_start(&char_index);
                    return_token!(current_state, char_index, Quote => Nothing);
                }
                (State::Nothing | State::InWhitespace, '`') => {
                    current_state.set_token_start(&char_index);
                    return_token!(current_state, char_index, QuasiQuote => Nothing);
                }
                (State::Nothing | State::InWhitespace, ',') => {
                    current_state.set_token_start(&char_index);
                    if let Some(next_char) = self.peek_next_char() {
                        if next_char == &'@' {
                            let next_char = self.next_char().unwrap();
                            return_token!(
                                current_state,
                                next_char,
                                UnquoteSplicing => Nothing
                            );
                        }
                    }
                    return_token!(current_state, char_index, Unquote => Nothing);
                }
                (State::Nothing | State::InWhitespace, '.') => {
                    current_state.set_token_start(&char_index);
                    return_token!(current_state, char_index, Dot => Nothing);
                }
                // --------------------------------------------------------------------------------
                // Identifier values
                (State::Nothing | State::InWhitespace, '|') => {
                    current_state.set_token_start(&char_index);
                    current_state.set_state(State::InVBarIdentifier);
                }
                (State::Nothing | State::InWhitespace, c) if c.is_ascii_alphabetic() => {
                    current_state.set_token_start(&char_index);
                    current_state.set_state(State::InIdentifier);
                }
                (State::InIdentifier, c) if c.is_ascii_alphabetic() => {}
                (State::InIdentifier, _) => {
                    return_token!(current_state, char_index, Identifier => Nothing);
                }
                // --------------------------------------------------------------------------------
                // String values
                (State::Nothing | State::InWhitespace, '"') => {
                    current_state.set_token_start(&char_index);
                    current_state.set_state(State::InString);
                }
                (State::InString, '\\') => {
                    current_state.set_state(State::InStringEscape);
                }
                (State::InString, '"') => {
                    return_token!(current_state, char_index, String => Nothing);
                }
                (State::InString, _) => {}
                (State::InStringEscape, 'a' | 'b' | 't' | 'n' | 'r' | '"' | '|' | ' ' | '\t') => {
                    current_state.set_state(State::InString);
                }
                (State::InStringEscape, 'x') => {
                    current_state.set_state(State::InStringHexEscape);
                }
                (State::InStringHexEscape, d) if d.is_ascii_hexdigit() => {}
                (State::InStringHexEscape, _) => {
                    // TODO: validate hex string
                    current_state.set_state(State::InString);
                }
                // TODO: handle "\\\n" line endings
                // --------------------------------------------------------------------------------
                // Numeric values
                (State::Nothing | State::InWhitespace, c) if c.is_ascii_digit() => {
                    current_state.set_token_start(&char_index);
                    current_state.set_state(State::InNumeric);
                }
                // +inf.0 -inf.0 +nan.0 -nan.0
                // --------------------------------------------------------------------------------
                // Start of special forms
                (State::Nothing | State::InWhitespace, '#') => {
                    current_state.set_state(State::InSpecial);
                }
                // --------------------------------------------------------------------------------
                // Boolean values
                (State::InSpecial, 't') => {
                    current_state.set_token_start(&char_index);
                    return_token!(current_state, char_index, Boolean => Nothing);
                }
                (State::InSpecial, 'f') => {
                    current_state.set_token_start(&char_index);
                    return_token!(current_state, char_index, Boolean => Nothing);
                }
                // --------------------------------------------------------------------------------
                // Vector values
                (State::InSpecial, '(') => {
                    current_state.set_token_start(&char_index);
                    return_token!(current_state, char_index, OpenVector => Nothing);
                }
                (State::InSpecial, 'u') => {
                    current_state = self.push_state(current_state, State::InOpenByteVector);
                    current_state.set_token_start(&char_index);
                    // "#u8(" => OpenByteVector
                    current_state = self.pop_state();
                }
                // --------------------------------------------------------------------------------
                // Character values
                (State::InSpecial, '\\') => {
                    current_state.set_token_start(&char_index);
                    // TODO: peek for 'x' and hex value.
                    current_state.set_state(State::InCharacter);
                }
                // --------------------------------------------------------------------------------
                // Numeric values
                // 'b' binary radix
                // 'o' octal radix
                // 'd' decimal radix
                // 'x' hex radix
                // 'e' exact
                // 'i' inexact
                // Chez adds #<n>r where n in 2..=36
                (State::InSpecial, 'b' | 'd' | 'e' | 'i' | 'o' | 'x') => {
                    current_state.set_token_start(&char_index);
                    current_state.set_state(State::InNumeric);
                }

                // --------------------------------------------------------------------------------
                // Directives
                (State::InSpecial, '!') => {
                    current_state.set_token_start(&char_index);
                    current_state.set_state(State::InDirective);
                }
                (State::InDirective, c) if c.is_alphabetic() || c == '-' => {}
                (State::InDirective, _) => {
                    return_token!(current_state, char_index, DatumAssign => Nothing);
                }
                // --------------------------------------------------------------------------------
                // Datum references
                (State::InSpecial, c) if c.is_ascii_digit() => {
                    current_state.set_token_start(&char_index);
                    current_state.set_state(State::InDatumRef);
                }
                (State::InDatumRef, c) if c.is_ascii_digit() => {}
                (State::InDatumRef, '=') => {
                    return_token!(current_state, char_index - (1, 1), DatumAssign => Nothing);
                }
                (State::InDatumRef, '#') => {
                    return_token!(current_state, char_index - (1, 1), DatumRef => Nothing);
                }
                // --------------------------------------------------------------------------------
                // Comment Forms
                (State::Nothing | State::InWhitespace, ';') => {
                    current_state.set_token_start(&char_index);
                    current_state.set_state(State::InLineComment);
                }
                (State::InLineComment, c) if c != '\n' => {}
                (State::InLineComment, '\n') => {
                    self.push_back_char(char_index);
                    return_token!(current_state, char_index - (1, 0), LineComment => Nothing);
                }
                (State::InSpecial, '|') => {
                    current_state.set_token_start(&char_index);
                    current_state.set_state(State::InBlockComment);
                }
                (State::InSpecial, ';') => {
                    current_state.set_token_start(&char_index);
                    return_token!(current_state, char_index - (1, 1), DatumComment => Nothing);
                }
                // --------------------------------------------------------------------------------
                (State::InSpecial, c) => {
                    // push back?
                    println!(
                        "Found char {:?} at {:?}, which doesn't belong in a special",
                        c, char_index
                    );
                    return_error!(current_state, char_index, unclosed_special);
                }
                // --------------------------------------------------------------------------------
                (s, c) => {
                    unreachable!("Unexpected input; state: {:?}, char: {:?}", s, c);
                }
            }
        }
        if !self.state_stack.is_empty() {
            panic!("State Stack: {:#?}", self.state_stack);
        }
        // TODO:!! fix logic to capture last token before end!
        match current_state.state() {
            // ***** Safe Cases *****
            State::InDirective => {
                return_token!(current_state, last_char_index, Directive);
            }
            State::InIdentifier => {
                return_token!(current_state, last_char_index, Identifier);
            }
            State::InNumeric => todo!("add numeric cleanup"),
            State::InCharacter => todo!("add character cleanup"),
            State::InLineComment => {
                return_token!(current_state, last_char_index, LineComment);
            }
            // ***** Error Cases *****
            State::InVBarIdentifier => {
                panic!();
            }
            State::InSpecial => {
                return_error!(current_state, last_char_index, unclosed_special);
            }
            State::InString | State::InStringEscape | State::InStringHexEscape => {
                return_error!(current_state, last_char_index, unclosed_string);
            }
            State::InBlockComment => {
                return_error!(current_state, last_char_index, unclosed_block_comment);
            }
            State::InOpenByteVector => {
                return_error!(current_state, last_char_index, unclosed_byte_vector);
            }
            _ => None,
        }
    }
}

impl TokenIter<'_> {
    fn push_state(&mut self, current_state: IteratorState, new_state: State) -> IteratorState {
        let new_state = current_state.clone_with_new_state(new_state);
        self.state_stack.push(current_state);
        new_state
    }

    #[inline(always)]
    fn pop_state(&mut self) -> IteratorState {
        self.state_stack.pop().unwrap()
    }

    #[inline(always)]
    fn next_char(&mut self) -> Option<CharIndex> {
        self.chars.next()
    }

    #[inline(always)]
    fn peek_next_char(&mut self) -> Option<&CharIndex> {
        self.chars.peek()
    }

    #[inline(always)]
    fn push_back_char(&mut self, index: CharIndex) {
        self.chars.push_back(index)
    }

    #[inline(always)]
    pub fn token_str(&self, token: &Token) -> &str {
        self.get(token.byte_span().as_range()).unwrap()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
