use std::iter::zip;

fn main() {
    let input = include_str!("./input.txt")
        .split("\n")
        .collect::<Vec<&str>>();

    println!("Part 1- {}", part1(input.as_slice()));
}

fn part1(input: &[&str]) -> i32 {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input {
        let split = line.split("   ").collect::<Vec<&str>>();
        if split.len() == 1 {
            break;
        }

        let mut split = split.into_iter();

        left.push(split.next().unwrap().to_string());
        right.push(split.next().unwrap().to_string());
    }

    left.sort();
    right.sort();

    zip(left, right)
        .map(|(left, right)| {
            let left = left.parse::<i32>().unwrap();
            let right = right.parse::<i32>().unwrap();

            (left - right).abs()
        })
        .sum::<i32>()
}
