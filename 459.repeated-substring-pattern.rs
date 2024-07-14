/*
 * @lc app=leetcode id=459 lang=rust
 *
 * [459] Repeated Substring Pattern
 */

// @lc code=start
impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let n = s.len();
        let chars: Vec<char> = s.chars().collect();
        for i in 1..=n / 2 {
            if n % i == 0 {
                let mut j = 0;
                while j + i < n && chars[j] == chars[j + i] {
                    j += 1;
                }
                if j + i == n {
                    return true;
                }
            }
        }
        false
    }
}
// @lc code=end
