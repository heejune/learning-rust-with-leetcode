struct Solution {}

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums.clone();
        nums.sort();
        //nums.iter().rev().take(3).sum(); element의 합이 0이 될 수 있으므로 사용할 수 없음
        let mut diff = i32::MAX;

        let size = nums.len();

        for i in 0..size {
            let mut lo = i + 1;
            let mut hi = size - 1;

            if diff == 0 {
                break;
            }

            // diff가 제일 작은 값을 찾는다
            while lo < hi {
                let sum = nums[i] + nums[lo] + nums[hi];
                if (target - sum).abs() < diff.abs() {
                    // diff에 abs 안해줘서 한참 삽질
                    diff = target - sum;
                }

                if sum < target {
                    lo += 1;
                } else {
                    hi -= 1;
                }
            }
        }

        println!("{}, {}", target, diff);
        target - diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v1 = vec![1, 1, -1, -1, 3];
        assert_eq!(Solution::three_sum_closest(v1, -1), -1);

        let v2 = vec![0, 0, 0];
        assert_eq!(Solution::three_sum_closest(v2, 1), 0);
    }
}
