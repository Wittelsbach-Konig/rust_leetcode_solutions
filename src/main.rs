mod easy;
fn main() {
    easy::roman_to_integer::Solution::roman_to_int(String::from("III"));
    easy::palindrome_number::Solution::is_palindrome(121);
    easy::two_sum::Solution::two_sum(vec![2, 7, 11, 15], 9);
}
