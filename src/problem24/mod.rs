use itertools::Itertools;

pub fn lexicographic_decimal_permutation(index: usize) -> usize {
    (0..10u32)
        .permutations(10)
        .nth(index - 1)
        .unwrap()
        .into_iter()
        .map(|v| char::from_digit(v, 10).unwrap())
        .join("")
        .parse()
        .unwrap()
}
