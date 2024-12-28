use regex::Regex;

#[derive(Clone, Copy, Debug)]
enum Matches {
    DoM(usize),
    DontM(usize),
}

pub fn part2(input: &[&str]) -> i32 {
    let hay = concat_input(input);
    let re_mul = Regex::new(r"mul\((?<num1>[0-9]+),(?<num2>[0-9]+)\)").unwrap();
    let ranges = gen_valid_ranges(hay.as_str());

    re_mul
        .captures_iter(hay.as_str())
        .map(|caps| {
            let num1 = caps.name("num1").unwrap().as_str().parse::<i32>().unwrap();
            let num2 = caps.name("num2").unwrap().as_str().parse::<i32>().unwrap();
            let recon = format!("mul({num1},{num2})");
            let position = hay.find(recon.as_str()).unwrap();

            if in_range(ranges.as_slice(), position) {
                num1 * num2
            } else {
                0
            }
        })
        .sum::<i32>()
}

fn in_range(ranges: &[(usize, usize)], val: usize) -> bool {
    for range in ranges.iter() {
        if val > range.0 && val < range.1 {
            return true;
        }
    }

    false
}

fn gen_valid_ranges(hay: &str) -> Vec<(usize, usize)> {
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();

    let do_matches = re_do
        .find_iter(hay)
        .map(|s| s.start())
        .collect::<Vec<usize>>();

    let dont_matches = re_dont
        .find_iter(hay)
        .map(|s| s.start())
        .collect::<Vec<usize>>();

    let mut ranges = Vec::<(usize, usize)>::new();
    if !dont_matches.is_empty() {
        ranges.push((0, dont_matches[0]));
    }
    let merged_matches = merge_matches(do_matches, dont_matches);

    let mut last_val: Option<Matches> = None;

    for matched_val in merged_matches.iter() {
        if last_val.is_none() {
            last_val = Some(*matched_val);
            continue;
        }

        match last_val.unwrap() {
            Matches::DoM(val) => {
                if let Matches::DontM(val_curr) = *matched_val {
                    ranges.push((val, val_curr));
                    last_val = None;
                }
            }
            _ => last_val = None,
        }
    }

    if let Matches::DoM(val) = merged_matches.last().unwrap() {
        ranges.push((*val, hay.len()));
    }

    ranges
}

fn merge_matches(mut do_matches: Vec<usize>, mut dont_matches: Vec<usize>) -> Vec<Matches> {
    let mut merged_matches = Vec::<Matches>::new();

    while !do_matches.is_empty() && !dont_matches.is_empty() {
        let do_val = do_matches.first().unwrap();
        let dont_val = dont_matches.first().unwrap();

        if do_val < dont_val {
            merged_matches.push(Matches::DoM(*do_val));
            do_matches.remove(0);
        } else {
            merged_matches.push(Matches::DontM(*dont_val));
            dont_matches.remove(0);
        }
    }

    while !do_matches.is_empty() {
        let do_val = do_matches.first().unwrap();
        merged_matches.push(Matches::DoM(*do_val));
        do_matches.remove(0);
    }

    while !dont_matches.is_empty() {
        let do_val = dont_matches.first().unwrap();
        merged_matches.push(Matches::DontM(*do_val));
        dont_matches.remove(0);
    }

    merged_matches
}

fn concat_input(input: &[&str]) -> String {
    let mut concated = String::new();

    input.iter().for_each(|s| {
        concated.push_str(s);
    });

    concated
}

#[test]
fn test1() {
    let test_str = ["xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"];
    assert_eq!(part2(test_str.as_slice()), 48);
}
