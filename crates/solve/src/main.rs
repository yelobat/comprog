//! My submission to A. Sleeping Through Classes
//!
//! This contains some Rust boilerplate for parsing the input
//! of the problem as well as common data structures. It is
//! currently a work in progress and will be iterated upon
//! as I solve more and more problems.

use std::io;

/// This is just a simple u32 type read from stdin.
fn read_u32() -> u32 {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read a line");
    line.trim().parse().unwrap()
}

/// This is white-space separated list of items.
/// # Example
/// 4 1 2 -> vec![4, 1, 2]
///
/// It would be nice to generalise this to parse an
/// whitespace separated list of things.
fn read_vec_u32() -> Vec<u32> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read a line");
    line
        .split(" ")
        .filter(|&s| s != " ")
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

/// This is just a simple String type read from stdin.
fn read_string() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read a line");
    line.trim().to_string()
}

fn solve(_n: u32, k: u32, s: String) -> u32 {
    let mut ans: u32 = 0;
    let mut allowed: u32 = 0;
    for c in s.chars() {
        if c == '1' {
            allowed = k;
        } else {
            if allowed == 0 {
                ans += 1;
            } else {
                allowed -= 1;
            }
        }
    }

    ans
}

fn main() {
    let t: u32 = read_u32();

    for _ in 0..t {
        let v = read_vec_u32();
        let n: u32 = v[0];
        let k: u32 = v[1];
        let s: String = read_string();
        println!("{}", solve(n, k, s));
    }
}

/// Write the test cases below here
#[test]
fn sample_input() {
    assert_eq!(solve(4, 1, "1001".into()), 1);
    assert_eq!(solve(3, 3, "000".into()), 3);
    assert_eq!(solve(3, 1, "001".into()), 2);
    assert_eq!(solve(8, 2, "01000101".into()), 2);
}
