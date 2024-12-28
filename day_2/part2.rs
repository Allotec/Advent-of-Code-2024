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
    let nums_list = gen_all_perms(nums);

    for nums in nums_list.iter() {
        let nums = nums.as_slice();

        if (all_inc(nums) || all_dec(nums)) && level_diff(nums) {
            return true;
        }
    }

    false
}

fn gen_all_perms(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut nums_list: Vec<Vec<i32>> = Vec::new();
    nums_list.push(nums.to_vec());

    for i in 0..nums.len() {
        let mut nums = nums.to_vec();
        nums.remove(i);

        nums_list.push(nums);
    }

    nums_list
}

fn all_inc(nums: &[i32]) -> bool {
    for window in nums.windows(2) {
        if window[0] < window[1] {
            return false;
        }
    }

    true
}

fn all_dec(nums: &[i32]) -> bool {
    for window in nums.windows(2) {
        if window[0] > window[1] {
            return false;
        }
    }

    true
}

fn level_diff(nums: &[i32]) -> bool {
    for window in nums.windows(2) {
        let diff = (window[0] - window[1]).abs();

        if diff == 0 || diff > 3 {
            return false;
        }
    }

    true
}

#[test]
fn part2_test() {
    let nums1 = vec![7, 6, 4, 2, 1];
    assert!(is_safe(nums1.as_slice()));

    let nums2 = vec![1, 2, 7, 8, 9];
    assert!(!is_safe(nums2.as_slice()));

    let nums3 = vec![9, 7, 6, 2, 1];
    assert!(!is_safe(nums3.as_slice()));

    let nums4 = vec![1, 3, 2, 4, 5];
    assert!(is_safe(nums4.as_slice()));

    let nums5 = vec![8, 6, 4, 4, 1];
    assert!(is_safe(nums5.as_slice()));

    let nums6 = vec![1, 3, 6, 7, 9];
    assert!(is_safe(nums6.as_slice()));

    let nums7 = vec![6, 7, 9, 11, 13, 14, 18];
    assert!(is_safe(nums7.as_slice()));

    let nums8 = vec![6, 4, 7, 8];
    assert!(is_safe(nums8.as_slice()));
}
