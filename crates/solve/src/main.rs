//! My submission to A. Little Fairy's Painting
//!
//! This contains some Rust boilerplate for parsing the input
//! of the problem as well as common data structures. It is
//! currently a work in progress and will be iterated upon
//! as I solve more and more problems.

use std::collections::HashSet;
use std::{io, str::FromStr};
use std::fmt::Debug;

/// Parses generic whitespace separated list of types.
/// # Example
/// 4 1 2   -> vec![4, 1, 2]
/// 4 -8 -1 -> vec![4, -8, -1]
fn read_vec<T>() -> Vec<T>
where
    T: FromStr, <T as FromStr>::Err: Debug,
{
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read a line");
    line
        .split_whitespace()
        .map(|s| s.trim().parse::<T>().unwrap())
        .collect()
}

/// Parses a single line containing the type T.
fn read<T>() -> T
where
    T: FromStr, <T as FromStr>::Err: Debug,
{
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read a line");
    line.trim().parse::<T>().unwrap()
}

/// Implement the solution here.
fn solve(_n: u64, a: Vec<u64>) -> u64 {
    let mut s = HashSet::new();
    let mut chosen = a[0];
    for &v in a.iter() {
        s.insert(v);
        chosen = chosen.max(v);
    }

    let l = s.len() as u64;
    for v in a.iter() {
        if v >= &l && v <= &chosen {
            chosen = *v;
        }
    }
    chosen
}

/// Parse the input and pass input into `solve`.
fn main() {
    let t: u32 = read::<u32>();

    for _ in 0..t {
        let n = read::<u64>();
        let a = read_vec::<u64>();
        println!("{}", solve(n, a));
    }
}

/// Write the test cases below here
#[test]
fn sample_input() {
    assert_eq!(solve(6, vec![1, 1, 1, 1, 1, 1]), 1);
    assert_eq!(solve(1, vec![1000]), 1000);
    assert_eq!(solve(5, vec![8, 10, 15, 20, 25]), 8);
    assert_eq!(solve(8, vec![2, 5, 2, 4, 1, 2, 5, 3]), 5);
    assert_eq!(solve(6, vec![40, 4, 1, 95, 8, 40]), 8);
}
