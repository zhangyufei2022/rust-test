#[cfg(test)]

mod tests {
    use std::ops::Add;

    #[derive(Debug, Clone, Copy)]
    struct Point<T: Add<Output = T>> {
        //特征约束：限制类型T必须实现了Add特征
        x: T,
        y: T,
    }

    // 为自定义类型实现Add特征
    impl<T: Add<Output = T>> Add for Point<T> {
        type Output = Point<T>;

        fn add(self, rhs: Point<T>) -> Self::Output {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    // 定义一个特征
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    // 为特定类型实现此特征
    impl Summary for Point<i32> {
        fn summarize(&self) -> String {
            format!("Point{{x: {}, y: {}}}", self.x, self.y)
        }
    }

    // 特征作为函数参数
    fn summarize(p: &impl Summary) -> String {
        p.summarize()
    }

    #[test]
    // 为自定义类型实现Add特征，然后就可以进行+操作了
    fn test_impl() {
        let p1 = Point {
            x: 1.1f32,
            y: 1.1f32,
        };
        let p2 = Point {
            x: 2.1f32,
            y: 2.1f32,
        };
        let p12 = p1 + p2;
        println!("{:?}", p12);
        assert!((p1.x + p2.x - p12.x).abs() < 0.0001);
        assert!((p1.y + p2.y - p12.y).abs() < 0.0001);

        let p3 = Point { x: 1i32, y: 1i32 };
        let p4 = Point { x: 2i32, y: 2i32 };
        let p34 = p3 + p4;
        println!("{:?}", p34);
        assert_eq!(p3.x + p4.x, p34.x);
        assert_eq!(p3.y + p4.y, p34.y);
    }

    #[test]
    fn test_trait_as_para() {
        let p1 = Point { x: 3, y: 4 };
        let s = summarize(&p1);
        assert_eq!(s, String::from("Point{x: 3, y: 4}"));
    }
}
