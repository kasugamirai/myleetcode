/*
 * @lc app=leetcode id=274 lang=rust
 *
 * [274] H-Index
 */

// @lc code=start
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut citations = citations;
        citations.sort();
        let n = citations.len() as i32;
        for i in 0..n {
            if citations[(n - i - 1) as usize] >= i + 1 {
                continue;
            }
            return i;
        }
        n
    }
}
// @lc code=end
