use std::collections::hash_map::Entry;
use std::{collections::HashMap, iter::zip};

fn main() {
    let input = include_str!("./input.txt")
        .split("\n")
        .collect::<Vec<&str>>();

    println!("Part 1- {}", part1(input.as_slice()));
    println!("Part 2- {}", part2(input.as_slice()));
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

fn part2(input: &[&str]) -> i32 {
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

    let right = right
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let left = left
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut right_set = HashMap::<i32, i32>::new();

    for num in right {
        if let Entry::Vacant(e) = right_set.entry(num) {
            e.insert(1);
            continue;
        }

        right_set.entry(num).and_modify(|num| *num += 1);
    }

    left.iter()
        .map(|num| {
            if !right_set.contains_key(num) {
                0
            } else {
                let multiplier = right_set.get_key_value(num).unwrap().1;
                *num * multiplier
            }
        })
        .sum()
}
