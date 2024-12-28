mod part1;
mod part2;

fn main() {
    let input = include_str!("./input.txt")
        .split("\n")
        .collect::<Vec<&str>>();

    println!("Part 1- {}", part1::part1(input.as_slice()));
    println!("Part 2- {}", part2::part2(input.as_slice()));
}
