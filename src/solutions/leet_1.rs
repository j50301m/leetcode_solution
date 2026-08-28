use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for i in 0..nums.len() {
            let t = target - nums[i];
            let Some(&idx) = map.get(&t) else {
                map.insert(nums[i], i);
                continue;
            };

            return vec![i as i32, idx as i32];
        }
        Vec::new()
    }
}
