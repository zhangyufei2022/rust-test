#[cfg(test)]

mod tests {
    use std::{fmt::Display, ops::Add};

    // 使用newtype避开孤儿原则，为外部类型实现外部特征
    struct Meters(u32);

    impl Display for Meters {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} meters", self.0)
        }
    }

    impl Add for Meters {
        type Output = Meters;

        fn add(self, rhs: Self) -> Self::Output {
            Meters(self.0 + rhs.0)
        }
    }

    #[test]
    fn test_newtype() {
        let d1 = Meters(10);
        assert_eq!(format!("{}", d1), "10 meters");
        let d2 = Meters(20);
        let d = d1 + d2;
        assert_eq!(format!("{}", d), "30 meters");

        let i: u32 = 2;
        assert_eq!(i.pow(2), 4);

        let _n = Meters(i);
        // 下面的代码将报错，因为`Meters`类型上没有`pow`方法，因为Meters是新类型而不是内部包裹的u32了。
        // assert_eq!(n.pow(2), 4);
    }

    enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add,
        Subtract,
    }

    // 类型别名不是新的类型，只是为了让可读性更好
    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

    impl Operations {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                Self::Add => x + y,
                Self::Subtract => x - y,
            }
        }
    }

    #[test]
    fn test_type_alias() {
        let add = Operations::Add;
        let mut res = add.run(5, 3);
        assert_eq!(res, 8);

        res = Operations::Subtract.run(5, 3);
        assert_eq!(res, 2);
    }

    fn my_function<const N: usize>() -> [u32; N] {
        [123; N]
    }

    #[test]
    fn test_sized() {
        let arr = my_function::<5>();
        println!("{:?}", arr);
        assert_eq!(arr, [123; 5]);
    }
}
