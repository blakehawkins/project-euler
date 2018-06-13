#[macro_use]
extern crate quicli;

use quicli::prelude::*;

#[derive(Debug, StructOpt)]
struct Cli {
    problem_number: usize,
}

fn multiples_of_3_and_5() {
    let soln = (3..1000)
        .into_iter()
        .filter(|x| x % 3 == 0 || x % 5 == 0)
        .fold(0, |x, accum| accum + x);
    println!("{}", soln);
}

main!(|args: Cli| {
    match args.problem_number {
        1 => multiples_of_3_and_5(),
        _ => unimplemented!(),
    }
});
