pub struct Solution;

impl Solution {
    pub fn char2int(a: &char) -> i32 {
        match a {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => todo!(),
        }
    }

    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let mut prev_value = 0;
        for ch in s.chars().rev() {
            let value = Self::char2int(&ch);
            if value < prev_value {
                result -= value;
            } else {
                result += value;
            }
            prev_value = value;
        }
        result as _
    }
}
