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

        for (i, c) in s1.chars().enumerate() {
            println!("{}: {}", i, c);
        }

        // 事实上 String 是一个智能指针，它作为一个结构体存储在栈上，然后指向存储在堆上的字符串底层数据。
        // 存储在栈上的智能指针结构体由三部分组成：一个指针只指向堆上的字节数组，已使用的长度以及已分配的容量 capacity
        // (已使用的长度小于等于已分配的容量，当容量不够时，会重新分配内存空间)。
        let mut s = String::with_capacity(20);
        assert_eq!(20, s.capacity());
        for _ in 0..2 {
            s.push_str("hello");
            assert_eq!(20, s.capacity());
        }
    }

    #[test]
    fn test_slice() {
        let arr = [1, 2, 3];
        let s1: &[i32] = &arr[0..2];
        assert_eq!(s1, [1, 2]);

        let s2: &str = "hello, world";
        assert_eq!(s2, "hello, world");

        let arr: [char; 3] = ['中', '国', '人'];
        let slice = &arr[..2];
        // 切片和数组不一样，它是引用，占用两个字大小的空间。
        assert_eq!(std::mem::size_of_val(&slice), 16);

        // char类型是Unicode编码，大小固定为4字节，两个字符为8字节
        assert_eq!(std::mem::size_of_val(&arr), 12);

        let s = "你好，世界";
        // 字符串是utf8编码，单个字符占1-4个字节，其中中文字符占3字节。
        let slice = &s[0..3];
        assert!(slice == "你");

        let s = "hello".to_string();
        let v = vec![104, 101, 108, 108, 111];
        // 将字节数组转换成 String
        let s1 = String::from_utf8(v).unwrap();
        assert_eq!(s, s1);
    }

    #[test]
    fn test_mem() {
        let story = String::from("Rust By Practice");

        // 阻止 String 的数据被自动 drop
        let mut story = std::mem::ManuallyDrop::new(story);

        let ptr = story.as_mut_ptr();
        let len = story.len();
        let capacity = story.capacity();

        assert_eq!(16, len);

        // 我们可以基于 ptr 指针、长度和容量来重新构建 String.
        // 这种操作必须标记为 unsafe，因为我们需要自己来确保这里的操作是安全的
        let s = unsafe { String::from_raw_parts(ptr, len, capacity) };
        assert_eq!(*story, s);
    }
}
