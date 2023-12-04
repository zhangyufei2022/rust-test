#[cfg(test)]

mod tests {
    use core::num;

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

    #[test]
    fn test_tuple() {
        let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
        assert_eq!(t.3, "hello");

        let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
        println!("too long tuple: {:?}", too_long_tuple); // 12以上长度打不出来

        let tup = (1, 6.4, "hello");

        // 结构元组
        let (x, z, y) = tup;
        assert_eq!(x, 1);
        assert_eq!(y, "hello");
        assert_eq!(z, 6.4);
    }

    // #[derive(Debug)] 表示使用derive派生实现debug特征，这样才能使用 {:?} 的方式对其进行打印输出
    #[derive(Debug)]
    struct File {
        name: String,
        data: Vec<u8>,
    }

    #[test]
    fn test_struct() {
        let f1 = File {
            name: dbg!(String::from("f1.txt")), // dbg!可以在打印信息的同时，返回表达式的值
            data: Vec::new(),
        };

        let f1_name = &f1.name;
        let f1_length = &f1.data.len();
        println!("{:?}", f1);
        println!("{} is {} bytes long", f1_name, f1_length);

        let name = "zhangsan".to_string();
        let data = vec![1, 2, 3];
        let file1 = build_file(name, data);
        assert_eq!(file1.data, vec![1, 2, 3]);

        let file2 = File {
            name: String::from("lisi"),
            ..file1 // 基于一个架构体实例构造另一个，这里会把file1.data所有权转给file2
        };
        assert_eq!(file2.data, vec![1, 2, 3]);

        let File { ref name, ref data } = file2; // 如果不用ref，则是把file2.name的所有权转移给name，后续不能访问file2
        assert_eq!(*name, file2.name);
        assert_eq!(*data, file2.data);
        println!("{:#?}", file2);
    }

    fn build_file(name: String, data: Vec<u8>) -> File {
        File { name, data }
    }

    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    #[test]
    fn test_enum() {
        let msg = Message::Move { x: 1, y: 1 };
        // 枚举成员变量的值可以用模式匹配获取
        if let Message::Move { x: a, y: b } = msg {
            assert_eq!(a, b);
        } else {
            panic!("不要让这行代码运行！");
        }

        let msgs: [Message; 3] = [
            Message::Quit,
            Message::Write("write sth".to_string()),
            Message::ChangeColor(255, 255, 0),
        ];
        println!("msgs: {:?}", msgs);

        // rust没有null，而是通过枚举Option<T>来处理空值
        let num = Some(1);
        let two = plus_one(num);
        let none = plus_one(None);
        assert_eq!(two, Some(2));
        assert_eq!(none, None);
    }

    fn plus_one(x: Option<isize>) -> Option<isize> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    #[test]
    fn test_array() {
        let names = [String::from("Sunfei"), "Sunface".to_string()];

        // get返回的是Option<T>
        let name0: &String = names.get(0).unwrap();
        assert_eq!(*name0, String::from("Sunfei"));

        // 直接使用下标访问有越界的风险
        let name1: &String = &names[1];
        assert_eq!(*name1, String::from("Sunface"));

        // 数组切片
        let sub: &[String] = &names[0..1];
        assert_eq!(*sub, [String::from("Sunfei")])
    }
}
