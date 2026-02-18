// @leet start

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map:HashMap<i32,usize> =HashMap::new();

        for (i, &val ) in nums.iter().enumerate() {
        let compliment = target - val;
            if let Some(&index) = map.get(&compliment) {
                return vec![index as i32,i as i32];
            }
            map.insert(val, i);
        }
        vec![]
    }
}
// @leet end
