#[cfg(test)]

mod tests {
    #[test]
    fn test_scope() {
        let x = define_x();
        println!("{}, world", x);
        assert_eq!(x, "hello");
    }

    fn define_x() -> String {
        let x = "hello".to_string();
        x
    }

    #[test]
    fn test_shadowing() {
        let x: i32 = 5;
        {
            let x = 12;
            assert_eq!(x, 12);
        }

        assert_eq!(x, 5);

        let x = 42;
        println!("{}", x);
        assert_eq!(x, 42);
    }

    #[test]
    fn test_destructuring() {
        let (x, y);
        (x, ..) = (3, 4);
        [.., y] = [1, 2];
        assert_eq!([x, y], [3, 2]);
    }
}
