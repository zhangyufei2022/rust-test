#[cfg(test)]

mod tests {
    // T: Fn(u32) -> u32表示T实现了相应的闭包特征
    struct Cacher<T: Fn(E) -> E, E: Copy> {
        query: T,
        value: Option<E>,
    }

    impl<T, E> Cacher<T, E>
    where
        T: Fn(E) -> E,
        E: Copy,
    {
        fn new(query: T) -> Self {
            Cacher { query, value: None }
        }

        fn get_value(&mut self, arg: E) -> E {
            // 有值直接返回，没有就调用query对应的函数得到一个值再返回
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.query)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    #[test]
    fn test_closure_trait() {
        let query = |x| x + 1;
        let mut cacher = Cacher::new(query);
        let mut value = cacher.get_value(1);
        assert_eq!(value, 2);

        value = cacher.get_value(2);
        assert_eq!(value, 2);

        let query2 = |x: u32| x + 1;
        let mut cacher2 = Cacher::new(query2);
        let mut value2 = cacher2.get_value(1);
        assert_eq!(value2, 2);

        value2 = cacher2.get_value(2);
        assert_eq!(value2, 2);
    }

    fn fn_once<F>(func: F) -> Vec<bool>
    where
        F: FnOnce(usize) -> bool + Copy, // 仅实现FnOnce的闭包会拿走捕获变量的所有权，同时实现了Copy特征的则不会拿走所有权
    {
        vec![func(3), func(4)]
    }

    #[test]
    fn test_fnonce() {
        let x = vec![String::from("1"), String::from("2"), String::from("3")];
        // 闭包自动实现Copy特征的规则是，只要闭包捕获的类型都实现了Copy特征的话，这个闭包就会默认实现Copy特征。
        // 因此下面这个闭包实现了Copy特征，这里取得的是x的不可变引用，所以是能Copy的。
        let func = |z| z == x.len();
        let result = fn_once(func);
        assert_eq!(result, vec![true, false]);
        // fn_once(move |z| z == x.len()); // 使用move强制闭包拿走捕获变量的所有权，这种情况下就不满足Copy特征的约束
    }

    // FnMut 以可变借用的方式捕获环境中的值
    fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
        f("hello")
    }

    #[test]
    fn test_fnmut() {
        let mut s = String::new();

        let update_string = |str| s.push_str(str);

        exec(update_string);

        assert_eq!(s, "hello")
    }

    // 使用Box可以实现返回不同的闭包
    fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
        let num = 5;

        if x > 5 {
            Box::new(move |x| x - num)
        } else {
            Box::new(move |x| x + num)
        }
    }

    #[test]
    fn test_return_closure() {
        let f = factory(1);
        let answer = f(1);
        assert_eq!(6, answer);

        let f = factory(8);
        let answer = f(1);
        assert_eq!(-4, answer);
    }

    /*
    实际上，一个闭包并不仅仅实现某一种 Fn 特征，规则如下：
    1.所有的闭包都自动实现了 FnOnce 特征，因此任何一个闭包都至少可以被调用一次
    2.没有移出所捕获变量的所有权的闭包自动实现了 FnMut 特征
    3.不需要对捕获变量进行改变的闭包自动实现了 Fn 特征
     */
    fn exec_fnonce<F: FnOnce()>(f: F) {
        f()
    }
    fn exec_fnonce_2<F: FnOnce() -> Box<i32>>(f: F) -> Box<i32> {
        f()
    }

    fn exec_fnmut<F: FnMut()>(mut f: F) {
        f()
    }

    fn exec_fn<F: Fn()>(f: F) {
        f()
    }

    #[test]
    fn test_fn_type() {
        let movable = Box::new(3);

        // 满足规则一、二、三
        let consume = || {
            println!("`movable`: {:?}", movable);
        };
        consume();
        consume();

        exec_fnonce(consume);
        exec_fnmut(consume);
        exec_fn(consume);

        let mut s = 3;
        // 满足规则一、二，但是改变了捕获变量movable的值，所以不满足规则三
        let mut update_value = || {
            s += 1;
            println!("`s`:{s}");
        };
        exec_fnonce(&mut update_value);
        exec_fnmut(&mut update_value);

        // 满足规则一，但是移出了捕获变量movable的所有权，所以不满足规则二
        let move_value = || {
            println!("`movable`: {:?}", movable);
            movable
        };
        assert_eq!(exec_fnonce_2(move_value), Box::new(3));
    }
}
