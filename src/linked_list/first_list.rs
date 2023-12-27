struct Node {
    value: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, value: i32) {
        let node = Box::new(Node {
            value,
            // 使用 mem::replace，这个非常有用的函数允许我们从一个借用中偷出一个值的同时再放入一个新值。
            next: std::mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.value)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = std::mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = std::mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn test_list() {
        let mut list1 = List::new();
        assert_eq!(list1.pop(), None);

        list1.push(1);
        list1.push(2);
        list1.push(3);
        assert_eq!(list1.pop(), Some(3));

        list1.push(4);
        list1.push(5);
        assert_eq!(list1.pop(), Some(5));
        assert_eq!(list1.pop(), Some(4));
        assert_eq!(list1.pop(), Some(2));
        assert_eq!(list1.pop(), Some(1));
        assert_eq!(list1.pop(), None);
    }

    #[test]
    fn test_long_list() {
        let mut list = List::new();
        for i in 0..100000 {
            list.push(i);
        }
        // 如果不重写List的Drop，则会导致 STATUS_STACK_OVERFLOW，因为链表的drop是递归的
        drop(list);
    }
}
