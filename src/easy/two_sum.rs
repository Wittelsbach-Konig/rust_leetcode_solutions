use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut delta_hashmap = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let delta = target - num;
            if let Some(&index) = delta_hashmap.get(&delta) {
                return vec![index as i32, i as i32];
            }
            delta_hashmap.insert(num, i as i32);
        }
        vec![]
    }
}
