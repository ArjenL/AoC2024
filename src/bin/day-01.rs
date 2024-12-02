use itertools::Itertools;
use std::{collections::HashMap, io};

fn main() -> anyhow::Result<()> {
    let input = io::read_to_string(io::stdin())?;

    let (left, right): (Vec<_>, Vec<_>) = input
        .split_whitespace()
        .map(|s| s.parse::<u32>())
        .tuples()
        .unzip();
    let mut left = left.into_iter().collect::<Result<Vec<_>, _>>()?;
    let mut right = right.into_iter().collect::<Result<Vec<_>, _>>()?;

    left.sort();
    right.sort();

    let distance: u32 = left
        .iter()
        .zip(right.iter())
        .map(|(&l, &r)| l.abs_diff(r))
        .sum();

    println!("Distance between the lists: {distance}");

    let mut right_map: HashMap<u32, u32> = HashMap::new();
    for n in right.iter() {
        *right_map.entry(*n).or_default() += 1;
    }
    let similarity_score: u32 = left
        .iter()
        .map(|n| n * *right_map.entry(*n).or_insert(0))
        .sum();

    println!("Similarity score of the lists: {similarity_score}");
    Ok(())
}
