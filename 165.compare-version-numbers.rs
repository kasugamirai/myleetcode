/*
 * @lc app=leetcode id=165 lang=rust
 *
 * [165] Compare Version Numbers
 */

// @lc code=start
impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let v1: Vec<i32> = version1.split('.').map(|s| s.parse().unwrap()).collect();
        let v2: Vec<i32> = version2.split('.').map(|s| s.parse().unwrap()).collect();
        let n1 = v1.len();
        let n2 = v2.len();
        let n = std::cmp::max(n1, n2);
        for i in 0..n {
            let a = if i < n1 { v1[i] } else { 0 };
            let b = if i < n2 { v2[i] } else { 0 };
            if a < b {
                return -1;
            } else if a > b {
                return 1;
            }
        }
        0
    }
}
// @lc code=end
