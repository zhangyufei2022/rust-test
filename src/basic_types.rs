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
}
