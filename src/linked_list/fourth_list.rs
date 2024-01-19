/// 双向链表
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

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

    pub fn push_back(&mut self, value: T) {
        let new_node = Node::new(value);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_node);
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

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().value
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        // map<U, F>(orig: Ref<'b, T>, f: F) -> Ref<'b, U>
        // where F: FnOnce(&T) -> &U,
        // U: ?Sized
        // 就像在 Result 和 Option 上使用 map 一样，我们还能在 Ref 上使用 map:
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.value))
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.value))
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        // map<U, F>(orig: Ref<'b, T>, f: F) -> Ref<'b, U>
        // where F: FnOnce(&T) -> &U,
        // U: ?Sized
        // 就像在 Result 和 Option 上使用 map 一样，我们还能在 Ref 上使用 map:
        self.head
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.value))
    }

    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.value))
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

        list.push_back(4);
        list.push_front(3);
        list.push_back(5);
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_front_mut().is_none());
        assert!(list.peek_back_mut().is_none());

        list.push_front(1);
        list.push_front(2);

        // 需要注意的是 Ref 不能被直接比较，因此我们需要先利用 Deref 解引用出其中的值，再进行比较。
        // assert_eq!(*list.peek_front().unwrap(), 2);   // 两种比较方式都可以
        assert_eq!(&*list.peek_front().unwrap(), &2);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 2);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);

        *list.peek_front_mut().unwrap() = 3;
        assert_eq!(list.pop_front(), Some(3));
    }
}
