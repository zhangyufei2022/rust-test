#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};
    use List::{Cons, Nil};
    use List2::{Cons2, Nil2};

    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }

    #[test]
    fn test_circle_ref() {
        let a = Rc::new(Cons(1, RefCell::new(Rc::new(Nil))));
        println!("a的初始化rc计数 = {}", Rc::strong_count(&a));
        println!("a指向的节点 = {:?}", a.tail());

        // 创建b到a的引用
        let b = Rc::new(Cons(2, RefCell::new(Rc::clone(&a))));
        println!("创建b到a的引用后:");
        println!("a的初始化rc计数 = {}", Rc::strong_count(&a));
        println!("b的初始化rc计数 = {}", Rc::strong_count(&b));
        println!("b指向的节点 = {:?}", b.tail());
        assert_eq!(Rc::strong_count(&a), 2);
        assert_eq!(Rc::strong_count(&b), 1);

        // 利用RefCell的可变性创建a到b的引用
        if let Some(pointer) = a.tail() {
            *pointer.borrow_mut() = Rc::clone(&b);
        }
        println!("利用RefCell的可变性创建a到b的引用后:");
        println!("a的rc计数 = {}", Rc::strong_count(&a));
        println!("b的rc计数 = {}", Rc::strong_count(&b));
        assert_eq!(Rc::strong_count(&a), 2);
        assert_eq!(Rc::strong_count(&b), 2);

        // 下面一行println!将导致循环引用
        // 我们可怜的8MB大小的main线程栈空间将被它冲垮，最终造成栈溢出
        // println!("a next item = {:?}", a.tail());
    }

    #[test]
    fn test_weak() {
        let a = Rc::new(5);
        let weak_a: Weak<_> = Rc::downgrade(&a);
        let strong_a: Option<Rc<_>> = weak_a.upgrade();
        // Weak引用的内存还存在的时候，能取到值
        assert_eq!(*strong_a.unwrap(), 5);

        drop(a);
        let strong_a = weak_a.upgrade();
        // Weak引用的内存不存在了，返回None
        assert_eq!(strong_a, None);
    }

    #[derive(Debug)]
    enum List2 {
        Cons2(i32, RefCell<Weak<List2>>),
        Nil2,
    }

    impl List2 {
        fn tail(&self) -> Option<&RefCell<Weak<List2>>> {
            match self {
                Cons2(_, p) => Some(p),
                Nil2 => None,
            }
        }
    }

    // Weak指针可以解决循环引用问题
    // 使用方式简单总结下：对于父子引用关系，可以让父节点通过 Rc 来引用子节点，然后让子节点通过 Weak 来引用父节点。
    #[test]
    fn test_circle_ref_with_weak() {
        let a = Rc::new(Cons2(1, RefCell::new(Rc::downgrade(&Rc::new(Nil2)))));
        println!("a的初始化rc计数 = {}", Rc::strong_count(&a));
        println!("a指向的节点 = {:?}", a.tail());

        // 创建b到a的引用
        let b = Rc::new(Cons2(2, RefCell::new(Rc::downgrade(&a))));
        println!("创建b到a的引用后:");
        println!("a的初始化rc计数 = {}", Rc::strong_count(&a));
        println!("b的初始化rc计数 = {}", Rc::strong_count(&b));
        println!("b指向的节点 = {:?}", b.tail());
        // Weak不计数
        assert_eq!(Rc::strong_count(&a), 1);
        assert_eq!(Rc::strong_count(&b), 1);

        // 利用RefCell的可变性创建a到b的引用
        if let Some(pointer) = a.tail() {
            *pointer.borrow_mut() = Rc::downgrade(&b);
        }
        println!("利用RefCell的可变性创建a到b的引用后:");
        println!("a的rc计数 = {}", Rc::strong_count(&a));
        println!("b的rc计数 = {}", Rc::strong_count(&b));
        assert_eq!(Rc::strong_count(&a), 1);
        assert_eq!(Rc::strong_count(&b), 1);

        // 下面一行println!不会导致循环引用
        println!("a next item = {:?}", a.tail());
    }

    // tree 数据结构
    #[derive(Debug)]
    pub struct Node {
        pub value: i32,
        pub parent: RefCell<Weak<Node>>, // 子节点通过 Weak 来引用父节点
        pub children: RefCell<Vec<Rc<Node>>>, // 父节点通过 Rc 来引用子节点
    }

    #[test]
    fn test_tree() {
        let leaf_1 = Rc::new(Node {
            value: 1,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        let leaf_2 = Rc::new(Node {
            value: 2,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });
        assert_eq!(Rc::strong_count(&leaf_2), 1);
        assert_eq!(Rc::weak_count(&leaf_2), 0);

        {
            let root = Rc::new(Node {
                value: 2,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf_1)]),
            });

            *leaf_1.parent.borrow_mut() = Rc::downgrade(&root);
            println!("leaf_1 parent = {:?}", leaf_1.parent.borrow().upgrade());
            assert_eq!(Rc::strong_count(&leaf_1), 2);
            assert_eq!(Rc::weak_count(&leaf_1), 0);
            assert_eq!(Rc::strong_count(&root), 1);
            assert_eq!(Rc::weak_count(&root), 1);
        }

        println!("leaf_1 parent = {:?}", leaf_1.parent.borrow().upgrade());
        assert_eq!(Rc::strong_count(&leaf_1), 1);
        assert_eq!(Rc::weak_count(&leaf_1), 0);
    }
}
