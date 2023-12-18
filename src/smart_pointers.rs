#[cfg(test)]

mod tests {
    use std::ops::{Deref, DerefMut};

    #[test]
    fn test_box() {
        // 1. 使用 Box<T> 将数据存储在堆上；没有Box的情况下，i32肯定是存储在栈上的
        let a = Box::new(3);
        // a = 3，这里发生了自动解引用
        println!("a = {}", a);
        // 下面一行代码将报错，类型不匹配
        // assert_eq!(a, 3);

        // 2. 使用Box避免栈上数据的拷贝
        // 在栈上创建一个长度为1000的数组
        let arr = [0; 1000];
        // 将arr所有权转移arr1，由于 `arr` 分配在栈上，因此这里实际上是直接重新深拷贝了一份数据
        let arr1 = arr;

        // arr 和 arr1 都拥有各自的栈上数组，因此不会报错
        println!("{:?}", arr.len());
        println!("{:?}", arr1.len());

        // 在堆上创建一个长度为1000的数组，然后使用一个智能指针指向它
        let arr = Box::new([0; 1000]);
        println!("{:?}", arr.as_ptr());
        // 将堆上数组的所有权转移给 arr1，由于数据在堆上，因此仅仅拷贝了智能指针的结构体，底层数据并没有被拷贝
        // 所有权顺利转移给 arr1，arr 不再拥有所有权
        let arr1 = arr;
        println!("{:?}", arr1.len());
        // 由于 arr 不再拥有底层数组的所有权，因此下面代码将报错；但是如果上面是 let arr1 = arr.clone(); 则下面这行不会报错
        // println!("{:?}", arr.len());
        let arr2 = arr1.clone();
        println!("{:?}", arr2.len());
        assert_eq!(arr1.len(), arr2.len());
        println!("{:?}", arr1.as_ptr());
        println!("{:?}", arr2.as_ptr());
        assert_ne!(arr1.as_ptr(), arr2.as_ptr());

        // 3. 将DST转换为sized
        // enum List {
        //     Cons(i32, Box<List>),
        //     Nil,
        // }
        // 这里如果使用Cons(i32, List), 编译时无法知道List这个类型的大小，因为它是递归的，理论上可以一直嵌套下去

        // 4. 使用特征对象结合Box，使一个数组包含不同类型的数据
        trait Draw {
            fn draw(&self);
        }

        struct Button {
            id: u32,
        }
        impl Draw for Button {
            fn draw(&self) {
                println!("这是屏幕上第{}号按钮", self.id)
            }
        }

        struct Select {
            id: u32,
        }

        impl Draw for Select {
            fn draw(&self) {
                println!("这个选择框贼难用{}", self.id)
            }
        }

        let elems: Vec<Box<dyn Draw>> =
            vec![Box::new(Button { id: 1 }), Box::new(Select { id: 2 })];
        // 只能使用 & 借用数组中的元素，否则会报所有权错误，因为Box将数据存储在堆上
        let (_e1, _e2) = (&elems[0], &elems[1]);

        for e in elems {
            e.draw()
        }
    }

    // 实现自己的智能指针
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(t: T) -> MyBox<T> {
            MyBox(t)
        }
    }

    // 为智能指针实现 Deref 特征
    impl<T> Deref for MyBox<T> {
        type Target = T;

        // deref 返回的是一个常规引用，可以被 * 进行解引用
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    fn display(s: &str) {
        println!("{}", s);
    }

    impl<T> DerefMut for MyBox<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    fn display2(s: &mut String) {
        s.push_str("world");
        println!("{s}");
    }

    #[test]
    fn test_deref() {
        let x = MyBox::new(5);
        assert_eq!(*x, 5);

        // Deref 可以支持连续的隐式转换，下面代码调用display时发生了两次自动解引用
        // 当 T: Deref<Target=U>，可以将 &T 转换成 &U
        let s = MyBox::new(String::from("hello world"));
        display(&s);

        // 赋值操作需要手动解引用
        let s1: &str = &s;
        assert_eq!(s1, "hello world");
        // 方法调用会自动解引用，这里MyBox并没有to_string方法，s.to_string()实际上是对MyBox应用了Deref后调用的String的方法
        let s2: String = s.to_string();
        assert_eq!(s2, String::from("hello world"));

        // 当 T: DerefMut<Target=U>，可以将 &mut T 转换成 &mut U
        let mut s = MyBox::new(String::from("hello, "));
        display2(&mut s);

        // 当 T: Deref<Target=U>，可以将 &mut T 转换成 &U
        display(&s);
    }
}
