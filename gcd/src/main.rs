use std::env;
use std::str::FromStr;

fn main() {
    let mut numbers: Vec<u64> = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("Error parsing argument\n"));
    }

    if numbers.len() == 0 {
        eprint!("Program requires at least one number to be passed as argument\n");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for x in &numbers[1..] {
        d = gcd(d, *x);
    }

    print!("The greatest commond divisor of {:?} is {}\n", numbers, d);
}

fn gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(m != 0 && n != 0);
    while n > 0 {
        if n < m {
            let t = n;
            n = m;
            m = t;
        }
        n = n % m;
    }
    m
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(8, 12), 4);
}
