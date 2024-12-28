use regex::Regex;

pub fn part1(input: &[&str]) -> i32 {
    let hay = concat_input(input);
    let re = Regex::new(r"mul\((?<num1>[0-9]+),(?<num2>[0-9]+)\)").unwrap();

    re.captures_iter(hay.as_str())
        .map(|caps| {
            let num1 = caps.name("num1").unwrap().as_str().parse::<i32>();
            let num2 = caps.name("num2").unwrap().as_str().parse::<i32>();
            num1.unwrap() * num2.unwrap()
        })
        .sum::<i32>()
}

fn concat_input(input: &[&str]) -> String {
    let mut concated = String::new();

    input.iter().for_each(|s| {
        concated.push_str(s);
    });

    concated
}
