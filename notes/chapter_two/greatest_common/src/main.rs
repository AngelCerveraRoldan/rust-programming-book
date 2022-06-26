use std::env;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(
            // This will return Ok(v) or Err(v)
            u64::from_str(&arg).expect("Error parsing"),
        );
    }

    if numbers.len() == 0 {
        eprint!("Please enter numbers as arguments");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {numbers:?} is {d}")
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);

    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }

        m = m % n;
    }
    // We dont need to write `return`, if the last line doesnt contain `;`
    // it will be returned by the function
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}
