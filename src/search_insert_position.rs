struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut low : usize = 0;
        let mut high : usize = nums.len() - 1;

        while low <= high {

            let mid = low + (high - low) /2;

            if let Some(cur) = nums.get(mid) {
                if *cur == target {
                    return mid as i32;
                }
                else if *cur < target {
                    low = mid  + 1
                }
                else if *cur > target {
                    if mid == 0 {
                        break;
                    }
                    high = mid - 1  
                }
            }
        }

        low as i32
    }
}

#[cfg(test)]
mod tests {
        use super::*;

    #[test]
    fn it_works() {

        let v1 = vec![1,3,5,6];
        assert_eq!(Solution::search_insert(v1, 5), 2);

        let v2 = vec![1,3,5,6];
        assert_eq!(Solution::search_insert(v2, 2), 1);

        let v3 = vec![1,3,5,6];
        assert_eq!(Solution::search_insert(v3, 7), 4);

        let v4 = vec![1,3,5,6];
        assert_eq!(Solution::search_insert(v4, 0), 0);
    }
}
