/*
 * @lc app=leetcode id=415 lang=rust
 *
 * [415] Add Strings
 */

// @lc code=start
impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut res = String::new();
        let mut carry = 0;
        let mut i = num1.len() as i32 - 1;
        let mut j = num2.len() as i32 - 1;
        while i >= 0 || j >= 0 {
            let mut sum = carry;
            if i >= 0 {
                sum += num1.chars().nth(i as usize).unwrap() as i32 - '0' as i32;
                i -= 1;
            }
            if j >= 0 {
                sum += num2.chars().nth(j as usize).unwrap() as i32 - '0' as i32;
                j -= 1;
            }
            carry = sum / 10;
            res.push((sum % 10 + '0' as i32) as u8 as char);
        }
        if carry > 0 {
            res.push((carry + '0' as i32) as u8 as char);
        }
        res.chars().rev().collect()
    }
}
// @lc code=end
