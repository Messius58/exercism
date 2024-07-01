use std::collections::HashMap;

/*************************************************************** */

fn primes_generated(start: u32, end: u32) -> Vec<u32> {
    let mut num_naturals = HashMap::with_capacity((end - start) as usize);
    for num in start..=end {
        num_naturals.insert(num, true);
    }
    let sqrt_end = (end as f64).sqrt() as u32;
    for i in start..=sqrt_end {
        if num_naturals[&i] {
            for j in ((i.pow(2))..=end).step_by(i as usize) {
                num_naturals.insert(j, false);
            }
        }
    }
    let mut v = num_naturals.into_iter().filter(|n| n.1).map(|m| m.0).collect::<Vec<u32>>();
    v.sort();
    v
}

pub fn nth(n: u32) -> u32 {
    let rslt = primes_generated(2, 105_000);
    rslt[n as usize]
}

#[test]
fn first_prime() {
    let output = nth(0);
    let expected = 2;
    assert_eq!(output, expected);
}

#[test]
fn second_prime() {
    let output = nth(1);
    let expected = 3;
    assert_eq!(output, expected);
}

#[test]
fn sixth_prime() {
    let output = nth(5);
    let expected = 13;
    assert_eq!(output, expected);
}

#[test]

fn big_prime() {
    let output = nth(10000);
    let expected = 104743;
    assert_eq!(output, expected);
}
