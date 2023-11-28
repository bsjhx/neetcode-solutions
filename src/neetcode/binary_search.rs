#[cfg(test)]
mod test {

    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len());

        while l < r {
            let med = l + (r - l) / 2;
            if target < nums[med] {
                r = med;
            } else if target > nums[med] {
                l = med + 1;
            } else if target == nums[med] {
                return med as i32;
            }
        }

        -1
    }

    #[test]
    fn test_search() {
        assert_eq!(search(vec![2, 5], 2), 0);
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12, 14], 9), 4);
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }
}
