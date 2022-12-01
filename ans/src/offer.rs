
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