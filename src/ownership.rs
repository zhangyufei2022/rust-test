#[cfg(test)]

mod tests {
    #[test]
    fn test_copy() {
        let x: &str = "hello, world";
        let y = x; // 只拷贝了引用
        assert_eq!(x, y);

        let x = String::from("hello, world");
        // 深拷贝，在堆上完整拷贝了x的内容
        // 改为 let y = x; 则x的所有权被转移给y，后续不能再访问x
        let y = x.clone();
        assert_eq!(x, y);
    }

    #[test]
    fn test_move_ownership() {
        let s1 = String::from("hello, world");
        let s2 = take_ownership(s1);
        assert_eq!(s2, String::from("hello, world"));

        let s3 = give_ownership();
        assert_eq!(s3, String::from("hello, world"));
    }

    // 入参 s 的所有权被转移给了返回值
    fn take_ownership(s: String) -> String {
        println!("{}", s);
        s
    }

    fn give_ownership() -> String {
        let s = String::from("hello, world");
        // Convert String to Vec
        let _s = s.as_bytes(); // into_bytes 会发生所有权转移；as_bytes 不会，因为入参是引用类型
        s
    }

    #[test]
    fn test_partial_move() {
        let t = (String::from("hello"), String::from("world"));
        let _s = t.0;
        assert_eq!("world", t.1);

        let t = (String::from("hello"), String::from("world"));
        let (ref s1, ref s2) = t;
        println!("{:?}, {:?}, {:?}", s1, s2, t);
        assert_eq!(t, (String::from("hello"), String::from("world")));
    }
}
