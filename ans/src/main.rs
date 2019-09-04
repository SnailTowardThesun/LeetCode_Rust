/**
 * 1. two sum
 * Given an array of integers, return indices of the two numbers such that they add up to a specific target.
 * You may assume that each input would have exactly one solution, and you may not use the same element twice.
 *
 * Example:
 * Given nums = [2, 7, 11, 15], target = 9,

 * Because nums[0] + nums[1] = 2 + 7 = 9,
 * return [0, 1].
 */
use std::collections::HashMap;
use std::option::Option;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    let mut m = HashMap::new();

    for (index, value) in nums.iter().enumerate() {
        let num = target - *value;
        if m.contains_key(&num) {
            return vec![m[&num], index as i32];
        }
        m.insert(value, index as i32);
    }

    return v;
}

#[test]
fn test_two_sum() {
    let v = vec![2, 7, 11, 9];
    let ans = two_sum(v, 9);

    assert_eq!(ans[0], 0);
    assert_eq!(ans[1], 1);
}

/**
 * 2. add two numbers
 * You are given two non-empty linked lists representing two non-negative integers.
 * The digits are stored in reverse order and each of their nodes contain a single digit.
 * Add the two numbers and return it as a linked list.
 *
 * You may assume the two numbers do not contain any leading zero, except the number 0 itself.
 *
 * Example:
 * Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
 * Output: 7 -> 0 -> 8
 * Explanation: 342 + 465 = 807.
 */
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut ret = Some(Box::new(ListNode::new(0)));
    
    let (mut p1, mut p2, mut carry) = (l1, l2, 0);

    let mut pos = ret.as_mut();
    while p1.is_some() || p2.is_some() {
        let mut sum = carry;

        if let Some(v) = p1 {
            sum += v.val;
            p1 = v.next;
        }

        if let Some(v) = p2 {
            sum += v.val;
            p2 = v.next;
        }

        if sum >= 10 {
            carry = 1;
            sum = sum - 10;
        } else {
            carry = 0;
        }

        let v = pos.unwrap();
        v.next = Some(Box::new(ListNode::new(sum)));
        pos = v.next.as_mut();
    }

    if carry > 0 {
        let v = pos.unwrap();
        v.next = Some(Box::new(ListNode::new(1)));
    }

    return ret.unwrap().next;
}

#[test]
fn test_add_two_numbers() {
    assert_eq!(1, 1);

    let mut l1 = Box::new(ListNode::new(2));
    l1.next = Some(Box::new(ListNode::new(4)));
    l1.next.as_mut().unwrap().next= Some(Box::new(ListNode::new(7)));

    let mut l2 = Box::new(ListNode::new(5));
    l2.next = Some(Box::new(ListNode::new(6)));
    l2.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));

    let ret = add_two_numbers(Some(l1), Some(l2));

    let expected_ret = [7, 0, 2, 1];
    let mut pos = ret;
    for i in (0..expected_ret.len()) {
        assert_eq!(pos.is_some(), true);
        if !pos.is_some() {
            break;
        }

        if let Some(v) = pos {
            assert_eq!(v.val, expected_ret[i]);
            pos = v.next;
        }

    }
}

fn main() {
    println!("the answer of leetcode.com using rust");
}
