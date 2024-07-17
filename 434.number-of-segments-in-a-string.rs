/*
 * @lc app=leetcode id=434 lang=rust
 *
 * [434] Number of Segments in a String
 */

// @lc code=start
impl Solution {
    pub fn count_segments(s: String) -> i32 {
        let mut count = 0;
        let mut prev = ' ';
        for c in s.chars() {
            if c != ' ' && prev == ' ' {
                count += 1;
            }
            prev = c;
        }
        count
    }
}
// @lc code=end
