/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::error::{
    incomplete_block_comment, incomplete_identifier, incomplete_special, incomplete_string,
    invalid_byte_vector_prefix, invalid_char_input, invalid_datum_label, invalid_directive_input,
    Error,
};
use crate::input::indices::CharIndex;
use crate::input::iter::CharIndices;
use crate::lexer::internals::{IteratorState, State};
use crate::lexer::token::{Span, Token, TokenKind};
use crate::syntax::{
    IDENTIFIER_WRAPPER, PAIR_END, PAIR_START, QUASI_QUOTE_ABBREV, QUOTE_ABBREV, STRING_QUOTE,
    UNQUOTE_ABBREV, UNQUOTE_SPLICING_ABBREV,
};
use crate::{SourceId, Sourced};
use tracing::{error, trace, trace_span};
use unicode_categories::UnicodeCategories;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct TokenIter<'a> {
    source: CharIndices<'a>,
    state_stack: Vec<IteratorState>,
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
        trace!(
            "state change {:?} => {:?}",
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
        trace!("return token {token:?}");
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
        trace!("return token {token:?}");
        return token;
    };
}

macro_rules! return_error {
    ($current_state:expr, $char_index:expr, $error_fn:ident) => {
        state_change!($current_state => Nothing);
        let err = Some($error_fn(Span::new(
            $current_state.token_starts_at().character(),
            $char_index.index().character(),
        )));
        error!("return error {err:?}");
        return err;
    };
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<'a> From<CharIndices<'a>> for TokenIter<'a> {
    fn from(source: CharIndices<'a>) -> Self {
        Self {
            state_stack: Vec::default(),
            source,
        }
    }
}

impl Sourced for TokenIter<'_> {
    #[inline(always)]
    fn source_id(&self) -> &SourceId {
        self.source.source_id()
    }

    #[inline(always)]
    fn source_str(&self) -> &str {
        self.source.source_str()
    }
}

#[inline(always)]
fn is_radix_char(c: char, radix: u32) -> bool {
    match (radix, c) {
        (2, c) if c.is_ascii_digit() && c < '2' => true,
        (8, c) if c.is_ascii_digit() && c < '8' => true,
        (10, c) if c.is_ascii_digit() => true,
        (16, c) if c.is_ascii_hexdigit() => true,
        _ => false,
    }
}

impl Iterator for TokenIter<'_> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let span = trace_span!("next-token", ?self.state_stack);
        let _scope = span.enter();

        let mut current_state = if let Some(state) = self.state_stack.pop() {
            state
        } else {
            IteratorState::default()
        };

        let mut last_char_index = CharIndex::new(0, 0, '\u{00}');
        let mut number_radix: u32 = 10;

        while let Some(char_index) = self.next_char() {
            trace!(?current_state, ?char_index, "match");

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
                (State::Nothing | State::InWhitespace, PAIR_START) => {
                    current_state.set_token_start(&char_index);
                    return_token_and_add_char!(current_state, char_index, OpenParenthesis => Nothing);
                }
                (State::Nothing | State::InWhitespace, PAIR_END) => {
                    current_state.set_token_start(&char_index);
                    return_token_and_add_char!(current_state, char_index, CloseParenthesis => Nothing);
                }
                (State::Nothing | State::InWhitespace, QUOTE_ABBREV) => {
                    current_state.set_token_start(&char_index);
                    return_token_and_add_char!(current_state, char_index, Quote => Nothing);
                }
                (State::Nothing | State::InWhitespace, QUASI_QUOTE_ABBREV) => {
                    current_state.set_token_start(&char_index);
                    return_token_and_add_char!(current_state, char_index, QuasiQuote => Nothing);
                }
                (State::Nothing | State::InWhitespace, UNQUOTE_ABBREV) => {
                    current_state.set_token_start(&char_index);
                    if let Some(next_char) = self.peek_next_char() {
                        if next_char == &UNQUOTE_SPLICING_ABBREV {
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
                // Start of special forms
                (State::Nothing | State::InWhitespace | State::InNumber, '#') => {
                    state_change_at!(char_index, current_state => InSpecial);
                }
                // --------------------------------------------------------------------------------
                // These are ambiguous
                (State::Nothing | State::InWhitespace, '+' | '-') => {
                    state_change_at!(char_index, current_state => InNumberOrIdentifier);
                }
                (State::Nothing | State::InWhitespace, '.') => {
                    state_change_at!(char_index, current_state => InDotNumberOrIdentifier);
                }
                (State::InNumberOrIdentifier, 'i') => {
                    state_change!(current_state => State::InMaybeInf(0));
                }
                (State::InNumberOrIdentifier, 'n') => {
                    state_change!(current_state => State::InMaybeNan(0));
                }
                (State::InNumberOrIdentifier, c) if c.is_ascii_digit() => {
                    state_change!(current_state => InNumber);
                }
                (State::InNumberOrIdentifier, '.') => {
                    state_change!(current_state => InDotNumberOrIdentifier);
                }
                (State::InNumberOrIdentifier, c) if is_sign_subsequent(c) => {
                    state_change!(current_state => InPeculiarIdentifier);
                }
                (State::InDotNumberOrIdentifier, c) if c.is_ascii_digit() => {
                    state_change!(current_state => InNumber);
                }
                (State::InDotNumberOrIdentifier, c) if is_dot_subsequent(c) => {
                    state_change!(current_state => InPeculiarIdentifier);
                }
                (State::InPeculiarIdentifier, c) if is_identifier_subsequent(c) => {}
                (State::InPeculiarIdentifier, _) => {
                    self.push_back_char(char_index);
                    return_token!(current_state, char_index, Identifier => Nothing);
                }
                (State::InNumberOrIdentifier, _) => {
                    self.push_back_char(char_index);
                    return_token!(current_state, char_index, Identifier => Nothing);
                }
                (State::InDotNumberOrIdentifier, _) => {
                    self.push_back_char(char_index);
                    return_token!(current_state, char_index, Dot => Nothing);
                }
                // --------------------------------------------------------------------------------
                // Identifier values
                (State::Nothing | State::InWhitespace, IDENTIFIER_WRAPPER) => {
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
                (State::InVBarIdentifier, IDENTIFIER_WRAPPER) => {
                    return_token_and_add_char!(current_state, char_index, Identifier => Nothing);
                }
                (State::InVBarIdentifier, '\\') => {
                    state_change!(current_state => InVBarIdentifierEscape);
                }
                (State::InVBarIdentifier, _) => {}
                (State::InVBarIdentifierEscape, c) if is_mnemonic_escape(c) || c == 'x' => {
                    state_change!(current_state => InVBarIdentifier);
                }
                // --------------------------------------------------------------------------------
                // String values
                (State::Nothing | State::InWhitespace, STRING_QUOTE) => {
                    state_change_at!(char_index, current_state => InString);
                }
                (State::InString, '\\') => {
                    state_change!(current_state => InStringEscape);
                }
                (State::InString, STRING_QUOTE) => {
                    return_token_and_add_char!(current_state, char_index, String => Nothing);
                }
                (State::InString, _) => {}
                (State::InStringEscape, _) => {
                    state_change!(current_state => InString);
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
                // Numeric values
                (State::InMaybeInf(0), 'n') => {
                    state_change!(current_state => State::InMaybeInf(1));
                }
                (State::InMaybeInf(1), 'f') => {
                    state_change!(current_state => State::InMaybeInf(2));
                }
                (State::InMaybeInf(2), '.') => {
                    state_change!(current_state => State::InMaybeInf(3));
                }
                (State::InMaybeInf(3), '0') => {
                    return_token_and_add_char!(current_state, char_index, Number => Nothing);
                }
                (State::InMaybeInf(_), _) => {
                    state_change!(current_state => State::InIdentifier);
                }
                (State::InMaybeNan(0), 'a') => {
                    state_change!(current_state => State::InMaybeNan(1));
                }
                (State::InMaybeNan(1), 'n') => {
                    state_change!(current_state => State::InMaybeNan(2));
                }
                (State::InMaybeNan(2), '.') => {
                    state_change!(current_state => State::InMaybeNan(3));
                }
                (State::InMaybeNan(3), '0') => {
                    return_token_and_add_char!(current_state, char_index, Number => Nothing);
                }
                (State::InMaybeNan(_), _) => {
                    state_change!(current_state => State::InIdentifier);
                }
                (State::Nothing | State::InWhitespace, c) if is_radix_char(c, number_radix) => {
                    state_change_at!(char_index, current_state => InNumber);
                }
                (State::InNumber, 'e') => {}
                (State::InNumber, 'i') => {
                    return_token_and_add_char!(current_state, char_index, Number => Nothing);
                }
                (State::InNumber, c) if is_radix_char(c, number_radix) => {}
                (State::InNumber, c) if is_identifier_subsequent(c) => {
                    state_change!(current_state => InIdentifier);
                }
                (State::InNumber, _) => {
                    self.push_back_char(char_index);
                    return_token!(current_state, char_index, Number => Nothing);
                }
                (State::InSpecial | State::InNumberPrefix, 'e' | 'i') => {
                    state_change!(current_state => InNumberPrefix);
                }
                (State::InSpecial | State::InNumberPrefix, 'b') => {
                    number_radix = 2;
                    state_change!(current_state => InNumberPrefix);
                }
                (State::InSpecial | State::InNumberPrefix, 'o') => {
                    number_radix = 8;
                    state_change!(current_state => InNumberPrefix);
                }
                (State::InSpecial | State::InNumberPrefix, 'd') => {
                    number_radix = 10;
                    state_change!(current_state => InNumberPrefix);
                }
                (State::InSpecial | State::InNumberPrefix, 'x') => {
                    number_radix = 16;
                    state_change!(current_state => InNumberPrefix);
                }
                (State::InNumberPrefix, c) if is_radix_char(c, number_radix) => {
                    state_change!(current_state => InNumber);
                }
                (State::InNumberPrefix, '+' | '-' | '.') => {
                    state_change!(current_state => InNumber);
                }
                (State::InNumberPrefix, '#') => {}
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
                // Directives
                (State::InSpecial, '!') => {
                    state_change!(current_state => InDirective);
                }
                (State::InDirective, c) if is_directive(c) => {
                    state_change!(current_state => InDirectiveText);
                }
                (State::InDirective, _) => {
                    return_error!(current_state, char_index, invalid_directive_input);
                }
                (State::InDirectiveText, c) if is_directive(c) => {}
                (State::InDirectiveText, _) => {
                    self.push_back_char(char_index);
                    return_token!(current_state, char_index, Directive => Nothing);
                }
                // --------------------------------------------------------------------------------
                // Datum references
                (State::InSpecial, c) if c.is_ascii_digit() => {
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
                (State::InBlockComment, '|') => {
                    state_change!(current_state => InBlockCommentBar);
                }
                (State::InBlockCommentBar, '#') => {
                    return_token_and_add_char!(current_state, char_index, BlockComment => Nothing);
                }
                (State::InBlockCommentBar, _) => {
                    state_change!(current_state => InBlockComment);
                }
                (State::InBlockComment, _) => {}
                (State::InSpecial, ';') => {
                    return_token_and_add_char!(current_state, char_index, DatumComment => Nothing);
                }
                // --------------------------------------------------------------------------------
                (State::InSpecial, c) => {
                    // push back?
                    error!("Found char {c:?} at {char_index:?}, which doesn't belong in a special");
                    return_error!(current_state, char_index, incomplete_special);
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
            State::InDirectiveText => {
                return_token!(current_state, last_char_index, Directive);
            }
            State::InIdentifier | State::InPeculiarIdentifier | State::InNumberOrIdentifier => {
                return_token!(current_state, last_char_index, Identifier);
            }
            State::InNumber => {
                return_token!(current_state, last_char_index, Number);
            }
            State::InLineComment => {
                return_token!(current_state, last_char_index, LineComment);
            }
            State::InCharacterName | State::InCharacterX => {
                return_token!(current_state, last_char_index, Character);
            }
            // ***** Error Cases *****
            State::InVBarIdentifier => {
                return_error!(current_state, last_char_index, incomplete_identifier);
            }
            State::InSpecial => {
                return_error!(current_state, last_char_index, incomplete_special);
            }
            State::InDirective => {
                return_error!(current_state, last_char_index, invalid_directive_input);
            }
            State::InString => {
                return_error!(current_state, last_char_index, incomplete_string);
            }
            State::InBlockComment => {
                return_error!(current_state, last_char_index, incomplete_block_comment);
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
    #[inline(always)]
    fn next_char(&mut self) -> Option<CharIndex> {
        self.source.next()
    }

    #[inline(always)]
    fn peek_next_char(&mut self) -> Option<&CharIndex> {
        self.source.peek()
    }

    #[inline(always)]
    fn push_back_char(&mut self, index: CharIndex) {
        self.source.push_back(index)
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
