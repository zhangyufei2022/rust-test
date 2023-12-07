#[cfg(test)]

mod tests {
    #[test]
    fn test_vector_methods() {
        let arr: [u8; 3] = [1, 2, 3];

        let v1 = Vec::from(arr);
        is_vec(&v1);

        let v2 = vec![1, 2, 3];
        is_vec(&v2);

        // vec!(..) 和 vec![..] 是同样的宏，宏可以使用 []、()、{}三种形式，因此...
        let v3 = vec![1, 2, 3];
        is_vec(&v3);
        assert_eq!(v1, v2);
        assert_eq!(v2, v3);

        let mut v1 = Vec::new();
        for i in &arr {
            v1.push(*i);
        }
        assert_eq!(v3, v1);
        // let v1 = vec!(arr); 此处 v1 是 Vec<[u8; 3]> , 而不是 Vec<u8>

        let mut v2 = Vec::new();
        v2.extend(&v1);
        assert_eq!(v1, v2);
    }

    fn is_vec(_v: &Vec<u8>) {}

    #[test]
    fn test_from_and_into() {
        // array -> Vec
        // impl From<[T; N]> for Vec
        let arr = [1, 2, 3];
        let v1 = Vec::from(arr);
        let v2: Vec<i32> = arr.into();

        assert_eq!(v1, v2);

        // String -> Vec
        // impl From<String> for Vec
        let s = "hello".to_string();
        let v1: Vec<u8> = s.into();

        let s = "hello".to_string();
        let v2 = s.into_bytes();
        assert_eq!(v1, v2);

        // impl<'_> From<&'_ str> for Vec
        let s = "hello";
        let v3 = Vec::from(s);
        assert_eq!(v2, v3);

        // 迭代器 Iterators 可以通过 collect 变成 Vec
        let v4: Vec<i32> = [0; 10].into_iter().collect();
        assert_eq!(v4, vec![0; 10]);

        println!("Success!")
    }

    #[test]
    fn test_vector_index() {
        let mut v = Vec::from([1, 2, 3]);
        for i in 0..3 {
            println!("{:?}", v[i])
        }

        // 使用下标访问vector可能越界，但是v.get()返回的是Option，是安全的访问方式
        for i in 0..5 {
            if let Some(x) = v.get(i) {
                v[i] = x + 1;
            } else {
                v.push(i + 2);
            }
        }

        assert_eq!(v, vec![2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_slice() {
        let mut v = vec![1, 2, 3];

        let slice1 = &v[..];
        let slice2 = &v[0..v.len()];
        assert_eq!(slice1, slice2);

        // 切片是只读的
        // 注意：切片和 `&Vec` 是不同的类型，后者仅仅是 `Vec` 的引用，并可以通过解引用直接获取 `Vec`
        let vec_ref: &mut Vec<i32> = &mut v;
        (*vec_ref).push(4);
        let slice3 = &mut v[0..];
        // slice3.push(4);

        assert_eq!(slice3, &[1, 2, 3, 4]);
    }

    #[test]
    fn test_capacity() {
        let mut vec = Vec::with_capacity(10);

        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), 10);

        // 由于提前设置了足够的容量，这里的循环不会造成任何内存分配...
        for i in 0..10 {
            vec.push(i);
        }
        assert_eq!(vec.len(), 10);
        assert_eq!(vec.capacity(), 10);

        // 但是下面的代码会造成新的内存分配
        vec.push(11);
        assert_eq!(vec.len(), 11);
        assert!(vec.capacity() >= 11);
        assert_eq!(vec.capacity(), 20);
        println!("{}", vec.capacity());
    }

    trait IpAddr {
        fn display(&self);
    }

    struct V4(String);
    impl IpAddr for V4 {
        fn display(&self) {
            println!("ipv4: {:?}", self.0)
        }
    }

    struct V6(String);
    impl IpAddr for V6 {
        fn display(&self) {
            println!("ipv6: {:?}", self.0)
        }
    }

    #[test]
    fn test_different_types() {
        let v: Vec<Box<dyn IpAddr>> = vec![
            Box::new(V4("127.0.0.1".to_string())),
            Box::new(V6("::1".to_string())),
        ];

        for ip in &v {
            (*ip).display();
        }
    }
}
