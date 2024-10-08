use std::collections::HashMap;
/*
See exercise arn transcription
*/
#[derive(Debug, PartialEq, Eq)]
struct Rna {
    strand: Vec<char>,
}

impl Rna {
    const RNA_NUCLEIDE_BASE: [char; 4] = ['A', 'C', 'G', 'U'];

    fn new(rna: &str) -> Result<Rna, usize> {
        let strand = Vec::from_iter(rna.chars());
        error_detection(&Rna::RNA_NUCLEIDE_BASE, &strand)?;
        Ok(Rna { strand })
    }
}

fn error_detection(nucleide_base: &[char; 4], dna: &Vec<char>) -> Result<(), usize> {
    let mut dna = dna.iter();
    if let Some(pos) = dna.position(|base| !nucleide_base.contains(base)) {
        return Err(pos);
    }
    Ok(())
}

struct CodonsInfo<'a> {
    proteins: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    const RNA_NUCLEIDE_BASE: [char; 4] = ['A', 'C', 'G', 'U'];

    fn name_for(&self, codon: &str) -> Option<&'a str> {
        if let Some(v) = self.proteins.get(codon) {
            Some(*v)
        } else {
            None
        }
    }

    fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let arn = match Rna::new(rna) {
            Err(_) => return None,
            Ok(x) => x,
        };

        let proteins = arn.strand.chunks(3)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>()
            .iter()
            .filter(|t| t.len() == 3 && self.proteins.contains_key(t.as_str()))
            .map(|t| self.proteins[t.as_str()])
            .collect::<Vec<&str>>();

        if proteins.contains(&"stop codon") || arn.strand.len() % 3 == 0 {
            let mut split = proteins.split(|condon| *condon == "stop codon");
            Some(split.next().unwrap().to_vec())
        } else {
            None
        }
    }
}

fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        proteins: HashMap::from_iter(pairs),
    }
}

#[test]
fn methionine() {
    let info = parse(make_pairs());
    assert_eq!(info.name_for("AUG"), Some("methionine"));
}
#[test]
fn cysteine_tgt() {
    let info = parse(make_pairs());
    assert_eq!(info.name_for("UGU"), Some("cysteine"));
}
#[test]
fn stop() {
    let info = parse(make_pairs());
    assert_eq!(info.name_for("UAA"), Some("stop codon"));
}
#[test]
fn valine() {
    let info = parse(make_pairs());
    assert_eq!(info.name_for("GUU"), Some("valine"));
}
#[test]
fn isoleucine() {
    let info = parse(make_pairs());
    assert_eq!(info.name_for("AUU"), Some("isoleucine"));
}
#[test]
fn arginine_cga() {
    let info = parse(make_pairs());
    assert_eq!(info.name_for("CGA"), Some("arginine"));
}
#[test]
fn arginine_aga() {
    let info = parse(make_pairs());
    assert_eq!(info.name_for("AGA"), Some("arginine"));
}
#[test]
fn arginine_agg() {
    let info = parse(make_pairs());
    assert_eq!(info.name_for("AGG"), Some("arginine"));
}
#[test]
fn empty_is_invalid() {
    let info = parse(make_pairs());
    assert_eq!(info.name_for(""), None);
}
#[test]
fn x_is_not_shorthand_so_is_invalid() {
    let info = parse(make_pairs());
    assert_eq!(info.name_for("VWX"), None);
}
#[test]
fn too_short_is_invalid() {
    let info = parse(make_pairs());
    assert_eq!(info.name_for("AU"), None);
}
#[test]
fn too_long_is_invalid() {
    let info = parse(make_pairs());
    assert_eq!(info.name_for("ATTA"), None);
}
#[test]
fn empty_rna_sequence_results_in_no_proteins() {
    let info = parse(make_pairs());
    assert_eq!(info.of_rna(""), Some(vec![]),);
}
#[test]
fn methionine_rna_sequence() {
    let info = parse(make_pairs());
    assert_eq!(info.of_rna("AUG"), Some(vec!["methionine"]),);
}
#[test]
fn phenylalanine_rna_sequence_1() {
    let info = parse(make_pairs());
    assert_eq!(info.of_rna("UUU"), Some(vec!["phenylalanine"]),);
}
#[test]
fn phenylalanine_rna_sequence_2() {
    let info = parse(make_pairs());
    assert_eq!(info.of_rna("UUC"), Some(vec!["phenylalanine"]),);
}
#[test]
fn leucine_rna_sequence_1() {
    let info = parse(make_pairs());
    assert_eq!(info.of_rna("UUA"), Some(vec!["leucine"]),);
}
#[test]
fn leucine_rna_sequence_2() {
    let info = parse(make_pairs());
    assert_eq!(info.of_rna("UUG"), Some(vec!["leucine"]),);
}
#[test]
fn serine_rna_sequence_1() {
    let info = parse(make_pairs());
    assert_eq!(info.of_rna("UCU"), Some(vec!["serine"]),);
}
#[test]
fn serine_rna_sequence_2() {
    let info = parse(make_pairs());
    assert_eq!(info.of_rna("UCC"), Some(vec!["serine"]),);
}
#[test]
fn serine_rna_sequence_3() {
    let info = parse(make_pairs());
    assert_eq!(info.of_rna("UCA"), Some(vec!["serine"]),);
}
#[test]
fn serine_rna_sequence_4() {
    let info = parse(make_pairs());
    assert_eq!(info.of_rna("UCG"), Some(vec!["serine"]),);
}
#[test]
fn tyrosine_rna_sequence_1() {
    let info = parse(make_pairs());
    assert_eq!(info.of_rna("UAU"), Some(vec!["tyrosine"]),);
}
#[test]
fn tyrosine_rna_sequence_2() {
    let info = parse(make_pairs());
    assert_eq!(info.of_rna("UAC"), Some(vec!["tyrosine"]),);
}
#[test]
fn cysteine_rna_sequence_1() {
    let info = parse(make_pairs());
    assert_eq!(info.of_rna("UGU"), Some(vec!["cysteine"]),);
}
#[test]
fn cysteine_rna_sequence_2() {
    let info = parse(make_pairs());
    assert_eq!(info.of_rna("UGC"), Some(vec!["cysteine"]),);
}
#[test]
fn tryptophan_rna_sequence() {
    let info = parse(make_pairs());
    assert_eq!(info.of_rna("UGG"), Some(vec!["tryptophan"]),);
}
#[test]
fn stop_codon_rna_sequence_1() {
    let info = parse(make_pairs());
    assert_eq!(info.of_rna("UAA"), Some(vec![]),);
}
#[test]
fn stop_codon_rna_sequence_2() {
    let info = parse(make_pairs());
    assert_eq!(info.of_rna("UAG"), Some(vec![]),);
}
#[test]
fn stop_codon_rna_sequence_3() {
    let info = parse(make_pairs());
    assert_eq!(info.of_rna("UGA"), Some(vec![]),);
}
#[test]
fn sequence_of_two_protein_codons_translates_into_proteins() {
    let info = parse(make_pairs());
    assert_eq!(
        info.of_rna("UUUUUU"),
        Some(vec!["phenylalanine", "phenylalanine"]),
    );
}
#[test]
fn sequence_of_two_different_protein_codons_translates_into_proteins() {
    let info = parse(make_pairs());
    assert_eq!(info.of_rna("UUAUUG"), Some(vec!["leucine", "leucine"]),);
}
#[test]
fn translate_rna_strand_into_correct_protein_list() {
    let info = parse(make_pairs());
    assert_eq!(
        info.of_rna("AUGUUUUGG"),
        Some(vec!["methionine", "phenylalanine", "tryptophan"]),
    );
}
#[test]
fn translation_stops_if_stop_codon_at_beginning_of_sequence() {
    let info = parse(make_pairs());
    assert_eq!(info.of_rna("UAGUGG"), Some(vec![]),);
}
#[test]
fn translation_stops_if_stop_codon_at_end_of_two_codon_sequence() {
    let info = parse(make_pairs());
    assert_eq!(info.of_rna("UGGUAG"), Some(vec!["tryptophan"]),);
}
#[test]
fn translation_stops_if_stop_codon_at_end_of_three_codon_sequence() {
    let info = parse(make_pairs());
    assert_eq!(
        info.of_rna("AUGUUUUAA"),
        Some(vec!["methionine", "phenylalanine"]),
    );
}
#[test]
fn translation_stops_if_stop_codon_in_middle_of_three_codon_sequence() {
    let info = parse(make_pairs());
    assert_eq!(info.of_rna("UGGUAGUGG"), Some(vec!["tryptophan"]),);
}
#[test]
fn translation_stops_if_stop_codon_in_middle_of_six_codon_sequence() {
    let info = parse(make_pairs());
    assert_eq!(
        info.of_rna("UGGUGUUAUUAAUGGUUU"),
        Some(vec!["tryptophan", "cysteine", "tyrosine"]),
    );
}
#[test]
fn unknown_amino_acids_not_part_of_a_codon_can_t_translate() {
    let info = parse(make_pairs());
    assert_eq!(info.of_rna("XYZ"), None,);
}
#[test]
fn incomplete_rna_sequence_can_t_translate() {
    let info = parse(make_pairs());
    assert_eq!(info.of_rna("AUGU"), None,);
}
#[test]
fn incomplete_rna_sequence_can_translate_if_valid_until_a_stop_codon() {
    let info = parse(make_pairs());
    assert_eq!(
        info.of_rna("UUCUUCUAAUGGU"),
        Some(vec!["phenylalanine", "phenylalanine"]),
    );
}
// The input data constructor. Returns a list of codon, name pairs.
fn make_pairs() -> Vec<(&'static str, &'static str)> {
    let grouped = vec![
        ("isoleucine", vec!["AUU", "AUC", "AUA"]),
        ("valine", vec!["GUU", "GUC", "GUA", "GUG"]),
        ("phenylalanine", vec!["UUU", "UUC"]),
        ("methionine", vec!["AUG"]),
        ("cysteine", vec!["UGU", "UGC"]),
        ("alanine", vec!["GCU", "GCC", "GCA", "GCG"]),
        ("glycine", vec!["GGU", "GGC", "GGA", "GGG"]),
        ("proline", vec!["CCU", "CCC", "CCA", "CCG"]),
        ("threonine", vec!["ACU", "ACC", "ACA", "ACG"]),
        ("serine", vec!["UCU", "UCC", "UCA", "UCG"]),
        ("tyrosine", vec!["UAU", "UAC"]),
        ("tryptophan", vec!["UGG"]),
        ("glutamine", vec!["CAA", "CAG"]),
        ("asparagine", vec!["AAU", "AAC"]),
        ("histidine", vec!["CAU", "CAC"]),
        ("glutamic acid", vec!["GAA", "GAG"]),
        ("aspartic acid", vec!["GAU", "GAC"]),
        ("lysine", vec!["AAA", "AAG"]),
        ("arginine", vec!["CGU", "CGC", "CGA", "CGG", "AGA", "AGG"]),
        ("leucine", vec!["UUA", "UUG"]),
        ("stop codon", vec!["UAA", "UAG", "UGA"]),
    ];
    let mut pairs = Vec::<(&'static str, &'static str)>::new();
    for (name, codons) in grouped.into_iter() {
        for codon in codons {
            pairs.push((codon, name));
        }
    }
    pairs.sort_by(|&(_, a), &(_, b)| a.cmp(b));
    pairs
}
