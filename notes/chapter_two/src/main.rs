fn main() {
    println!("{}", gcd(12, 24))
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

    assert_eq!(gcd(
        2 * 3 * 5 * 11 * 17,
        3 * 7 * 11 * 13* 19
    ), 3 * 11);
}
