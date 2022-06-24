// https://leetcode.com/problems/remove-duplicates-from-sorted-array/
fn remove_dups(nums: &mut Vec<i32>) -> i32 {
    let mut last_write = 0usize;
    let mut last_value = 0i32;

    for i in 0..nums.len() {
        let val = nums[i];
        if i == 0 {
            // skip the first element
            last_value = val;
            continue;
        }
        if val == last_value {
            // we've encountered a duplicate continue on
            continue;
        }
        // we have the first occurrence of a new value
        last_write += 1;
        last_value = val;
        nums[last_write] = last_value;
    }

    return (last_write + 1) as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_dups() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(2, remove_dups(&mut nums));
        assert_eq!(1, nums[0]);
        assert_eq!(2, nums[1]);
    }
}