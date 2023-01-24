/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::error::{
    invalid_byte_vector_prefix, invalid_char_input, invalid_datum_label, invalid_escape_string,
    invalid_identifier_input, unclosed_block_comment, unclosed_special, unclosed_string, Error,
};
use crate::input::indices::CharIndex;
use crate::input::iter::CharIndices;
use crate::lexer::internals::{IteratorState, State};
use crate::lexer::token::{Span, Token, TokenKind};
use crate::Sourced;
use unicode_categories::UnicodeCategories;

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

#[inline(always)]
pub fn is_identifier_initial(c: char) -> bool {
    !(c.is_ascii_digit()
        || c.is_separator()
        || c.is_control()
        || [
            '(', ')', '[', ']', '{', '}', '"', ',', '\'', '`', ';', '#', '|', '\\',
        ]
        .contains(&c))
}

#[inline(always)]
pub fn is_identifier_subsequent(c: char) -> bool {
    is_identifier_initial(c) || c.is_ascii_digit()
}

#[inline(always)]
pub fn is_vbar_identifier(c: char) -> bool {
    !(c == '|' || c.is_other_private_use())
}

#[inline(always)]
pub fn is_sign_subsequent(c: char) -> bool {
    is_identifier_initial(c) || ['+', '-', '@'].contains(&c)
}

#[inline(always)]
pub fn is_dot_subsequent(c: char) -> bool {
    is_sign_subsequent(c) || c == '.'
}

#[inline(always)]
pub fn is_mnemonic_escape(c: char) -> bool {
    ['a', 'b', 't', 'n', 'r', '"', '\\', '|'].contains(&c)
}

#[inline(always)]
pub fn is_numeric_prefix(c: char) -> bool {
    ['b', 'd', 'e', 'i', 'o', 'x'].contains(&c)
}

#[inline(always)]
pub fn is_directive(c: char) -> bool {
    c.is_alphabetic() || c == '-'
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! state_change_at {
    ($char_index:expr, $current_state:expr => $state:ident) => {
        state_change_at!($char_index, $current_state => State::$state);
    };
    ($char_index:expr, $current_state:expr => $state:expr) => {
        $current_state.set_token_start(&$char_index);
        state_change!($current_state => $state);
    };
}

macro_rules! state_change {
    ($current_state:expr => $state:ident) => {
        state_change!($current_state => State::$state);
    };
    ($current_state:expr => $state:expr) => {
        println!(
            "lexer state {:?} => {:?}",
            $current_state.state(),
            $state
        );
        $current_state.set_state($state);
    };
}

macro_rules! return_token_and_add_char {
    ($current_state:expr, $char_index:expr, $kind:ident => $state:ident) => {
        state_change!($current_state => $state);
        return_token_and_add_char!($current_state, $char_index, $kind);
    };
    ($current_state:expr, $char_index:expr, $kind:ident) => {
        let token = Some(Ok(Token::new_and_add_char(
            TokenKind::$kind,
            $current_state.token_starts_at(),
            $char_index,
        )));
        println!("lexer token {:?}", token);
        return token;
    };
}

macro_rules! return_token {
    ($current_state:expr, $char_index:expr, $kind:ident => $state:ident) => {
        state_change!($current_state => $state);
        return_token!($current_state, $char_index, $kind);
    };
    ($current_state:expr, $char_index:expr, $kind:ident) => {
        let token = Some(Ok(Token::new(
            TokenKind::$kind,
            $current_state.token_starts_at(),
            $char_index,
        )));
        println!("lexer token {:?}", token);
        return token;
    };
}

macro_rules! return_error {
    ($current_state:expr, $char_index:expr, $error_fn:ident, $state:ident) => {
        state_change!($current_state => $state);
        let err = Some(Err($error_fn(Span::new(
            $current_state.token_starts_at().character(),
            $char_index.index().character(),
        ))));
        eprintln!("lexer error {:?}", err);
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
            //println!("lexer::match ({:?}, {:?})", current_state, char_index);
            last_char_index = char_index;
            match (current_state.state(), char_index.character()) {
                // --------------------------------------------------------------------------------
                // White space handling
                (State::Nothing, c) if c.is_whitespace() => {
                    state_change!(current_state => InWhitespace);
                }
                (State::InWhitespace, c) if c.is_whitespace() => {}
                (State::Nothing | State::InWhitespace, '\n') => {
                    state_change!(current_state => InWhitespace);
                }
                // --------------------------------------------------------------------------------
                // Single character (mostly) tokens
                (State::Nothing | State::InWhitespace, '(') => {
                    current_state.set_token_start(&char_index);
                    return_token_and_add_char!(current_state, char_index, OpenParenthesis => Nothing);
                }
                (State::Nothing | State::InWhitespace, ')') => {
                    current_state.set_token_start(&char_index);
                    return_token_and_add_char!(current_state, char_index, CloseParenthesis => Nothing);
                }
                (State::Nothing | State::InWhitespace, '\'') => {
                    current_state.set_token_start(&char_index);
                    return_token_and_add_char!(current_state, char_index, Quote => Nothing);
                }
                (State::Nothing | State::InWhitespace, '`') => {
                    current_state.set_token_start(&char_index);
                    return_token_and_add_char!(current_state, char_index, QuasiQuote => Nothing);
                }
                (State::Nothing | State::InWhitespace, ',') => {
                    current_state.set_token_start(&char_index);
                    if let Some(next_char) = self.peek_next_char() {
                        if next_char == &'@' {
                            let next_char = self.next_char().unwrap();
                            return_token_and_add_char!(
                                current_state,
                                next_char,
                                UnquoteSplicing => Nothing
                            );
                        }
                    }
                    return_token_and_add_char!(current_state, char_index, Unquote => Nothing);
                }
                // --------------------------------------------------------------------------------
                // These are ambiguous
                (State::Nothing | State::InWhitespace, '+' | '-') => {
                    state_change_at!(char_index, current_state => InNumberOrIdentifier);
                }
                (State::Nothing | State::InWhitespace, '.') => {
                    state_change_at!(char_index, current_state => InDotNumberOrIdentifier);
                }
                (State::InNumberOrIdentifier, c) if c.is_ascii_digit() => {
                    state_change!(current_state => InNumeric);
                }
                (State::InNumberOrIdentifier, '.') => {
                    state_change!(current_state => InDotNumberOrIdentifier);
                }
                (State::InNumberOrIdentifier, c) if is_sign_subsequent(c) => {
                    state_change!(current_state => InPeculiarIdentifier);
                }
                (State::InDotNumberOrIdentifier, c) if c.is_ascii_digit() => {
                    state_change!(current_state => InNumeric);
                }
                (State::InDotNumberOrIdentifier, c) if is_dot_subsequent(c) => {
                    state_change!(current_state => InPeculiarIdentifier);
                }
                (State::InPeculiarIdentifier, c) if is_identifier_subsequent(c) => {}
                (State::InPeculiarIdentifier, _) => {
                    self.push_back_char(char_index);
                    return_token!(current_state, char_index, Identifier => Nothing);
                }
                (State::InDotNumberOrIdentifier, _) => {
                    self.push_back_char(char_index);
                    return_token!(current_state, char_index, Dot => Nothing);
                }
                // --------------------------------------------------------------------------------
                // Identifier values
                (State::Nothing | State::InWhitespace, '|') => {
                    state_change_at!(char_index, current_state => InVBarIdentifier);
                }
                (State::Nothing | State::InWhitespace, c) if is_identifier_initial(c) => {
                    state_change_at!(char_index, current_state => InIdentifier);
                }
                (State::InIdentifier, c) if is_identifier_subsequent(c) => {}
                (State::InIdentifier, _) => {
                    self.push_back_char(char_index);
                    return_token!(current_state, char_index, Identifier => Nothing);
                }
                (State::InVBarIdentifier, '|') => {
                    return_token_and_add_char!(current_state, char_index, Identifier => Nothing);
                }
                (State::InVBarIdentifier, '\\') => {
                    state_change!(current_state => InVBarIdentifierEscape);
                }
                (State::InVBarIdentifier, _) => {}
                (State::InVBarIdentifierEscape, c) if is_mnemonic_escape(c) => {
                    state_change!(current_state => InVBarIdentifier);
                }
                (State::InVBarIdentifierEscape, 'x') => {
                    state_change!(current_state => InVBarIdentifierHexEscape);
                }
                (State::InVBarIdentifierHexEscape, d) if d.is_ascii_hexdigit() => {
                    // R7Rs says `<hex digit>+`
                    state_change!(current_state => InVBarIdentifierHexEscapeDigits);
                }
                (State::InVBarIdentifierHexEscapeDigits, d) if d.is_ascii_hexdigit() => {}
                (State::InVBarIdentifierHexEscapeDigits, ';') => {
                    // TODO: validate hex string
                    state_change!(current_state => InVBarIdentifier);
                }
                (
                    State::InVBarIdentifierEscape
                    | State::InVBarIdentifierHexEscape
                    | State::InVBarIdentifierHexEscapeDigits,
                    _,
                ) => {
                    // TODO: Fix the span, it starts with the string start, not the escape start.
                    return_error!(current_state, char_index, invalid_identifier_input);
                }
                // --------------------------------------------------------------------------------
                // String values
                (State::Nothing | State::InWhitespace, '"') => {
                    state_change_at!(char_index, current_state => InString);
                }
                (State::InString, '\\') => {
                    state_change!(current_state => InStringEscape);
                }
                (State::InString, '"') => {
                    return_token_and_add_char!(current_state, char_index, String => Nothing);
                }
                (State::InString, _) => {}
                // TODO: '\\' ⟨intraline whitespace⟩*⟨line ending⟩ ⟨intraline whitespace⟩*
                (State::InStringEscape, c) if is_mnemonic_escape(c) => {
                    state_change!(current_state => InString);
                }
                (State::InStringEscape, 'x') => {
                    state_change!(current_state => InStringHexEscape);
                }
                (State::InStringHexEscape, d) if d.is_ascii_hexdigit() => {
                    // R7Rs says `<hex digit>+`
                    state_change!(current_state => InStringHexEscapeDigits);
                }
                (State::InStringHexEscapeDigits, d) if d.is_ascii_hexdigit() => {}
                (State::InStringHexEscapeDigits, ';') => {
                    // TODO: validate hex string
                    state_change!(current_state => InString);
                }
                (
                    State::InStringEscape
                    | State::InStringHexEscape
                    | State::InStringHexEscapeDigits,
                    _,
                ) => {
                    // TODO: Fix the span, it starts with the string start, not the escape start.
                    return_error!(current_state, char_index, invalid_escape_string);
                }
                // +inf.0 -inf.0 +nan.0 -nan.0
                // --------------------------------------------------------------------------------
                // Start of special forms
                (State::Nothing | State::InWhitespace, '#') => {
                    state_change_at!(char_index, current_state => InSpecial);
                }
                // --------------------------------------------------------------------------------
                // Boolean values
                (State::InSpecial, 't') => {
                    return_token_and_add_char!(current_state, char_index, Boolean => Nothing);
                }
                (State::InSpecial, 'f') => {
                    return_token_and_add_char!(current_state, char_index, Boolean => Nothing);
                }
                // --------------------------------------------------------------------------------
                // Vector values
                (State::InSpecial, '(') => {
                    return_token_and_add_char!(current_state, char_index, OpenVector => Nothing);
                }
                (State::InSpecial, 'u') => {
                    state_change!(current_state => State::InOpenByteVector('u'));
                }
                (State::InOpenByteVector('u'), '8') => {
                    state_change!(current_state => State::InOpenByteVector('8'));
                }
                (State::InOpenByteVector('8'), '(') => {
                    return_token_and_add_char!(current_state, char_index, OpenByteVector => Nothing);
                }
                (State::InOpenByteVector(_), _) => {
                    return_error!(current_state, char_index, invalid_byte_vector_prefix);
                }
                // --------------------------------------------------------------------------------
                // Numeric values
                (State::Nothing | State::InWhitespace, c) if c.is_ascii_digit() => {
                    state_change_at!(char_index, current_state => State::InNumeric);
                }
                (State::InNumeric, c) if c.is_ascii_digit() => {}
                (State::InNumeric, c) if is_identifier_subsequent(c) => {
                    state_change!(current_state => InIdentifier);
                }
                (State::InNumeric, _) => {
                    self.push_back_char(char_index);
                    return_token!(current_state, char_index, Numeric => Nothing);
                }
                (State::InSpecial, 'e' | 'i') => {
                    return_token_and_add_char!(current_state, char_index, NumericExactnessPrefix => Nothing);
                }
                (State::InSpecial, 'b' | 'o' | 'd' | 'x') => {
                    return_token_and_add_char!(current_state, char_index, NumericRadixPrefix => Nothing);
                }
                // --------------------------------------------------------------------------------
                // Character values
                (State::InSpecial, '\\') => {
                    state_change!(current_state => InCharacter);
                }
                (State::InCharacter, 'x') => {
                    state_change!(current_state => InCharacterX);
                }
                (State::InCharacter, c) if c.is_ascii_alphabetic() || c == '-' => {
                    state_change!(current_state => State::InCharacterName);
                }
                (State::InCharacter, _) => {
                    return_token_and_add_char!(current_state, char_index, Character => Nothing);
                }
                (State::InCharacterName, c) if c.is_ascii_alphabetic() || c == '-' => {}
                (State::InCharacterName, _) => {
                    self.push_back_char(char_index);
                    return_token!(current_state, char_index, Character => Nothing);
                }
                (State::InCharacterX, c) if c.is_ascii_hexdigit() => {
                    state_change!(current_state => InCharacterXNum);
                }
                (State::InCharacterX, _) => {
                    self.push_back_char(char_index);
                    return_token!(current_state, char_index, Character => Nothing);
                }
                (State::InCharacterXNum, c) if c.is_ascii_hexdigit() => {}
                (State::InCharacterXNum, ';') => {
                    return_token_and_add_char!(current_state, char_index, Character => Nothing);
                }
                // --------------------------------------------------------------------------------
                // Directives
                (State::InSpecial, '!') => {
                    state_change!(current_state => InDirective);
                }
                (State::InDirective, c) if is_directive(c) => {}
                (State::InDirective, _) => {
                    return_token_and_add_char!(current_state, char_index, Directive => Nothing);
                }
                // --------------------------------------------------------------------------------
                // Datum references
                (State::InSpecial, c) if c.is_ascii_digit() => {
                    state_change!(current_state => InDatumRefNum);
                }
                (State::InDatumRefNum, c) if c.is_ascii_digit() => {
                    state_change!(current_state => InDatumRef);
                }
                (State::InDatumRef, c) if c.is_ascii_digit() => {}
                (State::InDatumRef, '=') => {
                    return_token_and_add_char!(current_state, char_index, DatumAssign => Nothing);
                }
                (State::InDatumRef, '#') => {
                    return_token_and_add_char!(current_state, char_index, DatumRef => Nothing);
                }
                (State::InDatumRef, _) => {
                    return_error!(current_state, char_index, invalid_datum_label);
                }
                // --------------------------------------------------------------------------------
                // Comment Forms
                (State::Nothing | State::InWhitespace, ';') => {
                    state_change_at!(char_index, current_state => InLineComment);
                }
                (State::InLineComment, c) if c != '\n' => {}
                (State::InLineComment, '\n') => {
                    self.push_back_char(char_index);
                    return_token_and_add_char!(current_state, char_index, LineComment => Nothing);
                }
                (State::InSpecial, '|') => {
                    state_change!(current_state => InBlockComment);
                }
                (State::InSpecial, ';') => {
                    return_token_and_add_char!(current_state, char_index, DatumComment => Nothing);
                }
                // --------------------------------------------------------------------------------
                (State::InSpecial, c) => {
                    // push back?
                    eprintln!(
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

        last_char_index.set_byte_index(self.source_len());

        match current_state.state() {
            // ***** Safe Cases *****
            State::InDirective => {
                return_token!(current_state, last_char_index, Directive);
            }
            State::InIdentifier | State::InPeculiarIdentifier | State::InNumberOrIdentifier => {
                return_token!(current_state, last_char_index, Identifier);
            }
            State::InNumeric => {
                return_token!(current_state, last_char_index, Numeric);
            }
            State::InLineComment => {
                return_token!(current_state, last_char_index, LineComment);
            }
            State::InCharacterName | State::InCharacterX => {
                return_token!(current_state, last_char_index, Character);
            }
            // ***** Error Cases *****
            State::InVBarIdentifier => {
                panic!();
            }
            State::InSpecial => {
                return_error!(current_state, last_char_index, unclosed_special);
            }
            State::InString => {
                return_error!(current_state, last_char_index, unclosed_string);
            }
            State::InStringEscape | State::InStringHexEscape | State::InStringHexEscapeDigits => {
                return_error!(current_state, last_char_index, invalid_escape_string);
            }
            State::InVBarIdentifierEscape
            | State::InVBarIdentifierHexEscape
            | State::InVBarIdentifierHexEscapeDigits => {
                return_error!(current_state, last_char_index, invalid_escape_string);
            }
            State::InBlockComment => {
                return_error!(current_state, last_char_index, unclosed_block_comment);
            }
            State::InOpenByteVector(_) => {
                return_error!(current_state, last_char_index, invalid_byte_vector_prefix);
            }
            State::InCharacter | State::InCharacterXNum => {
                return_error!(current_state, last_char_index, invalid_char_input);
            }
            _ => None,
        }
    }
}

impl TokenIter<'_> {
    // fn push_state(&mut self, current_state: IteratorState, new_state: State) -> IteratorState {
    //     let new_state = current_state.clone_with_new_state(new_state);
    //     self.state_stack.push(current_state);
    //     new_state
    // }
    //
    // #[inline(always)]
    // fn pop_state(&mut self) -> IteratorState {
    //     self.state_stack.pop().unwrap()
    // }

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
