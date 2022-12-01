
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