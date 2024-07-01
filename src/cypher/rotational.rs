
use crate::cypher::PLAIN;

pub fn rotate(input: &str, key: u8) -> String {
    let (str1, str2) = PLAIN.split_at(key as usize % PLAIN.len());
    let rot_alpha = Vec::from_iter((str2.to_owned() + str1).chars());
    let crypted = input.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let is_upper = c.is_ascii_uppercase();
            let cara = if is_upper { c.to_ascii_lowercase() } else { c };
            if let Some(pos) = PLAIN.chars().position(|c| c == cara) {
                if is_upper {
                    return rot_alpha[pos].to_ascii_uppercase(); }
                    else { return rot_alpha[pos]; }
            }
        }
        c
    }).collect::<String>();

    crypted
}

#[test]
fn rotate_a_by_0_same_output_as_input() {
    let text = "a";
    let shift_key = 0;
    let output = rotate(text, shift_key);
    let expected = "a";
    assert_eq!(output, expected);
}
#[test]
fn rotate_a_by_1() {
    let text = "a";
    let shift_key = 1;
    let output = rotate(text, shift_key);
    let expected = "b";
    assert_eq!(output, expected);
}
#[test]
fn rotate_a_by_26_same_output_as_input() {
    let text = "a";
    let shift_key = 26;
    let output = rotate(text, shift_key);
    let expected = "a";
    assert_eq!(output, expected);
}
#[test]
fn rotate_m_by_13() {
    let text = "m";
    let shift_key = 13;
    let output = rotate(text, shift_key);
    let expected = "z";
    assert_eq!(output, expected);
}
#[test]
fn rotate_n_by_13_with_wrap_around_alphabet() {
    let text = "n";
    let shift_key = 13;
    let output = rotate(text, shift_key);
    let expected = "a";
    assert_eq!(output, expected);
}
#[test]
fn rotate_capital_letters() {
    let text = "OMG";
    let shift_key = 5;
    let output = rotate(text, shift_key);
    let expected = "TRL";
    assert_eq!(output, expected);
}
#[test]
fn rotate_spaces() {
    let text = "O M G";
    let shift_key = 5;
    let output = rotate(text, shift_key);
    let expected = "T R L";
    assert_eq!(output, expected);
}
#[test]
fn rotate_numbers() {
    let text = "Testing 1 2 3 testing";
    let shift_key = 4;
    let output = rotate(text, shift_key);
    let expected = "Xiwxmrk 1 2 3 xiwxmrk";
    assert_eq!(output, expected);
}
#[test]
fn rotate_punctuation() {
    let text = "Let's eat, Grandma!";
    let shift_key = 21;
    let output = rotate(text, shift_key);
    let expected = "Gzo'n zvo, Bmviyhv!";
    assert_eq!(output, expected);
}
#[test]
fn rotate_all_letters() {
    let text = "The quick brown fox jumps over the lazy dog.";
    let shift_key = 13;
    let output = rotate(text, shift_key);
    let expected = "Gur dhvpx oebja sbk whzcf bire gur ynml qbt.";
    assert_eq!(output, expected);
}