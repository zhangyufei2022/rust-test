#[cfg(test)]

mod tests {
    #[test]
    fn test_string() {
        let mut s = String::from("");
        s.push_str("hello, worl");
        s.push('d');
        assert_eq!(s, "hello, world");

        s += "!"; // + 的右边必须是字符串切片
        assert_eq!(s, "hello, world!");

        let s1 = String::from("hello,");
        let s2 = String::from("world!");
        let s3 = s1.clone() + &s2;
        assert_eq!(s3, "hello,world!");
        println!("{}", s1);

        let s1 = String::from("hi,中国");
        let h = &s1[0..1]; // `h` only takes 1 byte in UTF8 format
        assert_eq!(h, "h");

        let h1 = &s1[3..6]; // `中` takes 3 bytes in UTF8 format
        assert_eq!(h1, "中");

        // 以 Unicode 字符的方式遍历字符串
        for c in s1.chars() {
            println!("{}", c)
        }
    }
}
