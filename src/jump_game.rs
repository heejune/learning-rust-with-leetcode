#[derive(Clone, PartialEq)]
enum Index {
    Good,
    Bad,
    Unknown,
}

struct Solution {}

struct Solution2 {}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut memo = vec![Index::Unknown; nums.len()];

        let last_index = memo.len() - 1;
        memo[last_index] = Index::Good;

        // 뒤에서부터 체크한다, 가장 마지막 element는 Good로 셋팅되었고
        // 바로 그 이전부터(len()-2) 0번째 인덱스까지
        for (pos, num) in nums.iter().enumerate().rev() {
            // skip last element
            if pos == nums.len() - 1 {
                continue;
            }

            let max_possible_jump = std::cmp::min(pos + nums[pos] as usize, nums.len() - 1);

            for next_pos in (pos + 1)..(max_possible_jump + 1) {
                if memo[next_pos] == Index::Good {
                    memo[pos] = Index::Good;
                    break;
                }
            }
        }

        memo[0] == Index::Good
    }
}

// or Greedy approach
impl Solution2 {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut last_pos = nums.len() - 1;

        for (pos, num) in nums.iter().enumerate().rev() {
            // 만약 해당 index(pos)에서 마지막 good까지 갈 수 있다면
            if (pos + *num as usize) >= last_pos {
                // 마지막 good pos를 업데이트한다
                last_pos = pos
            }
        }

        last_pos == 0
    }
}
