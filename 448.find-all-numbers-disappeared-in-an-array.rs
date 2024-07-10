/*
 * @lc app=leetcode id=448 lang=rust
 *
 * [448] Find All Numbers Disappeared in an Array
 */

// @lc code=start
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut nums = nums;
        let mut res = vec![];
        for i in 0..n {
            let index = (nums[i].abs() - 1) as usize;
            if nums[index] > 0 {
                nums[index] = -nums[index];
            }
        }
        for i in 0..n {
            if nums[i] > 0 {
                res.push(i as i32 + 1);
            }
        }
        res
    }
}
// @lc code=end
