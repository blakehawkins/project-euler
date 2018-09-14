extern crate itertools;

#[macro_use]
extern crate quicli;

use itertools::Itertools;
use quicli::prelude::*;
use std::fmt;

#[derive(Debug, StructOpt)]
struct Cli {
    problem_number: usize,
}

struct MyDuration {
    pub d: std::time::Duration,
}

impl From<std::time::Duration> for MyDuration {
    fn from(d: std::time::Duration) -> Self {
        MyDuration { d }
    }
}

impl fmt::Display for MyDuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}'{}\"", self.d.as_secs() / 60, self.d.as_secs() % 60)
    }
}

fn multiples_of_3_and_5(n: usize) -> usize {
    (3..n)
        .into_iter()
        .filter(|x| x % 3 == 0 || x % 5 == 0)
        .fold(0, |x, accum| accum + x)
}

fn even_fibonacci_numbers(f: usize) -> usize {
    let mut scratch = vec![1, 2];
    let mut a_b = (1, 2);
    while a_b.0 + a_b.1 < f {
        a_b = (a_b.1, a_b.0 + a_b.1);
        scratch.push(a_b.1);
    }

    scratch
        .into_iter()
        .filter(|n| n % 2 == 0)
        .fold(0, |x, accum| accum + x)
}

fn _primes_through(z: usize) -> Vec<usize> {
    let mut scratch: Vec<usize> = (2..z).into_iter().collect();
    for s in scratch.clone().iter() {
        scratch.retain(|&x| x == *s || x % s != 0);
    }

    scratch.clone()
}

fn largest_prime_factor(z: usize) -> usize {
    let mut n = z;

    let mut cap = 2;

    while cap < n {
        let primes = _primes_through(cap);

        primes.into_iter().for_each(|x| {
            if n % x == 0 {
                n /= x;
            }
        });

        cap *= 2;
    }

    n
}

fn largest_palindrome_product(digits: u32) -> usize {
    let lower = (10 as usize).pow(digits - 1);
    let upper: usize = (10 as usize).pow(digits);

    let mut biggest = 1;

    for i in lower..upper {
        for j in i..upper {
            let v = i * j;
            let repr = v.to_string();
            if repr == repr.chars().rev().collect::<String>() {
                if v > biggest {
                    biggest = v;
                }
            }
        }
    }

    biggest
}

fn smallest_multiple(upper: usize) -> usize {
    let mut try = 2520;
    let soln;

    loop {
        let mut scratch: Vec<usize> = (try..(try * 2)).into_iter().collect();
        for sieve in 11..(upper + 1) {
            scratch.retain(|&x| x % sieve == 0);
        }

        if scratch.get(0).is_some() {
            soln = scratch.get(0).unwrap().clone();
            break;
        }

        try *= 2;
        println!("{}", try);
    }

    soln
}

struct TriangleGenerator {
    nth: usize,
    val: usize,
}

impl TriangleGenerator {
    fn new(nth: usize) -> TriangleGenerator {
        TriangleGenerator { nth, val: (1..(nth + 1)).sum() }
    }
}

impl Iterator for TriangleGenerator {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.nth += 1;
        self.val += self.nth;

        Some(self.val)
    }
}

fn prime_ftor_count(val: usize) -> usize {
    let mut prime_ftors = vec![];

    let mut _val = val;
    while _val > 1 {
        for v in 2..(_val + 1) {
            if _val % v == 0 {
                prime_ftors.push(v);
                _val /= v;
                break;  // Break the for-loop
            }
        }
        // Possible to starve?
    }

    prime_ftors.sort();
    prime_ftors.into_iter()
               .group_by(|elt| *elt)
               .into_iter()
               .map(|(_k, grp)| grp.count() + 1)
               .product()
}

fn divisible_triangle(divisors: usize) -> usize {
    for val in TriangleGenerator::new(8) {
        if prime_ftor_count(val) > divisors {
          return val;
        }
    }

    0
}

main!(|args: Cli| {
    let time = std::time::SystemTime::now();

    let soln = match args.problem_number {
        1 => multiples_of_3_and_5(1000),
        2 => even_fibonacci_numbers(4_000_000),
        3 => largest_prime_factor(600_851_475_143),
        4 => largest_palindrome_product(3),
        5 => smallest_multiple(20),
        12 => divisible_triangle(500),
        _ => unimplemented!(),
    };

    println!("{}\t{}", MyDuration::from(time.elapsed().unwrap()), soln);
});
