/*
 * @lc app=leetcode id=482 lang=rust
 *
 * [482] License Key Formatting
 */

// @lc code=start
impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let mut res = String::new();
        let mut count = 0;
        for c in s.chars().rev() {
            if c == '-' {
                continue;
            }
            if count == k {
                res.push('-');
                count = 0;
            }
            res.push(c.to_ascii_uppercase());
            count += 1;
        }
        res.chars().rev().collect()
    }
}
// @lc code=end
