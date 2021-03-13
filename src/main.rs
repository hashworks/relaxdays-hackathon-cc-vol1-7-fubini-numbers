use std::{env, process::exit, time::Instant};

use num_bigint::BigUint;
use num_integer::binomial;
use num_traits::One;
extern crate num_bigint;
extern crate num_integer;

fn ordered_bell_numbers(n: usize) -> Vec<BigUint> {
    let mut r: Vec<BigUint> = vec![One::one(); n + 1];

    for j in 2..=n {
        for k in j..=n {
            let binominal = binomial(BigUint::from(k), BigUint::from(k - j + 1));
            let previous = r[j - 1].clone();
            r[k] += binominal * previous;
        }
    }

    r
}

fn main() {
    let mut args = env::args();
    let b = args.next().unwrap();
    let n_str = args.next();
    if n_str.is_none() {
        eprintln!("{} <n>", b);
        exit(2);
    }

    let n_parsed = n_str.unwrap().parse::<usize>();

    if n_parsed.is_err() {
        eprintln!("n must be an integer");
        exit(2);
    }

    let s1 = Instant::now();
    println!("{:?}", ordered_bell_numbers(n_parsed.unwrap()));

    println!("Time: {}ms", s1.elapsed().as_millis());
}

#[test]
fn test_10() {
    assert_eq!(
        vec![
            BigUint::from(1usize),
            BigUint::from(1usize),
            BigUint::from(3usize),
            BigUint::from(13usize),
            BigUint::from(75usize),
            BigUint::from(541usize),
            BigUint::from(4683usize),
            BigUint::from(47293usize),
            BigUint::from(545835usize),
            BigUint::from(7087261usize),
            BigUint::from(102247563usize)
        ],
        ordered_bell_numbers(10)
    );
}
