use std::{collections::HashMap, env, process::exit};

use num_bigint::BigUint;
use rayon::prelude::*;
extern crate num_bigint;
extern crate num_integer;

fn get_euler_map(n: usize) -> HashMap<(usize, usize), BigUint> {
    let mut euler_map = HashMap::new();
    for n in 1..=n {
        for m in 0..=(n - 1) / 2 {
            if m == 0 || n - 1 == m {
                euler_map.insert((n, m), BigUint::from(1usize));
            } else {
                euler_map.insert(
                    (n, m),
                    (n - m) * get_euler(&euler_map, n - 1, m - 1)
                        + (m + 1) * get_euler(&euler_map, n - 1, m),
                );
            }
        }
    }
    euler_map
}

fn get_euler(euler_map: &HashMap<(usize, usize), BigUint>, n: usize, m: usize) -> &BigUint {
    let m = if 2 * m >= n { n - m - 1 } else { m };
    euler_map.get(&(n, m)).unwrap()
}

fn ordered_bell_numbers(n: usize) -> Vec<BigUint> {
    let euler_map = get_euler_map(n - 1);

    (0..n)
        .into_par_iter()
        .map(|n| {
            if n == 0 {
                BigUint::from(1usize) // Normally this belongs in get_euler but I don't want to wrestle the borrow checker
            } else {
                (0..n)
                    .into_par_iter()
                    .map(|m| get_euler(&euler_map, n, m) * BigUint::from(2u32).pow(m as u32))
                    .sum::<BigUint>()
            }
        })
        .collect()
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

    println!("{:?}", ordered_bell_numbers(n_parsed.unwrap()));
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
            BigUint::from(7087261usize)
        ],
        ordered_bell_numbers(10)
    );
}
