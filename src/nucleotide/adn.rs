use std::collections::HashMap;

const DNA_NUCLEIDE_BASE: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !DNA_NUCLEIDE_BASE.contains(&nucleotide) {
        return Err(nucleotide);
    }
    let tot = dna.chars().try_fold(0, |acc, c| {
        match (DNA_NUCLEIDE_BASE.contains(&c), c == nucleotide) {
            (false, _) => Err(c),
            (_, true) => Ok(acc + 1),
            (_, _) => Ok(acc),
        }
    })?;

    Ok(tot)
}

// Ordre ADN : A(denine), C(ytosine), G(uanine), T(hymine)
pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut nucleos = HashMap::<char, usize>::new();
    for nucleotide in DNA_NUCLEIDE_BASE {
        nucleos.insert(nucleotide, count(nucleotide, dna)?);
    }

    Ok(nucleos)
}

fn process_nucleotidecounts_case(s: &str, pairs: &[(char, usize)]) {
    // The reason for the awkward code in here is to ensure that the failure
    // message for assert_eq! is as informative as possible. A simpler
    // solution would simply check the length of the map, and then
    // check for the presence and value of each key in the given pairs vector.
    let mut m: HashMap<char, usize> = nucleotide_counts(s).unwrap();
    for &(k, v) in pairs.iter() {
        assert_eq!((k, m.remove(&k)), (k, Some(v)));
    }
    // may fail with a message that clearly shows all extra pairs in the map
    assert_eq!(m.iter().collect::<Vec<(&char, &usize)>>(), vec![]);
}
#[test]
fn count_returns_result() {
    assert!(count('A', "").is_ok());
}
#[test]
fn count_empty() {
    assert_eq!(count('A', ""), Ok(0));
}
#[test]
fn count_invalid_nucleotide() {
    assert_eq!(count('X', "A"), Err('X'));
}
#[test]
fn count_invalid_dna() {
    assert_eq!(count('A', "AX"), Err('X'));
}
#[test]
fn count_repetitive_cytosine() {
    assert_eq!(count('C', "CCCCC"), Ok(5));
}
#[test]
fn count_only_thymine() {
    assert_eq!(count('T', "GGGGGTAACCCGG"), Ok(1));
}
#[test]
fn counts_returns_result() {
    assert!(nucleotide_counts("ACGT").is_ok());
}
#[test]
fn empty_strand() {
    process_nucleotidecounts_case("", &[('A', 0), ('T', 0), ('C', 0), ('G', 0)]);
}
#[test]
/// can count one nucleotide in single-character input
fn can_count_one_nucleotide_in_singlecharacter_input() {
    process_nucleotidecounts_case("G", &[('A', 0), ('C', 0), ('G', 1), ('T', 0)]);
}
#[test]
fn strand_with_repeated_nucleotide() {
    process_nucleotidecounts_case("GGGGGGG", &[('A', 0), ('T', 0), ('C', 0), ('G', 7)]);
}
#[test]
/// strand with multiple nucleotides
fn strand_with_multiple_nucleotides() {
    process_nucleotidecounts_case(
        "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC",
        &[('A', 20), ('T', 21), ('C', 12), ('G', 17)],
    );
}
#[test]
fn counts_invalid_nucleotide_results_in_err() {
    assert_eq!(nucleotide_counts("GGXXX"), Err('X'));
}
#[test]
/// strand with invalid nucleotides
fn strand_with_invalid_nucleotides() {
    assert_eq!(nucleotide_counts("AGXXACT"), Err('X'),);
}
