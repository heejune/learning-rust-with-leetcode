struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut lo: usize = 0;
        let mut hi = nums.len() - 1;

        while lo <= hi {
            let mut mid = (lo + hi) / 2;

            println!("{}:{}:{}", lo, mid, hi);

            if lo == hi && nums[mid] != target {
                return -1;
            }

            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] > target {
                hi = mid.saturating_sub(1);
            } else {
                lo = mid + 1;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::search(vec![5], -5), -1);
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 13), -1);
    }
}
