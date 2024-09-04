// https://en.wikipedia.org/wiki/Variable-length_quantity

#[derive(Debug, PartialEq, Eq)]
enum Error {
    IncompleteNumber,
}

// Connvertir une liste d'entier selon le modèle VLQ
// Parcourir l'entier à partir du poids fort pour trouver le 1er bit activé
// qui donnera le nombre de VLQ à traiter.
// On décale de 63 bits et test le bit de signe puis on continue en masquant les 7 bits suivants
// Parcourir à nouveau comme au dessus et converti la valeur en VLQ
// Traite le bit de signe puis récupère chaque 7 bits (& 0x7F) et initialise (| 0x80) le 8ème bit par défaut (multi)
// Une fois traité le nombre de 7 bits intéressant
// On reset le bit 8 (^ 0x80) du dernier (ou le seul) pour signaler que c'est terminé
fn to_bytes(values: &[u64]) -> Vec<u8> {
    let mut vlq = Vec::<u8>::new();
    for val in values {
        // nombre de longueur de 7 bits à traiter + le bit de signe
        let mut i = 9;
        if *val == 0 { // entier de zéro, pas de conversion
            vlq.push(0);
            continue;
        }
        loop {
            if (*val & (0x7F << (i * 7))) > 0 { break; }
            i -= 1;
        }
        for j in (0..=i).rev() {
            vlq.push((((*val >> (j * 7)) & 0x7F) | 0x80) as u8);
        }
        let l = vlq.last_mut().unwrap();
        *l ^= 0x80;
    }

    vlq
}

// Convertir une liste de nombre VLQ en entier
// Lister les bytes à traiter, le bit 8 servira à déterminer la séparation entre les nombres.
// Récupèrer les 7 bits (& 0x7F) intéressant et les placent dans r en décalant avant les
// valeurs précédentes (<< 7)
fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut numbers = Vec::<u32>::new();
    let mut r: u32 = 0;
	for b in bytes {
		r = (r << 7) | (*b as u32 & 0x7F);
        // si bit 8 non initialisé alors le nombre est complet
        if b & 0x80 == 0 {
            numbers.push(r);
            r = 0; // on reset pour le prochain nombre
            continue;
        }
	}
    if bytes.is_empty() || numbers.is_empty() {
        Err(Error::IncompleteNumber)
    } else {
        Ok(numbers)
    }
}

#[test]
fn zero() {
    let input = &[0];
    let output = to_bytes(input);
    let expected = vec![0x0];
    assert_eq!(output, expected);
}
#[test]
fn arbitrary_single_byte() {
    let input = &[64];
    let output = to_bytes(input);
    let expected = vec![0x40];
    assert_eq!(output, expected);
}
#[test]
fn largest_single_byte() {
    let input = &[127];
    let output = to_bytes(input);
    let expected = vec![0x7f];
    assert_eq!(output, expected);
}
#[test]
fn smallest_double_byte() {
    let input = &[128];
    let output = to_bytes(input);
    let expected = vec![0x81, 0x0];
    assert_eq!(output, expected);
}
#[test]
fn arbitrary_double_byte() {
    let input = &[8192];
    let output = to_bytes(input);
    let expected = vec![0xc0, 0x0];
    assert_eq!(output, expected);
}
#[test]
fn largest_double_byte() {
    let input = &[16383];
    let output = to_bytes(input);
    let expected = vec![0xff, 0x7f];
    assert_eq!(output, expected);
}
#[test]
fn smallest_triple_byte() {
    let input = &[16384];
    let output = to_bytes(input);
    let expected = vec![0x81, 0x80, 0x0];
    assert_eq!(output, expected);
}
#[test]
fn arbitrary_triple_byte() {
    let input = &[1048576];
    let output = to_bytes(input);
    let expected = vec![0xc0, 0x80, 0x0];
    assert_eq!(output, expected);
}
#[test]
fn largest_triple_byte() {
    let input = &[2097151];
    let output = to_bytes(input);
    let expected = vec![0xff, 0xff, 0x7f];
    assert_eq!(output, expected);
}
#[test]
fn smallest_quadruple_byte() {
    let input = &[2097152];
    let output = to_bytes(input);
    let expected = vec![0x81, 0x80, 0x80, 0x0];
    assert_eq!(output, expected);
}
#[test]
fn arbitrary_quadruple_byte() {
    let input = &[134217728];
    let output = to_bytes(input);
    let expected = vec![0xc0, 0x80, 0x80, 0x0];
    assert_eq!(output, expected);
}
#[test]
fn largest_quadruple_byte() {
    let input = &[268435455];
    let output = to_bytes(input);
    let expected = vec![0xff, 0xff, 0xff, 0x7f];
    assert_eq!(output, expected);
}
#[test]
fn smallest_quintuple_byte() {
    let input = &[268435456];
    let output = to_bytes(input);
    let expected = vec![0x81, 0x80, 0x80, 0x80, 0x0];
    assert_eq!(output, expected);
}
#[test]
fn arbitrary_quintuple_byte() {
    let input = &[4278190080];
    let output = to_bytes(input);
    let expected = vec![0x8f, 0xf8, 0x80, 0x80, 0x0];
    assert_eq!(output, expected);
}
#[test]
fn maximum_32_bit_integer_input() {
    let input = &[4294967295];
    let output = to_bytes(input);
    let expected = vec![0x8f, 0xff, 0xff, 0xff, 0x7f];
    assert_eq!(output, expected);
}
#[test]
fn two_single_byte_values() {
    let input = &[64, 127];
    let output = to_bytes(input);
    let expected = vec![0x40, 0x7f];
    assert_eq!(output, expected);
}
#[test]
fn two_multi_byte_values() {
    let input = &[16384, 1193046];
    let output = to_bytes(input);
    let expected = vec![0x81, 0x80, 0x0, 0xc8, 0xe8, 0x56];
    assert_eq!(output, expected);
}
#[test]
fn many_multi_byte_values() {
    let input = &[8192, 1193046, 268435455, 0, 16383, 16384];
    let output = to_bytes(input);
    let expected = vec![
        0xc0, 0x0, 0xc8, 0xe8, 0x56, 0xff, 0xff, 0xff, 0x7f, 0x0, 0xff, 0x7f, 0x81, 0x80, 0x0,
    ];
    assert_eq!(output, expected);
}
#[test]
fn one_byte() {
    let input = &[0x7f];
    let output = from_bytes(input);
    let expected = Ok(vec![127]);
    assert_eq!(output, expected);
}
#[test]
fn two_bytes() {
    let input = &[0xc0, 0x0];
    let output = from_bytes(input);
    let expected = Ok(vec![8192]);
    assert_eq!(output, expected);
}
#[test]
fn three_bytes() {
    let input = &[0xff, 0xff, 0x7f];
    let output = from_bytes(input);
    let expected = Ok(vec![2097151]);
    assert_eq!(output, expected);
}
#[test]
fn four_bytes() {
    let input = &[0x81, 0x80, 0x80, 0x0];
    let output = from_bytes(input);
    let expected = Ok(vec![2097152]);
    assert_eq!(output, expected);
}
#[test]
fn maximum_32_bit_integer() {
    let input = &[0x8f, 0xff, 0xff, 0xff, 0x7f];
    let output = from_bytes(input);
    let expected = Ok(vec![4294967295]);
    assert_eq!(output, expected);
}
#[test]
fn incomplete_sequence_causes_error() {
    let input = &[0xff];
    let output = from_bytes(input);
    let expected = Err(Error::IncompleteNumber);
    assert_eq!(output, expected);
}
#[test]
fn incomplete_sequence_causes_error_even_if_value_is_zero() {
    let input = &[0x80];
    let output = from_bytes(input);
    let expected = Err(Error::IncompleteNumber);
    assert_eq!(output, expected);
}
#[test]
fn multiple_values() {
    let input = &[
        0xc0, 0x0, 0xc8, 0xe8, 0x56, 0xff, 0xff, 0xff, 0x7f, 0x0, 0xff, 0x7f, 0x81, 0x80, 0x0,
    ];
    let output = from_bytes(input);
    let expected = Ok(vec![8192, 1193046, 268435455, 0, 16383, 16384]);
    assert_eq!(output, expected);
}