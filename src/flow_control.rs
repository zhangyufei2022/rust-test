#[cfg(test)]

mod tests {
    #[test]
    fn test_break() {
        let mut n = 0;
        // 0..100 不包含100；0..=100包含100
        for _i in 0..=100 {
            if n == 66 {
                break;
            }
            n += 1;
        }

        assert_eq!(n, 66);

        n = 0;
        // loop是表达式，其值可以用于对变量进行赋值
        let result = loop {
            n += 1;
            if n == 10 {
                break n * 2; // break 可以在结束循环的同时带回一个返回值
            }
        };
        assert_eq!(result, 20);
    }

    #[test]
    fn test_double_loops() {
        let mut count = 0;
        'outer: loop {
            'inner1: loop {
                if count >= 20 {
                    // This would break only the inner1 loop
                    break 'inner1;
                }
                count += 2;
            }

            count += 5;

            loop {
                if count >= 30 {
                    // This breaks the outer loop
                    break 'outer;
                }

                // This will continue the outer loop
                continue 'outer;
            }
        }

        assert!(count == 30);
    }
}
