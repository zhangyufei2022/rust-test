/// 删除排序数组中的重复项
pub fn remove_duplicates(nums: &mut Vec<i32>) -> usize {
    let mut i: usize = 0;

    for j in 1..nums.len() {
        if nums[i] != nums[j] {
            i += 1;
            nums[i] = nums[j];
        }
    }

    i + 1
}

#[cfg(test)]
mod tests {
    use super::remove_duplicates;

    #[test]
    fn test() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];

        let count = remove_duplicates(&mut nums);

        assert_eq!(count, 5);
        assert_eq!(nums[0..count], vec![0, 1, 2, 3, 4]);
    }
}
