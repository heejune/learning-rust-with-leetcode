struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        // 해석되지 않는 더미 0을 하나 덧붙여서 마지막 문자까지 체크할 수 있도록 한다. (문자 2개씩 체크하므로)
        let iter = format!("{}0", s).chars().collect::<Vec<char>>();
        let win = iter.windows(2);
        let mut result = 0;

        for w in win {
            match w {
                ['I', 'V'] => result -= 1,
                ['I', 'X'] => result -= 1,
                ['I', _] => result += 1,

                ['V', _] => result += 5,

                ['X', 'L'] => result -= 10,
                ['X', 'C'] => result -= 10,
                ['X', _] => result += 10,

                ['L', _] => result += 50,

                ['C', 'D'] => result -= 100,
                ['C', 'M'] => result -= 100,
                ['C', _] => result += 100,

                ['D', _] => result += 500,

                ['M', _] => result += 1000,

                _ => (),
            }
            println!("{:?}, {:?}", w, result);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
    }
}
