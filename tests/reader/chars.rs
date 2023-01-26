use ffsr::reader::datum::SChar;
use paste::paste;

// ------------------------------------------------------------------------------------------------
// Single-valued success cases
// ------------------------------------------------------------------------------------------------

success_case!(single_a, "#\\a" => Char, SChar::from('a'));

success_case!(single_emoji, "#\\😀" => Char, SChar::from('😀'));

success_case!(single_x, "#\\x" => Char, SChar::from('x'));

success_case!(space_name, "#\\space" => Char, SChar::from(' '));

success_case!(mnemonic_escape, "#\\tab" => Char, SChar::from('\t'));

success_case!(hex_escape, "#\\x00fb;" => Char, SChar::from('û'));

success_case!(unicode, "#\\û" => Char, SChar::from('û'));

// ------------------------------------------------------------------------------------------------
// Failure cases
// ------------------------------------------------------------------------------------------------
