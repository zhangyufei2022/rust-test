/// 实现一个不可变的、共享所有权的持久化链表
use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    // 而之前的 push 和 pop 已无任何意义，因为新链表是不可变的，但我们可以使用功能相似的 prepend 和 tail 来返回新的链表。
    pub fn prepend(&self, value: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                value,
                next: self.head.clone(),
            })),
        }
    }

    // 该方法会将现有链表的首个元素移除，并返回剩余的链表
    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    // pub fn into_iter(self) -> IntoIter<T> {
    //     IntoIter(self)
    // }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        let mut next = None;
        if let Some(ref mut node) = self.head {
            next = Rc::get_mut(node);
        }
        IterMut { next }
    }
}

// // Rc（引用计数）类型是用于共享所有权的智能指针，因此不直接支持所有权转移
// pub struct IntoIter<T>(List<T>);

// impl<T> Iterator for IntoIter<T> {
//     type Item = T;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.head.take().and_then(|rc_node| {
//             if Rc::strong_count(&rc_node) > 0 {
//                 // let mut node = Rc::unwrap_or_clone(rc_node);
//                 // self.0.head = node.next.take();
//                 // Some(node.value)
//                 self.0.head = rc_node.next.take();
//                 Some(rc_node.value)
//             } else {
//                 None
//             }
//         })
//     }
// }

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.value
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            if let Some(ref mut rc_node) = node.next {
                self.next = Rc::get_mut(rc_node);
            }
            &mut node.value
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    pub fn test_basic() {
        let list = List::new();
        assert_eq!(list.peek(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.peek(), Some(&3));

        let list = list.tail();
        assert_eq!(list.peek(), Some(&2));

        let list = list.tail();
        assert_eq!(list.peek(), Some(&1));

        let list = list.tail();
        assert_eq!(list.peek(), None);

        let list = list.tail();
        assert_eq!(list.peek(), None);
    }

    // // todo: into_iter的写法有问题
    // #[test]
    // fn test_into_iter() {
    //     let mut list = List::new();
    //     assert_eq!(list.peek(), None);
    //     list = list.prepend(4).prepend(5).prepend(6);
    //     let list2 = list.prepend(7).prepend(8).prepend(9);

    //     let mut iter = list.into_iter();
    //     assert_eq!(iter.next(), Some(6));
    //     assert_eq!(iter.next(), Some(5));
    //     assert_eq!(iter.next(), Some(4));
    //     assert_eq!(iter.next(), None);

    //     let mut iter = list2.into_iter();
    //     assert_eq!(iter.next(), Some(9));
    //     assert_eq!(iter.next(), Some(8));
    //     assert_eq!(iter.next(), Some(7));
    //     assert_eq!(iter.next(), Some(6));
    //     assert_eq!(iter.next(), Some(5));
    //     assert_eq!(iter.next(), Some(4));
    //     assert_eq!(iter.next(), None);
    // }

    #[test]
    fn test_iter() {
        let list = List::new().prepend(2).prepend(3);
        let list2 = list.prepend(1);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(list.peek(), Some(&3));

        let mut iter2 = list2.iter();
        assert_eq!(iter2.next(), Some(&1));
        assert_eq!(iter2.next(), Some(&3));
        assert_eq!(iter2.next(), Some(&2));
        assert_eq!(list2.peek(), Some(&1));
    }

    #[test]
    fn test_iter_mut() {
        let mut list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter_mut();
        iter.next().map(|value| *value = 33);
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(list.peek(), Some(&33));
        println!("list: {:?}", list);
    }
}
