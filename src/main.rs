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

use crate::numbers::bcd::nombre::{reel::Reel, Nombre};

fn main() {

    /************** insérer ci-dessous le code à mesurer (penser à retirer les //) ******************/
    // let a = alphametics::backtracking::solver::solve("I + BB == ILL"); OK A RETESTER SI MODIF
    // let b = alphametics::backtracking::solver::solve("A + A + A + A + A + A + A + A + A + A + A + B == BCC"); OK A RETESTER SI MODIF
    //let c = alphametics::backtracking::solver::solve("SEND + MORE == MONEY");

    /// Create a Decimal from a string literal
    /// Use only when you _know_ that your value is valid.
    fn decimal(input: &str) -> Reel {
        Reel::try_from(input).expect("That was supposed to be a valid value")
    }

    fn nombre(input: &str) -> Nombre {
        Nombre::from(input)
    }
    /// Some big and precise values we can use for testing. [0] + [1] == [2]
    const BIGS: [&str; 3] = [
        "100000000000000000000000000000000000000000000.00000000000000000000000000000000000000001",
        "100000000000000000000000000000000000000000000.00000000000000000000000000000000000000002",
        "200000000000000000000000000000000000000000000.00000000000000000000000000000000000000003",
    ];

    // assert_eq!((nombre("333") * nombre("30")).unwrap(), nombre("9990"));
    // assert_eq!((nombre("333") * nombre("250")).unwrap(), nombre("83250"));
    // assert_eq!((nombre("0.0202") * nombre("0.5")).unwrap(), nombre("0.0101"));
    // assert_eq!((nombre("0.0203") * nombre("0.5")).unwrap(), nombre("0.01015"));
    // assert_eq!(decimal("33.0") * decimal("20.5"), decimal("676.5"));
    // assert_eq!(decimal(BIGS[0]) + decimal(BIGS[1]), decimal(BIGS[2]));
    // assert_eq!(decimal(BIGS[1]) + decimal(BIGS[0]), decimal(BIGS[2]));
    // let n = Reel::try_from("0.1").unwrap();
    // let m = Reel::try_from("0.1").unwrap();
    //assert_eq!((Nombre::from("0") - Nombre::from("1")).unwrap(), Nombre::from("-1"));
    //assert_eq!((Reel::from("-5.5") - Reel::from("-6.5")), Reel::from("1"));
    // assert_eq!((Reel::from("-5.5") - Reel::from("6.5")), Reel::from("-12"));
    // assert_eq!((Reel::from("5.5") - Reel::from("-6.5")), Reel::from("12"));
    // assert_eq!((Reel::from("5.5") - Reel::from("6.5")), Reel::from("-1"));
    assert_eq!(
        decimal("10000000000000000000000000000000000000000000000.999999999999999999999999998",)
            + decimal(
                "10000000000000000000000000000000000000000000000.000000000000000000000000002",
            ),
        decimal("20000000000000000000000000000000000000000000001")
    )
    // println!(" add {} + {}", n, m);
    // let p = n == m;
    // println!(" = {}", p);

    // let start = time::Instant::now();
    // let elapse = start.elapsed();
    // println!("elapsed vec : {:.2?}", elapse);
}

fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

fn is_leap_year(year: u64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}