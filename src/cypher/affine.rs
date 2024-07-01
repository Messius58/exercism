use crate::cypher::PLAIN;

#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

// Encodes the plaintext using the affine cipher with key (`a`, `b`).
// E(x) = (ax + b) mod m

// i is the letter's index from 0 to the length of the alphabet - 1
// m is the length of the alphabet. For the Roman alphabet m is 26.
// a and b are integers which make the encryption key
// a and m are coprime or error
pub fn encode(plaintext: &str, a: u32, b: u32) -> Result<String, AffineCipherError> {
    let _ = modinv(a as i32, PLAIN.len() as i32)?;
    let x = plaintext.chars()
                    .filter(|c| c.is_ascii_alphanumeric())
                    .map(|c| c.to_ascii_lowercase())
                    .map(|pt|
                        if pt.is_ascii_digit() { pt }
                        else {
                            let pos = (a as usize * PLAIN.chars().position(|bs| bs == pt).unwrap() + b as usize) % PLAIN.len();
                            PLAIN.chars().nth(pos).unwrap()
                        }
                    )
                    .collect::<Vec<char>>()
                    .chunks(5).map(|chunk| chunk.iter().collect::<String>())
                    .collect::<Vec<String>>()
                    .join(" ");

    Ok(x)
}

// Decodes the ciphertext using the affine cipher with key (`a`, `b`).

// D(y) = a^-1(y - b) mod m
// 1 = a^-1 mod m

// y = E(x) is the numeric value of an encrypted letter
// it is important to note that a^-1 is the modular multiplicative inverse (MMI) of a mod m
// the modular multiplicative inverse only exists if a and m are coprime.
// if index calculated from the letter encrypted is negative then add m
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let a_prime = modinv(a, PLAIN.len() as i32)?;
    let y = ciphertext.chars()
                    .filter(|c| c.is_ascii_alphanumeric())
                    .map(|c| c.to_ascii_lowercase())
                    .map(|ct|{
                        if ct.is_ascii_digit() { ct }
                        else {
                            let mut n = (a_prime * (PLAIN.chars().position(|bs| bs == ct).unwrap() as i32 - b)) % PLAIN.len() as i32;
                            n = if n < 0 { PLAIN.len() as i32 + n } else { n };
                            PLAIN.chars().nth(n as usize).unwrap()
                        }
                    })
                    .collect::<String>();

    Ok(y)
}

// Calculate the MMI of a and m
// The MMI of a is x such that the remainder after dividing ax by m is 1:
// ax mod m = 1
// return x such that (x * a) % b == 1
fn modinv(a: i32, m: i32) -> Result<i32, AffineCipherError> {
    let (g, x, _) = xgcd(a, m);
    if g != 1 {
        Err(AffineCipherError::NotCoprime(a))
    } else {
        Ok(x % m)
    }
}

// find the pgcd of a and b with extended euclidian
// return (g, x, y) such that a*x + b*y = g = gcd(a, b)
// g must to equal to 1
fn xgcd(a: i32, b: i32) -> (i32, i32, i32) {
    let mut x = (0, 1);
    let mut y = (1, 0);
    let mut q = 0;
    let mut b = b;
    let mut a = a;

    while a != 0 {
        (q, a, b) = (b.div_euclid(a), b.rem_euclid(a), a);
        y = (y.1, y.0 - q * y.1);
        x = (x.1, x.0 - q * x.1);
    }

    (b, x.0, y.0)
}

#[test]
fn encode_yes() {
    let phrase = "yes";
    let (a, b) = (5, 7);
    let output = encode(phrase, a, b);
    let expected = Ok("xbt".into());
    assert_eq!(output, expected);
}
#[test]
fn encode_no() {
    let phrase = "no";
    let (a, b) = (15, 18);
    let output = encode(phrase, a, b);
    let expected = Ok("fu".into());
    assert_eq!(output, expected);
}
#[test]
fn encode_omg() {
    let phrase = "OMG";
    let (a, b) = (21, 3);
    let output = encode(phrase, a, b);
    let expected = Ok("lvz".into());
    assert_eq!(output, expected);
}
#[test]
fn encode_o_m_g() {
    let phrase = "O M G";
    let (a, b) = (25, 47);
    let output = encode(phrase, a, b);
    let expected = Ok("hjp".into());
    assert_eq!(output, expected);
}
#[test]
fn encode_mindblowingly() {
    let phrase = "mindblowingly";
    let (a, b) = (11, 15);
    let output = encode(phrase, a, b);
    let expected = Ok("rzcwa gnxzc dgt".into());
    assert_eq!(output, expected);
}
#[test]
fn encode_numbers() {
    let phrase = "Testing,1 2 3, testing.";
    let (a, b) = (3, 4);
    let output = encode(phrase, a, b);
    let expected = Ok("jqgjc rw123 jqgjc rw".into());
    assert_eq!(output, expected);
}
#[test]
fn encode_deep_thought() {
    let phrase = "Truth is fiction.";
    let (a, b) = (5, 17);
    let output = encode(phrase, a, b);
    let expected = Ok("iynia fdqfb ifje".into());
    assert_eq!(output, expected);
}
#[test]
fn encode_all_the_letters() {
    let phrase = "The quick brown fox jumps over the lazy dog.";
    let (a, b) = (17, 33);
    let output = encode(phrase, a, b);
    let expected = Ok("swxtj npvyk lruol iejdc blaxk swxmh qzglf".into());
    assert_eq!(output, expected);
}
#[test]
fn encode_with_a_not_coprime_to_m() {
    let phrase = "This is a test.";
    let (a, b) = (6, 17);
    let output = encode(phrase, a, b);
    let expected = Err(AffineCipherError::NotCoprime(6));
    assert_eq!(output, expected);
}
#[test]
fn decode_exercism() {
    let phrase = "tytgn fjr";
    let (a, b) = (3, 7);
    let output = decode(phrase, a, b);
    let expected = Ok("exercism".into());
    assert_eq!(output, expected);
}
#[test]
fn decode_a_sentence() {
    let phrase = "qdwju nqcro muwhn odqun oppmd aunwd o";
    let (a, b) = (19, 16);
    let output = decode(phrase, a, b);
    let expected = Ok("anobstacleisoftenasteppingstone".into());
    assert_eq!(output, expected);
}
#[test]
fn decode_numbers() {
    let phrase = "odpoz ub123 odpoz ub";
    let (a, b) = (25, 7);
    let output = decode(phrase, a, b);
    let expected = Ok("testing123testing".into());
    assert_eq!(output, expected);
}
#[test]
fn decode_all_the_letters() {
    let phrase = "swxtj npvyk lruol iejdc blaxk swxmh qzglf";
    let (a, b) = (17, 33);
    let output = decode(phrase, a, b);
    let expected = Ok("thequickbrownfoxjumpsoverthelazydog".into());
    assert_eq!(output, expected);
}
#[test]
fn decode_with_no_spaces_in_input() {
    let phrase = "swxtjnpvyklruoliejdcblaxkswxmhqzglf";
    let (a, b) = (17, 33);
    let output = decode(phrase, a, b);
    let expected = Ok("thequickbrownfoxjumpsoverthelazydog".into());
    assert_eq!(output, expected);
}
#[test]
fn decode_with_too_many_spaces() {
    let phrase = "vszzm    cly   yd cg    qdp";
    let (a, b) = (15, 16);
    let output = decode(phrase, a, b);
    let expected = Ok("jollygreengiant".into());
    assert_eq!(output, expected);
}
#[test]
fn decode_with_a_not_coprime_to_m() {
    let phrase = "Test";
    let (a, b) = (13, 5);
    let output = decode(phrase, a, b);
    let expected = Err(AffineCipherError::NotCoprime(13));
    assert_eq!(output, expected);
}