type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    value: T,
    next: Link<T>,
}

pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        let node = Box::new(Node {
            value,
            next: self.head.take(), // take(): Takes the value out of the option, leaving a [`None`] in its place.
        });
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(node) => {
                self.head = node.next;
                Some(node.value)
            }
            None => None,
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur = self.head.take();
        while let Some(mut node) = cur {
            cur = node.next.take();
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
