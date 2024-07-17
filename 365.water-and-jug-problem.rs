/*
 * @lc app=leetcode id=365 lang=rust
 *
 * [365] Water and Jug Problem
 */

// @lc code=start
impl Solution {
    pub fn can_measure_water(x: i32, y: i32, target: i32) -> bool {
        let (x, y, target) = (x as i64, y as i64, target as i64);
        if x + y < target {
            return false;
        }
        if x == 0 || y == 0 {
            return target == 0 || x + y == target;
        }
        target % gcd(x, y) == 0
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
// @lc code=end
