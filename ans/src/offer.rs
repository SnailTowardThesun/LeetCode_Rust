// Author: hankun1991@outlook.com

use std::borrow::BorrowMut;

/**
* offer.06
*/

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
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

fn reverse_print(head: Option<Box<ListNode>>) ->Vec<i32> {
    let mut ret = vec![];
    let mut pos = &head;
    while let Some(node) = pos {
        ret.push(node.val);
        pos = &node.next;
    }

    ret.reverse();
    return ret;
}

#[test]
fn test_reverse_print() {
    let mut head = Box::new(ListNode{
        val: 1,
        next: None,
    });

    head.next = Some(Box::new(ListNode{
        val: 2,
        next: None,
    }));

    let ret = reverse_print(Some(head));
    for i in ret {
        println!("{}", i)
    }
}

/**
* offer.09
*/
struct CQueue {
    input: Vec<i32>,
    output: Vec<i32>
}

impl CQueue {

    fn new() -> Self {
        CQueue {
            input: Vec::new(),
            output: Vec::new()
        }
    }

    fn append_tail(&mut self, value: i32) {
        self.input.push(value);
    }

    fn delete_head(&mut self) -> i32 {
        if let Some(v) = self.output.pop() {
            return v;
        }

        // fill output with input
        while let Some(v) = self.input.pop() {
            self.output.push(v)
        }

        if self.output.is_empty() {
            return -1;
        }

        return self.output.pop().unwrap()
    }
}

/**
 * Your CQueue object will be instantiated and called as such:
 * let obj = CQueue::new();
 * obj.append_tail(value);
 * let ret_2: i32 = obj.delete_head();
 */
#[test]
fn test_cqueue() {
    let mut obj = CQueue::new();
    obj.append_tail(1);
    let ret_2: i32 = obj.delete_head();

    println!("{}", ret_2)
}

/**
* offer.24
*/
fn reverse_list(head: Option<Box<ListNode>>) ->Option<Box<ListNode>> {
    let mut container = vec![];
    let mut p = &head;
    while let Some(node) = p {
        container.push(node.val);
        p = &node.next;
    }

    container.reverse();
    let mut ret = Some(Box::new(ListNode{
        val: 0,
        next: None,
    }));

    let mut curr = ret.as_mut();
    for i in container {
        if let Some(mut node) = curr {
            node.next = Some(Box::new(ListNode{
                val: i,
                next: None,
            }));
            curr = node.next.as_mut()
        }
    }

    return ret.unwrap().next;
}

fn reverse_list_no_extra_space(head: Option<Box<ListNode>>) ->Option<Box<ListNode>> {
    let mut pre = None;
    let mut cur = head;
    while let Some(mut node) = cur.take() {
        let next = node.next.take();
        node.next = pre;
        pre = Some(node);
        cur = next;

    }

    return pre;
}

#[test]
fn test_reverse_list() {
    let mut head = Some(Box::new(ListNode{
        val: 1,
        next: None,
    }));

    let mut curr = head.as_mut();
    for i in 1..10 {
      if let Some(mut node) = curr {
          node.next = Some(Box::new(ListNode {
              val: i,
              next: None,
          }));
          curr = node.next.as_mut();
      }
    }

    let ret = reverse_list_no_extra_space(head);
    for i in ret {
        println!("{}", i.val)
    }
}

/**
* offer.30
*/

struct MinStack {
    container: Vec<i32>,
    helper: Vec<i32>
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack{
            container: Vec::new(),
            helper: Vec::new()
        }
    }

    fn push(&mut self, x: i32) {
        self.container.push(x);
        if self.helper.is_empty() || x <= *self.helper.last().unwrap() {
            self.helper.push(x);
        }
    }

    fn pop(&mut self) {
        let t = self.container.pop();
        if *self.helper.last().unwrap() == t.unwrap() {
           self.helper.pop();
        }
    }

    fn top(&self) -> i32 {
        *self.container.first().unwrap()
    }

    fn min(&self) -> i32 {
        *self.helper.last().unwrap()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.min();
 */
#[test]
fn test_min_stack() {
    let mut obj = MinStack::new();
    obj.push(1);
    let ret_3: i32 = obj.top();
    let ret_4: i32 = obj.min();
    obj.pop();

    println!("{}, {}", ret_3, ret_4)
}