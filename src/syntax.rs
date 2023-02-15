// ------------------------------------------------------------------------------------------------
// Special forms
// ------------------------------------------------------------------------------------------------

pub(crate) const SPECIAL_PREFIX_CHAR: char = '#';

// ------------------------------------------------------------------------------------------------
// Identifiers
// ------------------------------------------------------------------------------------------------

pub(crate) const IDENTIFIER_WRAPPER: char = '|';

// ------------------------------------------------------------------------------------------------
// Boolean
// ------------------------------------------------------------------------------------------------

pub(crate) const BOOLEAN_VALUE_TRUE: &str = "#t";
pub(crate) const BOOLEAN_VALUE_FALSE: &str = "#f";

pub(crate) const BOOLEAN_VALUE_TRUE_UC: &str = "#T";
pub(crate) const BOOLEAN_VALUE_FALSE_UC: &str = "#F";

// ------------------------------------------------------------------------------------------------
// Characters
// ------------------------------------------------------------------------------------------------

pub(crate) const CHAR_PREFIX_STR: &str = "#\\";
pub(crate) const CHAR_HEX_ESCAPE_START: char = 'x';
pub(crate) const CHAR_HEX_ESCAPE_END: char = ';';

pub(crate) const CHAR_NAME_ALARM: &str = "alarm";
pub(crate) const CHAR_NAME_BACKSPACE: &str = "backspace";
pub(crate) const CHAR_NAME_DELETE: &str = "delete";
pub(crate) const CHAR_NAME_ESCAPE: &str = "escape";
pub(crate) const CHAR_NAME_NEWLINE: &str = "newline";
pub(crate) const CHAR_NAME_NULL: &str = "null";
pub(crate) const CHAR_NAME_RETURN: &str = "return";
pub(crate) const CHAR_NAME_SPACE: &str = "space";
pub(crate) const CHAR_NAME_TAB: &str = "tab";

pub(crate) const CHAR_ESCAPE_ALARM: char = 'a';
pub(crate) const CHAR_ESCAPE_BACKSPACE: char = 'b';
pub(crate) const CHAR_ESCAPE_NEWLINE: char = 'n';
pub(crate) const CHAR_ESCAPE_RETURN: char = 'r';
pub(crate) const CHAR_ESCAPE_TAB: char = 't';

pub(crate) const CHAR_VALUE_ALARM: char = '\u{07}';
pub(crate) const CHAR_VALUE_BACKSPACE: char = '\u{08}';
pub(crate) const CHAR_VALUE_DELETE: char = '\u{7f}';
pub(crate) const CHAR_VALUE_ESCAPE: char = '\u{1b}';
pub(crate) const CHAR_VALUE_NEWLINE: char = '\n';
pub(crate) const CHAR_VALUE_NULL: char = '\u{00}';
pub(crate) const CHAR_VALUE_RETURN: char = '\r';
pub(crate) const CHAR_VALUE_SPACE: char = ' ';
pub(crate) const CHAR_VALUE_TAB: char = '\t';
pub(crate) const CHAR_VALUE_QUOTE: char = '\"';
pub(crate) const CHAR_VALUE_BACKSLASH: char = '\\';
pub(crate) const CHAR_VALUE_VBAR: char = '|';

// ------------------------------------------------------------------------------------------------
// Strings
// ------------------------------------------------------------------------------------------------

pub(crate) const STRING_QUOTE: char = '"';

// ------------------------------------------------------------------------------------------------
// Numbers
// ------------------------------------------------------------------------------------------------

pub(crate) const NUMERIC_PREFIX_EXACT: &str = "#e";
pub(crate) const NUMERIC_PREFIX_INEXACT: &str = "#i";
pub(crate) const NUMERIC_PREFIX_BINARY: &str = "#b";
pub(crate) const NUMERIC_PREFIX_OCTAL: &str = "#o";
//pub(crate) const NUMERIC_PREFIX_DECIMAL: &str = "#d";
pub(crate) const NUMERIC_PREFIX_HEXADECIMAL: &str = "#x";

//pub(crate) const NUMERIC_POSITIVE: char = '+';
pub(crate) const NUMERIC_NEGATIVE: char = '-';
pub(crate) const NUMERIC_DECIMAL_POINT: char = '.';
pub(crate) const NUMERIC_EXPONENT_MARK: char = 'e';
pub(crate) const NUMERIC_LONG_EXPONENT_MARK: char = 'l';
pub(crate) const NUMERIC_ALT_EXPONENT_MARK: char = '^';
pub(crate) const NUMERIC_COMPLEX_MARK: char = 'i';
pub(crate) const NUMERIC_RATIONAL_SEPARATOR: char = '/';
pub(crate) const NUMERIC_POLAR_SEPARATOR: char = '@';

pub(crate) const POSITIVE_INFINITY: &str = "+inf.0";
pub(crate) const NEGATIVE_INFINITY: &str = "-inf.0";

pub(crate) const POSITIVE_NAN: &str = "+nan.0";
pub(crate) const NEGATIVE_NAN: &str = "-nan.0";

// ------------------------------------------------------------------------------------------------
// Pairs & Lists
// ------------------------------------------------------------------------------------------------

pub(crate) const PAIR_START: char = '(';
pub(crate) const PAIR_END: char = ')';
pub(crate) const PAIR_DOT: char = '.';

// ------------------------------------------------------------------------------------------------
// Vectors
// ------------------------------------------------------------------------------------------------

pub(crate) const VECTOR_START: &str = "#(";
pub(crate) const VECTOR_END: char = ')';

// ------------------------------------------------------------------------------------------------
// Byte Vectors
// ------------------------------------------------------------------------------------------------

pub(crate) const BYTE_VECTOR_START: &str = "#u8(";
pub(crate) const BYTE_VECTOR_END: char = ')';

// ------------------------------------------------------------------------------------------------
// QUOTE
// ------------------------------------------------------------------------------------------------

pub(crate) const QUOTE_ABBREV: char = '\'';
pub(crate) const QUASI_QUOTE_ABBREV: char = '`';
pub(crate) const UNQUOTE_ABBREV: char = ',';
pub(crate) const UNQUOTE_SPLICING_ABBREV: char = '@';

// ------------------------------------------------------------------------------------------------
// Directives
// ------------------------------------------------------------------------------------------------

pub(crate) const DIRECTIVE_PREFIX_STR: &str = "#!";
pub(crate) const DIRECTIVE_FOLD_CASE: &str = "fold-case";
pub(crate) const DIRECTIVE_NO_FOLD_CASE: &str = "no-fold-case";

// ------------------------------------------------------------------------------------------------
// Comments
// ------------------------------------------------------------------------------------------------

pub(crate) const COMMENT_LINE_START: char = ';';

pub(crate) const COMMENT_BLOCK_START: &str = "#|";
pub(crate) const COMMENT_BLOCK_END: &str = "|#";

pub(crate) const COMMENT_DATUM_START: &str = "#;";
