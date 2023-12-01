#[cfg(test)]

mod tests {
    #[test]
    fn test_type() {
        // 如果我们没有显式的指定变量的类型，那编译器会自动帮我们推导
        let x = 5;
        assert_eq!("i32".to_string(), type_of(&x));

        let x = 5.001;
        assert_eq!("f64".to_string(), type_of(&x));
    }

    // 以下函数可以获取传入参数的类型，并返回类型的字符串形式，例如  "i8", "u8", "i32", "u32"
    fn type_of<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>())
    }

    #[test]
    fn test_float() {
        assert_eq!(0.1_f32 + 0.2_f32, 0.3_f32);
        println!(
            "(0.1_f64 + 0.2 - 0.3).abs() = {}",
            (0.1_f64 + 0.2 - 0.3).abs()
        );
        assert!((0.1_f64 + 0.2 - 0.3).abs() < 0.000001);
    }

    #[test]
    fn test_unit_type() {
        let x: () = ();
        assert_eq!(x, ret_unit_1());
        assert_eq!(x, ret_unit_2());

        let v = (2, 3);
        assert_eq!(v, ret_unit_3());

        // 单元类型，不占内存
        let unit: () = ();
        assert_eq!(std::mem::size_of_val(&unit), 0);
    }

    fn ret_unit_1() {
        println!("I will return a ()");
    }

    fn ret_unit_2() -> () {
        println!("I will return a ()");
    }

    fn ret_unit_3() -> (i32, i32) {
        println!("I will return a (2, 3)");
        (2, 3)
    }

    #[test]
    fn test_expression() {
        let v = {
            let mut x = 1;
            x += 2;
            x
        };
        assert_eq!(v, 3);

        let v = {
            let x = 1;
            x + 2
        };
        assert_eq!(v, 3);
    }
}
