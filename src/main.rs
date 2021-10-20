#[macro_use]
extern crate quicli;

use std::collections::HashMap;

use itertools::Itertools;
use num_bigint::{BigUint, ToBigUint};
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

    scratch
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
    let lower = (10usize).pow(digits - 1);
    let upper: usize = (10usize).pow(digits);

    let mut biggest = 1;

    for i in lower..upper {
        for j in i..upper {
            let v = i * j;
            let repr = v.to_string();
            if repr == repr.chars().rev().collect::<String>() && v > biggest {
                biggest = v;
            }
        }
    }

    biggest
}

fn smallest_multiple(upper: usize) -> usize {
    let mut tr = 2520;
    let soln;

    loop {
        let mut scratch: Vec<usize> = (tr..(tr * 2)).into_iter().collect();
        for sieve in 11..(upper + 1) {
            scratch.retain(|&x| x % sieve == 0);
        }

        if scratch.get(0).is_some() {
            soln = *scratch.get(0).unwrap();
            break;
        }

        tr *= 2;
    }

    soln
}

struct TriangleGenerator {
    nth: usize,
    val: usize,
}

impl TriangleGenerator {
    fn new(nth: usize) -> TriangleGenerator {
        TriangleGenerator {
            nth,
            val: (1..(nth + 1)).sum(),
        }
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
                break; // Break the for-loop
            }
        }
        // Possible to starve?
    }

    prime_ftors.sort_unstable();
    prime_ftors
        .into_iter()
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

fn factorial_digit_sum(z: usize) -> usize {
    (1..=z)
        .map(|v| v.to_biguint().unwrap())
        .product::<BigUint>()
        .to_string()
        .chars()
        .map(|ch| ch.to_digit(10))
        .map(Option::unwrap)
        .sum::<u32>() as usize
}

fn square_lattice(dim: usize) -> usize {
    let mut buf = (1..=dim).map(|_| vec![1usize; dim]).collect::<Vec<_>>();

    for ii in (0..=(dim - 2)).rev() {
        for jj in (0..=(dim - 2)).rev() {
            buf[ii][jj] = buf[ii + 1][jj] + buf[ii][jj + 1];
        }
    }

    buf[0][0] as usize
}

fn fib_term_len(len: usize) -> usize {
    let mut gen = (2, 1.to_biguint().unwrap(), 1.to_biguint().unwrap());

    loop {
        gen = (gen.0 + 1, gen.2.clone(), gen.1 + gen.2);

        if gen.2.to_string().len() >= len {
            break gen.0;
        }
    }
}

fn _number_letter_counts(val: usize) -> &'static str {
    let map = vec![
        (1000, "onethousand"),
        (90, "ninety"),
        (80, "eighty"),
        (70, "seventy"),
        (60, "sixty"),
        (50, "fifty"),
        (40, "forty"),
        (30, "thirty"),
        (20, "twenty"),
        (19, "nineteen"),
        (18, "eighteen"),
        (17, "seventeen"),
        (16, "sixteen"),
        (15, "fifteen"),
        (14, "fourteen"),
        (13, "thirteen"),
        (12, "twelve"),
        (11, "eleven"),
        (10, "ten"),
        (9, "nine"),
        (8, "eight"),
        (7, "seven"),
        (6, "six"),
        (5, "five"),
        (4, "four"),
        (3, "three"),
        (2, "two"),
        (1, "one"),
        (0, ""),
    ]
    .into_iter()
    .collect::<HashMap<usize, _>>();

    map[&val]
}

fn number_letter_counts(max: usize) -> usize {
    if max == 0 {
        return 0;
    }

    let res = match max {
        1000 => _number_letter_counts(1000).len(),
        90 => _number_letter_counts(90).len(),
        80 => _number_letter_counts(80).len(),
        70 => _number_letter_counts(70).len(),
        60 => _number_letter_counts(60).len(),
        50 => _number_letter_counts(50).len(),
        40 => _number_letter_counts(40).len(),
        30 => _number_letter_counts(30).len(),
        20 => _number_letter_counts(20).len(),
        19 => _number_letter_counts(19).len(),
        18 => _number_letter_counts(18).len(),
        17 => _number_letter_counts(17).len(),
        16 => _number_letter_counts(16).len(),
        15 => _number_letter_counts(15).len(),
        14 => _number_letter_counts(14).len(),
        13 => _number_letter_counts(13).len(),
        12 => _number_letter_counts(12).len(),
        11 => _number_letter_counts(11).len(),
        10 => _number_letter_counts(10).len(),
        9 => _number_letter_counts(9).len(),
        8 => _number_letter_counts(8).len(),
        7 => _number_letter_counts(7).len(),
        6 => _number_letter_counts(6).len(),
        5 => _number_letter_counts(5).len(),
        4 => _number_letter_counts(4).len(),
        3 => _number_letter_counts(3).len(),
        2 => _number_letter_counts(2).len(),
        1 => _number_letter_counts(1).len(),
        z if z >= 100 => {
            let rem = z % 100;
            let hundreds = z / 100;
            let rem_size = if rem == 0 {
                0
            } else {
                "and".len() + {
                    if (11..=19).contains(&rem) {
                        let teens = _number_letter_counts(rem);
                        teens.len()
                    } else {
                        let rem2 = rem % 10;
                        let tens = rem - rem2;
                        if rem2 == 0 {
                            let tens = _number_letter_counts(rem);
                            tens.len()
                        } else {
                            let str_tens = _number_letter_counts(tens);
                            let str_ones = _number_letter_counts(rem2);
                            str_tens.len() + str_ones.len()
                        }
                    }
                }
            };
            let str_hundreds = format!("{}{}", _number_letter_counts(hundreds), "hundred");
            str_hundreds.len() + rem_size
        }
        z if z >= 10 => {
            let rem = z % 10;
            if rem == 0 {
                let tens = _number_letter_counts(z);
                tens.len()
            } else {
                let tens = _number_letter_counts(z - rem);
                let ones = _number_letter_counts(rem);
                tens.len() + ones.len()
            }
        }
        _ => unimplemented!(),
    };

    res + number_letter_counts(max - 1)
}

fn amicable_numbers(max: usize) -> usize {
    let mut amicable_sum = 0;
    (2..max)
        .map(|num| {
            (
                num,
                (1..num)
                    .filter(|num2| num % num2 == 0)
                    .reduce(|a, b| a + b)
                    .unwrap(),
            )
        })
        .fold(HashMap::new(), |mut map, (idx, d)| {
            map.insert(idx, d);
            if map
                .get(&d)
                .map(|v| *v == idx && *v != d)
                .unwrap_or_else(|| false)
            {
                amicable_sum += d + idx;
            }

            map
        });

    amicable_sum
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
        15 => square_lattice(21),
        17 => number_letter_counts(1000),
        20 => factorial_digit_sum(100),
        21 => amicable_numbers(10000),
        25 => fib_term_len(1000),
        _ => unimplemented!(),
    };

    println!("{}\t{}", MyDuration::from(time.elapsed().unwrap()), soln);
});
