
/**
 * bubble sort
 */
fn bubbleSort(array: Vec<i32>) -> Vec<i32> {
    let mut array = array;
    for i in 0..(array.len() - 1) {
        for j in 0..(array.len() - 1 - i) {
            if array[j + 1] < array[j] {
                array.swap(j, j + 1);
            }
        }
    }

    return array;
}

#[test]
fn test_bubble_sort () {
    let demo = vec![3, 5, 6, 1, 0, 2, 9];
    let array = bubbleSort(demo);

    for number in &array {
        println!("{}", number);
    }
}

/**
 * select sort
 */
fn selectSort(array: Vec<i32>) -> Vec<i32> {
    let mut array = array;

    for i in 0..array.len() {
        for j in (i+1)..array.len() {
            if array[j] < array[i] {
                array.swap(i, j);
            }
        }
    }

    return array;
}

#[test]
fn test_select_sort() {
    let demo = vec![3, 5, 6, 1, 0, 2, 10, 9];
    let array = selectSort(demo);

    for number in &array {
        println!("{}", number);
    }
}

/**
 * insertion sort
 */
fn insertionSort(array: Vec<i32>) -> Vec<i32> {
    let mut array = array;

    for i in 1..array.len() {
        let mut j = i;
        while j > 0 && array[j] < array[j - 1] {
            array.swap(j, j - 1);
            j = j - 1;
        }
    }

    return array;
}

#[test]
fn test_insertion_sort() {
    let demo = vec![3, 5, 6, 1, 0, 2, 10, 9];
    let array = insertionSort(demo);

    for number in &array {
        println!("{}", number);
    }
}

/**
 * merge sort
 */
fn mergeSort(array: Vec<i32>) -> Vec<i32> {
    let mut array = array;
    let mid = array.len() / 2;
    if mid == 0 {
        return array
    }

    let mut left_array = mergeSort(array[..mid].to_owned());
    let mut right_array = mergeSort(array[mid..].to_owned());

    let mut left = 0; // Head of left pile.
    let mut right = 0; // Head of right pile.
    let mut index = 0;

    let mut ret = array;
    // Compare element and insert back to result array.
    while left < left_array.len() && right < right_array.len() {
        if left_array[left] <= right_array[right] {
            ret[index] = left_array[left];
            index += 1;
            left += 1;
        } else {
            ret[index] = right_array[right];
            index += 1;
            right += 1;
        }
    }

    // Copy the reset elements to returned array.
    // `memcpy` may be more performant than for-loop assignment.
    if left < left_array.len() {
        ret[index..].copy_from_slice(&left_array[left..]);
    }
    if right < right_array.len() {
        ret[index..].copy_from_slice(&right_array[right..]);
    }

    return ret;
}

#[test]
fn test_merge_sort() {
    let demo = vec![3, 5, 6, 1, 0, 2, 10, 9];
    let array = mergeSort(demo);

    for number in &array {
        println!("{}", number);
    }
}


/**
 * quick sort
 */
fn quickSort(array: &mut [i32]) {
    let n = array.len();
    if n > 1 {
        let mut j = 0;
        for i in 0..n {
            if &array[i] < &array[n - 1] {
                array.swap(i, j);
                j += 1;
            }
        }
        array.swap(j, n - 1); // pivot
        quickSort(&mut array[0..j]);
        quickSort(&mut array[j + 1..n]);
    }
}

#[test]
fn test_quick_sort() {
    let mut demo = vec![3, 5, 6, 1, 0, 2, 10, 9];
    quickSort(&mut demo);
    println!("{:?}", demo);
}

use std::rc::Rc;
use std::cell::RefCell;


#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/**
 * binary tree: pre-order
 */
fn binaryTreePreOrder(root: Option<Rc<RefCell<TreeNode>>>) {
    if let Some(node) = root.clone().unwrap().borrow().left.clone() {
        println!("{}", node.borrow().val);
    }

    if let Some(node) = root.clone().unwrap().borrow().right.clone() {
        println!("{}", node.borrow().val);
    }
}

#[test]
fn test_binary_tree_pre_order() {
    let root = Rc::new(RefCell::new(TreeNode::new(5)));
    root.clone().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.clone().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(6))));

    binaryTreePreOrder(Some(root))
}

struct UnionFind {
    root: Vec<i32>
}

impl UnionFind {
    fn union(&mut self, target1: i32, target2: i32) {
        let r1 = self.find(target1 as usize);
        let r2 = self.find(target2 as usize);
        if r1 != r2 {
            for i in 0..self.root.len() {
               if self.root[i] == r2 {
                   self.root[i] = r1;
               }
            }
        }

        for i in 0..self.root.len() {
            print!("{}\t", self.root[i]);
        }
        println!();
    }

    fn find(&self, t: usize) -> i32 {
        return self.root[t];
    }

    fn connected(&self, t1: i32, t2: i32) -> bool{
        self.root[t1 as usize] == self.root[t2 as usize]
    }

    fn new(size: usize) -> UnionFind {

        let mut obj = UnionFind{
            root: vec![0; size]
        };

        for i in 0..obj.root.len() {
            obj.root[i] = i as i32;
        }

        return obj
    }


}
#[test]
fn test_union_find() {
    let mut obj = UnionFind::new(10);

    obj.union(1, 2);
    obj.union(2, 5);
    obj.union(5, 6);
    obj.union(6, 7);
    obj.union(3, 8);
    obj.union(8, 9);

    println!("{}", obj.connected(1, 5));
    println!("{}", obj.connected(5, 7));
    println!("{}", obj.connected(4, 9));

}

struct QuickUnion {
    root: Vec<i32>
}

impl QuickUnion {

    fn union(&mut self, target1: i32, target2: i32) {
        let r1 = self.find(target1 as usize);
        let r2 = self.find(target2 as usize);
        if r1 != r2 {
            self.root[r2 as usize] = r1;
        }

        for i in 0..self.root.len() {
            print!("{}\t", self.root[i]);
        }
        println!();
    }

    fn find(&self, t: usize) -> i32 {
        if self.root[t] == t as i32 {
            return t as i32;
        }
        return self.find(self.root[t] as usize);
    }

    fn connected(&self, t1: i32, t2: i32) -> bool{
        self.find(t1 as usize) == self.find(t2 as usize)
    }

    fn new(size: usize) -> QuickUnion {

        let mut obj = QuickUnion{
            root: vec![0; size]
        };

        for i in 0..obj.root.len() {
            obj.root[i] = i as i32;
        }

        return obj
    }
}

#[test]
fn test_quick_union() {
    let mut obj = QuickUnion::new(10);

    obj.union(1, 2);
    obj.union(2, 5);
    obj.union(5, 6);
    obj.union(6, 7);
    obj.union(3, 8);
    obj.union(8, 9);

    println!("{}", obj.connected(1, 5));
    println!("{}", obj.connected(5, 7));
    println!("{}", obj.connected(4, 9));
}

struct OrderedUnion {
    root: Vec<i32>,
    rank: Vec<i32>
}

impl OrderedUnion {
    fn union(&mut self, target1: i32, target2: i32) {
        let r1 = self.find(target1 as usize);
        let r2 = self.find(target2 as usize);

        if r1 != r2 {
            if self.rank[r1 as usize] > self.rank[r2 as usize] {
                self.root[r2 as usize] = r1
            } else if self.rank[r1 as usize] < self.rank[r2 as usize] {
                self.root[r1 as usize] = r2
            } else {
                self.root[r2 as usize] = r1;
                self.rank[r2 as usize]+=1;
            }
        }

        for i in 0..self.root.len() {
            print!("{}\t", self.root[i]);
        }
        println!();
    }

    fn find(&mut self, t: usize) -> i32 {
        if self.root[t] == t as i32 {
            return t as i32;
        }

        let tmp = self.root[t];
        self.root[tmp as usize] = self.find(self.root[t] as usize);
        return self.root[t];
    }

    fn connected(&mut self, t1: i32, t2: i32) -> bool{
        self.find(t1 as usize) == self.find(t2 as usize)
    }

    fn new(size: usize) -> OrderedUnion{

        let mut obj = OrderedUnion{
            root: vec![0; size],
            rank: vec![1; size]
        };

        for i in 0..obj.root.len() {
            obj.root[i] = i as i32;
        }

        return obj
    }}

#[test]
fn test_ordered_union() {
    let mut obj = OrderedUnion::new(10);

    obj.union(1, 2);
    obj.union(2, 5);
    obj.union(5, 6);
    obj.union(6, 7);
    obj.union(3, 8);
    obj.union(8, 9);

    println!("{}", obj.connected(1, 5));
    println!("{}", obj.connected(5, 7));
    println!("{}", obj.connected(4, 9));
}


fn main() {
    println!("Hello base algorithm");
}
