#[cfg(test)]

mod tests {

    fn production_rate_per_hour(speed: u8) -> f64 {
        let cph: u8 = 22;
        match speed {
            1..=4 => (speed * cph) as f64,
            5..=8 => (speed * cph) as f64 * 0.9,
            9..=10 => (speed * cph) as f64 * 0.77,
            _ => 0 as f64,
        }
    }

    fn divide(x: u8, y: u8) {
        println!("{}", x / y)
    }

    #[test]
    fn test_panic() {
        assert_eq!("abc".as_bytes(), [97, 98, 99]);

        let v = vec![1, 2, 3];
        // If index is over 2, it will panic
        let _ele = v[2];
        // get returns Option<T>, unwrap may panic when get return a None
        let _ele = v.get(2).unwrap();
        // Sometimes, the compiler is unable to find the overflow errors for you in compile time ,so a panic will occur
        let _v = production_rate_per_hour(2);
        // because of the same reason as above
        divide(15, 3);
    }

    use std::num::ParseIntError;

    fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
        let n1 = n1_str.parse::<i32>();
        let n2 = n2_str.parse::<i32>();
        Ok(n1.unwrap() * n2.unwrap())
    }
    #[test]
    fn test_unwrap() {
        let result = multiply("10", "2");
        assert_eq!(result, Ok(20));

        let result = multiply("4", "2");
        assert_eq!(result.unwrap(), 8);
    }

    fn multiply2(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
        let n1 = n1_str.parse::<i32>()?;
        let n2 = n2_str.parse::<i32>()?;
        Ok(n1 * n2)
    }

    #[test]
    fn test_return_err() {
        assert_eq!(multiply2("3", "4").unwrap(), 12);
    }

    // 使用 map 和 and_then 做错误处理
    // and(): 若两个表达式的结果都是 Some 或 Ok，则第二个表达式中的值被返回。若任何一个的结果是 None 或 Err ，则立刻返回。
    // and_then(): 类似于and()，但第二个表达式是一个闭包。
    // map()：把 Some 或 Ok 的值映射成另一个，如果是 Err，不改变
    fn multiply3(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
        n1_str
            .parse::<i32>()
            .and_then(|n1| n2_str.parse::<i32>().map(|n2| n1 * n2))
    }

    #[test]
    fn test_map_and_then() {
        assert_eq!(multiply3("3", "4").unwrap(), 12);
    }
}
