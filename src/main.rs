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

use numbers::knapsack::Item;

fn main() {

    /************** insérer ci-dessous le code à mesurer (penser à retirer les //) ******************/
    // let a = alphametics::backtracking::solver::solve("I + BB == ILL"); OK A RETESTER SI MODIF
    // let b = alphametics::backtracking::solver::solve("A + A + A + A + A + A + A + A + A + A + A + B == BCC"); OK A RETESTER SI MODIF
    //let c = alphametics::backtracking::solver::solve("SEND + MORE == MONEY");

    let max_weight = 750;
    let items = [
        Item {
            weight: 70,
            value: 135,
        },
        Item {
            weight: 73,
            value: 139,
        },
        Item {
            weight: 77,
            value: 149,
        },
        Item {
            weight: 80,
            value: 150,
        },
        Item {
            weight: 82,
            value: 156,
        },
        Item {
            weight: 87,
            value: 163,
        },
        Item {
            weight: 90,
            value: 173,
        },
        Item {
            weight: 94,
            value: 184,
        },
        Item {
            weight: 98,
            value: 192,
        },
        Item {
            weight: 106,
            value: 201,
        },
        Item {
            weight: 110,
            value: 210,
        },
        Item {
            weight: 113,
            value: 214,
        },
        Item {
            weight: 115,
            value: 221,
        },
        Item {
            weight: 118,
            value: 229,
        },
        Item {
            weight: 120,
            value: 240,
        },
    ];
    let expected = 1458;

    let start = time::Instant::now();
    let output = numbers::knapsack::maximum_value(max_weight, &items);
    assert_eq!(output, expected);
    let elapse = start.elapsed();
    println!("elapsed vec : {:.2?}", elapse);
}

pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

pub fn is_leap_year(year: u64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}