pub fn part2(input: &[&str]) -> i32 {
    input
        .iter()
        .filter(|row| !row.trim().is_empty())
        .map(|row| {
            let row: Vec<i32> = row
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect();

            safe_to_val(row.as_slice())
        })
        .sum()
}

fn safe_to_val(nums: &[i32]) -> i32 {
    if is_safe(nums) {
        1
    } else {
        0
    }
}

fn is_safe(nums: &[i32]) -> bool {
    let inc = all_inc(nums);
    let dec = all_dec(nums);
    let diff = level_diff(nums);

    if !diff.0 {
        return false;
    }

    //Safe without removing levels
    let inc_no_rem = inc.0 && inc.1 == -1;
    let dec_no_rem = dec.0 && dec.1 == -1;
    let diff_no_rem = diff.0 && diff.1 == -1;

    let mut dbg = format!("Inc- {:?}, ", inc);
    dbg.push_str(format!("Dec- {:?}, ", dec).as_str());
    dbg.push_str(format!("Diff- {:?}", diff).as_str());
    panic!("{dbg}");

    if (inc_no_rem || dec_no_rem) && diff_no_rem {
        return true;
    }

    //Safe with removing levels
    if (diff.1 == inc.1 || diff.1 == -1 || inc.1 == -1) && inc.0 {
        return true;
    }
    if (diff.1 == dec.1 || diff.1 == -1 || dec.1 == -1) && dec.0 {
        return true;
    }

    false
}

fn all_inc(nums: &[i32]) -> (bool, i32) {
    let mut faults = 0;
    let mut remove_level = -1;

    for (i, window) in nums.windows(2).enumerate() {
        if window[0] < window[1] {
            faults += 1;
            remove_level = i as i32;
        }

        if faults == 2 {
            return (false, remove_level);
        }
    }

    (true, remove_level)
}

fn all_dec(nums: &[i32]) -> (bool, i32) {
    let mut faults = 0;
    let mut remove_level = -1;

    for (i, window) in nums.windows(2).enumerate() {
        if window[0] > window[1] {
            faults += 1;
            remove_level = i as i32;
        }

        if faults == 2 {
            return (false, remove_level);
        }
    }

    (true, remove_level)
}

fn level_diff(nums: &[i32]) -> (bool, i32) {
    let mut faults = 0;
    let mut remove_level = -1;

    for (i, window) in nums.windows(2).enumerate() {
        let diff = (window[0] - window[1]).abs();

        if diff == 0 || diff > 3 {
            faults += 1;
            remove_level = i as i32;
        }

        if faults == 2 {
            return (false, remove_level);
        }
    }

    (true, remove_level)
}

#[test]
fn part2_test() {
    // let nums1 = vec![7, 6, 4, 2, 1];
    // assert!(is_safe(nums1.as_slice()));

    let nums2 = vec![1, 2, 7, 8, 9];
    assert!(!is_safe(nums2.as_slice()));

    // let nums3 = vec![9, 7, 6, 2, 1];
    // assert!(!is_safe(nums3.as_slice()));
    //
    // let nums4 = vec![1, 3, 2, 4, 5];
    // assert!(is_safe(nums4.as_slice()));
    //
    // let nums5 = vec![8, 6, 4, 4, 1];
    // assert!(is_safe(nums5.as_slice()));
    //
    // let nums6 = vec![1, 3, 6, 7, 9];
    // assert!(is_safe(nums6.as_slice()));
}
