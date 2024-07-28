/*
 * @lc app=leetcode id=147 lang=rust
 *
 * [147] Insertion Sort List
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut sorted_head: Option<Box<ListNode>> = None;
        let mut current = head;

        while let Some(mut node) = current {
            current = node.next.take();

            if sorted_head.is_none() || sorted_head.as_ref().unwrap().val >= node.val {
                node.next = sorted_head;
                sorted_head = Some(node);
            } else {
                let mut prev = &mut sorted_head;
                while let Some(ref mut sorted_node) = prev {
                    if sorted_node.next.is_none()
                        || sorted_node.next.as_ref().unwrap().val >= node.val
                    {
                        node.next = sorted_node.next.take();
                        sorted_node.next = Some(node);
                        break;
                    }
                    prev = &mut sorted_node.next;
                }
            }
        }

        sorted_head
    }
}
// @lc code=end
