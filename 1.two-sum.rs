/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums_map = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = nums_map.get(&(target - num)) {
                return vec![j as i32, i as i32];
            }
            nums_map.insert(num, i);
        }
        vec![]
    }
}
// @lc code=end
