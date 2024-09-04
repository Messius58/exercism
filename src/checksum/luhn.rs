// Check a Luhn checksum.

use std::fmt::Display;

struct Luhn {
    input: String
}

trait LuhnCheck {
    fn valid_luhn(&self) -> bool;
}

impl<T> LuhnCheck for T
where T: ToString + Display {
    fn valid_luhn(&self) -> bool {
        let l  = Luhn::from(&self);
        l.is_valid()
    }
}

impl Luhn {
    // jolie solution avec 3 map trouvé dans la communauté exercism
    /*
    - Exclus le caractère blanc autorisé mais non digit.
    - Si non digit alors None, géré par try_fold, et passe dans map_or => false
    - Chaque map est une étape du calcul du luhn, réalisé dans un accumulateur de type tuple (fold)
    */
    fn is_valid(&self) -> bool {
        self.input.chars()
            .rev()
            .filter(|c| !c.is_whitespace())
            .try_fold((0, 0), |(sum, count), val| {
                val.to_digit(10)
                    .map(|num| if count % 2 == 1 { num * 2 } else { num })
                    .map(|num| if num > 9 { num - 9 } else { num })
                    .map(|num| (num + sum, count + 1))
            }).map_or(false, |(sum, count)| sum % 10 == 0 && count > 1)
    }
}

impl<T> From<T> for Luhn
where T: ToString {
    fn from(input: T) -> Self {
        Luhn {
            input: input.to_string()
        }
    }
}

fn process_valid_case(number: &str, is_luhn_expected: bool) {
    let l = Luhn::from(number);
    assert_eq!(l.is_valid(), is_luhn_expected);
}

#[test]
fn single_digit_strings_can_not_be_valid() {
    process_valid_case("1", false);
}
#[test]
fn a_single_zero_is_invalid() {
    process_valid_case("0", false);
}
#[test]
fn a_simple_valid_sin_that_remains_valid_if_reversed() {
    process_valid_case("059", true);
}
#[test]
fn a_simple_valid_sin_that_becomes_invalid_if_reversed() {
    process_valid_case("59", true);
}
#[test]
fn a_valid_canadian_sin() {
    process_valid_case("055 444 285", true);
}
#[test]
fn invalid_canadian_sin() {
    process_valid_case("055 444 286", false);
}
#[test]
fn invalid_credit_card() {
    process_valid_case("8273 1232 7352 0569", false);
}
#[test]
fn valid_number_with_an_even_number_of_digits() {
    process_valid_case("095 245 88", true);
}
#[test]
fn string_that_contain_non_digits_are_invalid() {
    process_valid_case("055a 444 285", false);
}
#[test]
fn valid_strings_with_punctuation_included_become_invalid() {
    process_valid_case("055-444-285", false);
}
#[test]
fn symbols_are_invalid() {
    process_valid_case("055£ 444$ 285", false);
}
#[test]
fn single_zero_with_space_is_invalid() {
    process_valid_case(" 0", false);
}
#[test]
fn more_than_a_single_zero_is_valid() {
    process_valid_case("0000 0", true);
}
#[test]
fn input_digit_9_is_correctly_converted_to_output_digit_9() {
    process_valid_case("091", true);
}
#[test]
/// using ASCII value for doubled non-digit isn't allowed
/// Convert non-digits to their ASCII values and then offset them by 48 sometimes accidentally declare an invalid string to be valid.
/// This test is designed to avoid that solution.
fn using_ascii_value_for_doubled_nondigit_isnt_allowed() {
    process_valid_case(":9", false);
}
#[test]
/// valid strings with a non-digit added at the end become invalid
fn valid_strings_with_a_nondigit_added_at_the_end_become_invalid() {
    process_valid_case("059a", false);
}
#[test]
/// valid strings with symbols included become invalid
fn valid_strings_with_symbols_included_become_invalid() {
    process_valid_case("055# 444$ 285", false);
}
#[test]
/// using ASCII value for non-doubled non-digit isn't allowed
/// Convert non-digits to their ASCII values and then offset them by 48 sometimes accidentally declare an invalid string to be valid.
/// This test is designed to avoid that solution.
fn using_ascii_value_for_nondoubled_nondigit_isnt_allowed() {
    process_valid_case("055b 444 285", false);
}
#[test]
/// valid number with an odd number of spaces
fn valid_number_with_an_odd_number_of_spaces() {
    process_valid_case("234 567 891 234", true);
}
#[test]
/// non-numeric, non-space char in the middle with a sum that's divisible by 10 isn't allowed
fn invalid_char_in_middle_with_sum_divisible_by_10_isnt_allowed() {
    process_valid_case("59%59", false);
}
#[test]
/// unicode numeric characters are not allowed in a otherwise valid number
fn valid_strings_with_numeric_unicode_characters_become_invalid() {
    process_valid_case("1249①", false);
}

#[test]
fn you_can_validate_from_a_str() {
    let valid = Luhn::from("046 454 286");
    let invalid = Luhn::from("046 454 287");
    assert!(valid.is_valid());
    assert!(!invalid.is_valid());
}
#[test]
fn you_can_validate_from_a_string() {
    let valid = Luhn::from(String::from("046 454 286"));
    let invalid = Luhn::from(String::from("046 454 287"));
    assert!(valid.is_valid());
    assert!(!invalid.is_valid());
}
#[test]
fn you_can_validate_from_a_u8() {
    let valid = Luhn::from(240u8);
    let invalid = Luhn::from(241u8);
    assert!(valid.is_valid());
    assert!(!invalid.is_valid());
}
#[test]
fn you_can_validate_from_a_u16() {
    let valid = Luhn::from(64_436u16);
    let invalid = Luhn::from(64_437u16);
    assert!(valid.is_valid());
    assert!(!invalid.is_valid());
}
#[test]
fn you_can_validate_from_a_u32() {
    let valid = Luhn::from(46_454_286u32);
    let invalid = Luhn::from(46_454_287u32);
    assert!(valid.is_valid());
    assert!(!invalid.is_valid());
}
#[test]
fn you_can_validate_from_a_u64() {
    let valid = Luhn::from(8273_1232_7352_0562u64);
    let invalid = Luhn::from(8273_1232_7352_0569u64);
    assert!(valid.is_valid());
    assert!(!invalid.is_valid());
}
#[test]
fn you_can_validate_from_a_usize() {
    let valid = Luhn::from(8273_1232_7352_0562usize);
    let invalid = Luhn::from(8273_1232_7352_0569usize);
    assert!(valid.is_valid());
    assert!(!invalid.is_valid());
}
#[test]
fn single_digit_string_is_invalid() {
    assert!(!Luhn::from("1").is_valid());
}
#[test]
fn single_zero_string_is_invalid() {
    assert!(!Luhn::from("0").is_valid());
}
#[test]
fn valid_canadian_sin_is_valid() {
    assert!(Luhn::from("046 454 286").is_valid());
}
#[test]
fn invalid_canadian_sin_is_invalid() {
    assert!(!Luhn::from("046 454 287").is_valid());
}
#[test]
fn invalid_credit_card_is_invalid() {
    assert!(!Luhn::from("8273 1232 7352 0569").is_valid());
}
#[test]
fn strings_that_contain_non_digits_are_invalid() {
    assert!(!Luhn::from("046a 454 286").is_valid());
}
#[test]
fn input_digit_9_is_still_correctly_converted_to_output_digit_9() {
    assert!(Luhn::from("091").is_valid());
}

#[test]
fn trait_you_can_validate_from_a_str() {
    assert!("046 454 286".valid_luhn());
    assert!(!"046 454 287".valid_luhn());
}
#[test]
fn trait_you_can_validate_from_a_string() {
    assert!(String::from("046 454 286").valid_luhn());
    assert!(!String::from("046 454 287").valid_luhn());
}
#[test]
fn trait_you_can_validate_from_a_u8() {
    assert!(240u8.valid_luhn());
    assert!(!241u8.valid_luhn());
}
#[test]
fn trait_you_can_validate_from_a_u16() {
    let valid = 64_436u16;
    let invalid = 64_437u16;
    assert!(valid.valid_luhn());
    assert!(!invalid.valid_luhn());
}
#[test]
fn trait_you_can_validate_from_a_u32() {
    let valid = 46_454_286u32;
    let invalid = 46_454_287u32;
    assert!(valid.valid_luhn());
    assert!(!invalid.valid_luhn());
}
#[test]
fn trait_you_can_validate_from_a_u64() {
    let valid = 8273_1232_7352_0562u64;
    let invalid = 8273_1232_7352_0569u64;
    assert!(valid.valid_luhn());
    assert!(!invalid.valid_luhn());
}
#[test]
fn trait_you_can_validate_from_a_usize() {
    let valid = 8273_1232_7352_0562usize;
    let invalid = 8273_1232_7352_0569usize;
    assert!(valid.valid_luhn());
    assert!(!invalid.valid_luhn());
}
#[test]
fn trait_input_digit_9_is_still_correctly_converted_to_output_digit_9() {
    assert!("091".valid_luhn());
}