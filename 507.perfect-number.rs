/*
 * @lc app=leetcode id=507 lang=rust
 *
 * [507] Perfect Number
 */

// @lc code=start
impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        let mut sum = 1;
        let mut i = 2;
        while i * i <= num {
            if num % i == 0 {
                sum += i;
                if i * i != num {
                    sum += num / i;
                }
            }
            i += 1;
        }
        sum == num && num != 1
    }
}
// @lc code=end
