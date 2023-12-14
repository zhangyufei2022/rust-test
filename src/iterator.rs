#[cfg(test)]

mod tests {
    use std::{
        slice::{Iter, IterMut},
        vec::IntoIter,
    };

    #[test]
    fn test_vector() {
        let values: Vec<i32> = vec![1, 2, 3];

        for v in values.into_iter() {
            println!("{}", v)
        }

        // 下面的代码将报错，因为 values 的所有权在上面 `for` 循环中已经被转移走
        // println!("{:?}",values);

        let values: Vec<i32> = vec![1, 2, 3];
        let _values_iter: Iter<'_, i32> = values.iter();
        // 不会报错，因为 values_iter 只是借用了 values 中的元素
        println!("{:?}", values);

        let mut values = vec![1, 2, 3];
        // 对 values 中的元素进行可变借用
        let mut values_iter_mut: IterMut<'_, i32> = values.iter_mut();

        // 取出第一个元素，并修改为0
        if let Some(v) = values_iter_mut.next() {
            *v = 0;
        }

        // 输出[0, 2, 3]
        println!("{:?}", values);
        assert_eq!(values, vec![0, 2, 3]);
    }

    #[test]
    fn test_next() {
        let v1 = vec![1, 2];

        // into_ 一类的方法通常是转移所有权
        let mut v1_iter: IntoIter<i32> = v1.into_iter();

        assert_eq!(v1_iter.next(), Some(1));
        assert_eq!(v1_iter.next(), Some(2));
        assert_eq!(v1_iter.next(), None);

        let v1 = vec![1, 2];

        // iter（） 方法是不可变借用，可变借用是iter_mut（）
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), None);
    }

    struct Fibonacci {
        curr: u32,
        next: u32,
    }

    impl Fibonacci {
        fn new() -> Self {
            Fibonacci { curr: 0, next: 1 }
        }
    }

    impl Iterator for Fibonacci {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            let next_new = self.curr + self.next;

            self.curr = self.next;
            self.next = next_new;

            Some(self.curr)
        }
    }

    #[test]
    fn test_my_iterator() {
        let mut fib = Fibonacci::new();
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(2));
        assert_eq!(fib.next(), Some(3));
        assert_eq!(fib.next(), Some(5));
    }

    use std::collections::HashMap;
    #[test]
    fn test_consume() {
        let names = [("sunface", 18), ("sunfei", 18)];
        let folks: HashMap<_, _> = names.into_iter().collect();
        let res = HashMap::from([("sunface", 18), ("sunfei", 18)]);
        assert_eq!(folks, res);
        println!("{:?}", folks);

        let v1: Vec<i32> = vec![1, 2, 3];
        // 这里不能使用iter()方法，因为它是借用，v2的类型会被推导为Vec<&i32>，到之后无法使用assert_eq判断是否相等
        let v2: Vec<_> = v1.into_iter().collect();
        assert_eq!(v2, vec![1, 2, 3]);

        let v1: Vec<i32> = vec![1, 2, 3];
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4]);
    }
}
