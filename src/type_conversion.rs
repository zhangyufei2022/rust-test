#[cfg(test)]

mod tests {
    #![allow(overflowing_literals)] // 避免数值溢出的编译错误，但是溢出还是会发生
    #[test]
    fn test_as() {
        let decimal = 97.123_f32;

        let integer: u8 = decimal as u8;
        // as 不具有传递性，decimal能转为u8，u8可以转为char，但是decimal不能直接转为char
        let c1 = decimal as u8 as char;
        let c2 = integer as char;
        assert_eq!(c1, 'a');
        assert_eq!(c2, 'a');
        assert_eq!(integer, 'b' as u8 - 1);

        assert_eq!(u8::MAX, 255);
        let v = 1000 as u8;
        assert_eq!(v, 232); // 发生溢出

        assert_eq!(1000 as u16, 1000);
        assert_eq!(-1_i8 as u8, 255);

        // 从 Rust 1.45 开始，当浮点数超出目标整数的范围时，转化会直接取正整数取值范围的最大或最小值
        assert_eq!(300.1_f32 as u8, 255);
        assert_eq!(-100.1_f32 as u8, 0);

        // 上面的浮点数转换有一点性能损耗，如果大家对于某段代码有极致的性能要求，
        // 可以考虑下面的方法，但是这些方法的结果可能会溢出并且返回一些无意义的值
        // 总之，请小心使用
        unsafe {
            // 300.0 is 44
            println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
            // -100.0 as u8 is 156
            println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
            // nan as u8 is 0
            println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
        }

        let arr: [u64; 13] = [0; 13];
        assert_eq!(std::mem::size_of_val(&arr), 8 * 13); // 返回数组的大小
        assert_eq!(std::mem::size_of_val(&&arr), 8); // 返回数组的引用的大小
        let a: *const [u64] = &arr;
        let b = a as *const [u8];
        unsafe { assert_eq!(std::mem::size_of_val(&*b), 13) }
    }

    #[test]
    fn test_from_into() {
        // impl From<bool> for i32
        let i1: i32 = false.into();
        let i2: i32 = i32::from(false);
        assert_eq!(i1, i2);
        assert_eq!(i1, 0);

        let i3: u32 = 'a'.into();
        assert_eq!(i3, 97);

        let s: String = 'a'.into();
        assert_eq!(s, String::from('a'));
    }

    // From 被包含在 `std::prelude` 中，因此我们没必要手动将其引入到当前作用域来
    // use std::convert::From;

    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        // 实现 `from` 方法
        fn from(value: i32) -> Self {
            Number { value }
        }
    }

    #[test]
    fn test_impl_from() {
        let num = Number::from(30);
        assert_eq!(num.value, 30);

        let num: Number = 30.into();
        assert_eq!(num.value, 30);
    }

    #[derive(Debug, PartialEq)]
    struct EvenNum(i32);

    impl TryFrom<i32> for EvenNum {
        type Error = String;

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNum(value))
            } else {
                Err(format!("{} is not even number.", value))
            }
        }
    }

    #[test]
    fn test_tryfrom() {
        assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
        assert_eq!(
            EvenNum::try_from(5),
            Err(String::from("5 is not even number."))
        );

        // 填空
        let result: Result<EvenNum, String> = 8i32.try_into();
        assert_eq!(result, Ok(EvenNum(8)));
        let result: Result<EvenNum, String> = 5i32.try_into();
        assert_eq!(result, Err(String::from("5 is not even number.")));
    }

    use std::fmt;

    struct Point {
        x: i32,
        y: i32,
    }

    // 实现 fmt::Display 特征之后，还会自动实现ToString
    impl fmt::Display for Point {
        // 实现 fmt 方法
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "The point is ({}, {})", self.x, self.y)
        }
    }

    #[test]
    fn test_display() {
        let origin = Point { x: 0, y: 0 };
        assert_eq!(origin.to_string(), "The point is (0, 0)");
        assert_eq!(format!("{}", origin), "The point is (0, 0)");
    }

    use std::str::FromStr;
    // 使用 parse 方法可以将一个 String 转换成 i32 数字，这是因为在标准库中为 i32 类型实现了 FromStr: : impl FromStr for i32
    #[test]
    fn test_fromstr() {
        let n1: i32 = "1".parse().unwrap();
        let n2 = "2".parse::<i32>().unwrap();
        let n3: i32 = i32::from_str("3").unwrap();
        let sum = n1 + n2 + n3;
        assert_eq!(sum, 6);
    }
}
