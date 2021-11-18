struct Solution {}

struct Solution2 {}

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        // greedy
        let mut n = num;
        let integers = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let roman = [
            "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
        ];
        let mut result = String::new();

        // 각 자리수를 뺄 수 있는지 체크
        for i in 0..integers.len() {
            if n >= integers[i] {
                // 더 가능한지 뺄 수 있을때까지 더 뺀다
                while n >= integers[i] {
                    n -= integers[i];
                    result += roman[i];
                }
            }
        }

        result
    }
}

impl Solution2 {
    pub fn int_to_roman(num: i32) -> String {
        let thousands = ["", "M", "MM", "MMM"];
        let hundreds = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
        let tens = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
        let ones = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];

        format!(
            "{}{}{}{}",
            thousands[(num / 1000) as usize],
            hundreds[(num % 1000 / 100) as usize],
            tens[(num % 100 / 10) as usize],
            ones[(num % 10) as usize]
        )
    }
}
