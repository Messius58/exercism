
/** Autre solution qui utilise windows (peut être intéressant à connaître) pour gérer la longueur MAIS
 * en passant par 1 vecteur intermédiaire et 2 collect
 *
 * Sur un petit benchmark de 100_000 itérations, il y a 30% de mauvaise performance.
 * */

// fn series(digits: &str, len: usize) -> Vec<String> {
//     if len == 0 {
//         return vec!["".to_string(); digits.len() + 1];
//     }
//     digits
//         .chars()
//         .collect::<Vec<char>>()
//         .windows(len)
//         .map(|c| c.into_iter().collect::<String>())
//         .collect()
// }

fn series(digits: &str, len: usize) -> Vec<String> {
    let mut suites = Vec::<String>::new();
    if len > 0 && len <= digits.len() {
        (len..=digits.len()).fold(0, |acc, i| {
            suites.push(digits[acc..i].to_string());
            acc + 1
        });
    }
    suites
}

#[test]
fn slices_of_one_from_one() {
    let input = "1";
    let length = 1;
    let output = series(input, length);
    let expected = &["1"];
    assert_eq!(output, expected);
}
#[test]
fn slices_of_one_from_two() {
    let input = "12";
    let length = 1;
    let output = series(input, length);
    let expected = &["1", "2"];
    assert_eq!(output, expected);
}
#[test]
fn slices_of_two() {
    let input = "35";
    let length = 2;
    let output = series(input, length);
    let expected = &["35"];
    assert_eq!(output, expected);
}
#[test]
fn slices_of_two_overlap() {
    let input = "9142";
    let length = 2;
    let output = series(input, length);
    let expected = &["91", "14", "42"];
    assert_eq!(output, expected);
}
#[test]
fn slices_can_include_duplicates() {
    let input = "777777";
    let length = 3;
    let output = series(input, length);
    let expected = &["777", "777", "777", "777"];
    assert_eq!(output, expected);
}
#[test]
fn slices_of_a_long_series() {
    let input = "918493904243";
    let length = 5;
    let output = series(input, length);
    let expected = &[
        "91849", "18493", "84939", "49390", "93904", "39042", "90424", "04243",
    ];
    assert_eq!(output, expected);
}
#[test]
fn slice_length_is_too_large() {
    let input = "12345";
    let length = 6;
    let output = series(input, length);
    let expected: &[&str] = &[];
    assert_eq!(output, expected);
}
#[test]
fn slice_length_is_way_too_large() {
    let input = "12345";
    let length = 42;
    let output = series(input, length);
    let expected: &[&str] = &[];
    assert_eq!(output, expected);
}
#[test]
fn empty_series_is_invalid() {
    let input = "";
    let length = 1;
    let output = series(input, length);
    let expected: &[&str] = &[];
    assert_eq!(output, expected);
}