// Ordre ADN : A(denine), C(ytosine), G(uanine), T(hymine)
// Ordre ARN : A(denine), C(ytosine), G(uanine), U(racil)
// Liaison ADN - ARN : A -> U | C -> G | G -> C | T <- A

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
struct Dna {
    strand: Vec<char>
}

impl Dna {
    const DNA_NUCLEIDE_BASE: [char; 4] = ['A', 'C', 'G', 'T'];

    fn new(dna: &str) -> Result<Dna, usize> {
        let strand = Vec::from_iter(dna.chars());
        error_detection(&Dna::DNA_NUCLEIDE_BASE, &strand)?;
        Ok(Dna {
            strand
        })
    }

    fn into_rna(self) -> Rna {
        let base_pairing: HashMap<&char, &char> = HashMap::from_iter(Dna::DNA_NUCLEIDE_BASE.iter().zip(Rna::RNA_NUCLEIDE_BASE.iter().rev()));
        let strand = self.strand.iter().map(|base| *base_pairing[base]).collect::<Vec<char>>();
        Rna { strand }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Rna {
    strand: Vec<char>
}

impl Rna {
    const RNA_NUCLEIDE_BASE: [char; 4] = ['A', 'C', 'G', 'U'];

    fn new(rna: &str) -> Result<Rna, usize> {
        let strand = Vec::from_iter(rna.chars());
        error_detection(&Rna::RNA_NUCLEIDE_BASE, &strand)?;
        Ok(Rna {
            strand
        })
    }
}

fn error_detection(nucleide_base: &[char; 4], dna: &Vec<char>) -> Result<(), usize> {
    let mut dna = dna.iter();
    if let Some(pos) = dna.position(|base| !nucleide_base.contains(base)) {
        return Err(pos)
    }
    Ok(())
}

#[test]
fn empty_rna_sequence() {
    let input = "";
    let output = Dna::new(input).unwrap().into_rna();
    let expected = Rna::new("").unwrap();
    assert_eq!(output, expected);
}
#[test]
fn rna_complement_of_cytosine_is_guanine() {
    let input = "C";
    let output = Dna::new(input).unwrap().into_rna();
    let expected = Rna::new("G").unwrap();
    assert_eq!(output, expected);
}
#[test]
fn rna_complement_of_guanine_is_cytosine() {
    let input = "G";
    let output = Dna::new(input).unwrap().into_rna();
    let expected = Rna::new("C").unwrap();
    assert_eq!(output, expected);
}
#[test]
fn rna_complement_of_thymine_is_adenine() {
    let input = "T";
    let output = Dna::new(input).unwrap().into_rna();
    let expected = Rna::new("A").unwrap();
    assert_eq!(output, expected);
}
#[test]
fn rna_complement_of_adenine_is_uracil() {
    let input = "A";
    let output = Dna::new(input).unwrap().into_rna();
    let expected = Rna::new("U").unwrap();
    assert_eq!(output, expected);
}
#[test]
fn rna_complement() {
    let input = "ACGTGGTCTTAA";
    let output = Dna::new(input).unwrap().into_rna();
    let expected = Rna::new("UGCACCAGAAUU").unwrap();
    assert_eq!(output, expected);
}
#[test]
fn invalid_dna_input() {
    let input = "U";
    let output = Dna::new(input);
    let expected = Err(0);
    assert_eq!(output, expected);
}
#[test]
fn invalid_dna_input_at_offset() {
    let input = "ACGTUXXCTTAA";
    let output = Dna::new(input);
    let expected = Err(4);
    assert_eq!(output, expected);
}
#[test]
fn invalid_rna_input() {
    let input = "T";
    let output = Rna::new(input);
    let expected = Err(0);
    assert_eq!(output, expected);
}
#[test]
fn invalid_rna_input_at_offset() {
    let input = "ACGTUXXCTTAA";
    let output = Rna::new(input);
    let expected = Err(3);
    assert_eq!(output, expected);
}