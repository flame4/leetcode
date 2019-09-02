#[allow(dead_code)]
struct MyQueue {
    container_: Vec<i32>,
    tmp: Vec<i32>,
}


/// https://leetcode.com/problems/implement-queue-using-stacks/
/// 使用栈模拟队列, 实现 push, pop, peek(获得第一个元素), empty.
/// 栈在 rust 内就是 vec 的实现(只用 pop + push api)
/// 实现思路比较朴实的, 倒着插入栈即可, 也就是每次插入栈先把 container 的依次写入 tmp,
/// 然后添加新的, 再把 tmp 里的写入 container.
/// 声明 tmp 为了避免重复分配内存.
///
/// `&self` means the method takes an immutable reference.
/// If you need a mutable reference, change it to `&mut self` instead.
#[allow(dead_code)]
impl MyQueue {
    fn new() -> Self {
        MyQueue { container_: vec![], tmp: vec![] }
    }

    // 实现思路就是维持 container 内的数据就是按照头顶是队列头的顺序(头顶实际是vec尾巴),
    // 新来一个数据就把它放到栈底(也就是 vec 头部). O(n) 时间复杂度.
    fn push(&mut self, x: i32) {
        MyQueue::transfer(&mut self.container_, &mut self.tmp);
        self.tmp.push(x);
        MyQueue::transfer(&mut self.tmp, &mut self.container_);
    }

    // O(1)
    fn pop(&mut self) -> i32 {
        self.container_.pop().unwrap()
    }

    // O(1)
    fn peek(&self) -> i32 {
        *self.container_.last().unwrap()
    }

    // O(1)
    fn empty(&self) -> bool {
        self.container_.is_empty()
    }

    // 把 source 内的数据转移到 target 内.  O(n)
    fn transfer(source: &mut Vec<i32>, target: &mut Vec<i32>) {
        assert_eq!(target.is_empty(), true);
        while let Some(val) = source.pop() {
            target.push(val)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MyQueue;

    #[test]
    pub fn myqueue_test() {
        let mut queue = MyQueue::new();

        queue.push(1);
        queue.push(2);
        assert_eq!(queue.peek(), 1);
        assert_eq!(queue.pop(), 1);
        assert_eq!(queue.empty(), false);
        assert_eq!(queue.pop(), 2);
        assert_eq!(queue.empty(), true);
    }
}
