/// 双向链表
use std::{cell::RefCell, rc::Rc};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    prev: Link<T>,
    next: Link<T>,
}

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value,
            prev: None,
            next: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Node::new(value);
        match self.head.take() {
            Some(old_head) => {
                // 下面两行不能交换，因为 new_node.borrow_mut().next = Some(old_head); 会拿走old_head的所有权
                // 再执行 old_head.borrow_mut() 就会报错
                // 同理，应该先用new_node.clone()再用new_node进行赋值
                old_head.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                // 原头节点的下一个节点为新的头节点，新头节点不为空的场景：
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                // 新头节点为空的场景，即弹出一个元素后链表为空：
                None => {
                    // self.head.take()前面已经执行过了，这里是需要处理self.tail即可
                    self.tail.take();
                }
            }
            // into_inner() 消费掉 RefCell 并返回内部的值
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().value
        })
    }
}

// 不实现Drop的情况下，默认的drop只是每次将引用计数减1，若存在循环引用则可能出现引用计数不能清零的情况
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    pub fn test_basics() {
        let mut list = List::new();
        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        list.push_front(2);
        assert_eq!(list.pop_front(), Some(2));

        list.push_front(3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_front(), None);
    }
}
