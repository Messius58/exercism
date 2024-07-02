use std::time::{self};

mod checksum;
mod numbers;
mod beer;
mod raindrop;
mod kindergarten_garden;
mod proverb;
mod matching_brackets;
mod eliuds_eggs;
mod bob;
mod high_scores;
mod clock;
mod sublist;
mod dot_dsl;
mod etl;
mod nucleotide;
mod cypher;
mod alphametics;
mod singlelinkedlist;

fn main() {

    /************** insérer ci-dessous le code à mesurer (penser à retirer les //) ******************/
    // let a = alphametics::backtracking::solver::solve("I + BB == ILL"); OK A RETESTER SI MODIF
    // let b = alphametics::backtracking::solver::solve("A + A + A + A + A + A + A + A + A + A + A + B == BCC"); OK A RETESTER SI MODIF
    //let c = alphametics::backtracking::solver::solve("SEND + MORE == MONEY");
    let c = numbers::euler::larger_series::lsp("1234a5", 2);
//    assert_eq!(Err(numbers::euler::Error::InvalidDigit('a')), c);
    assert_eq!(Ok(72), c);
    // let start = time::Instant::now();
    // let elapse = start.elapsed();
    // println!("elapsed vec : {:.2?}", elapse);
}

pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

pub fn is_leap_year(year: u64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}