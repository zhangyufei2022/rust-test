#[cfg(test)]

mod tests {
    #[test]
    fn test_generics_fn() {
        assert_eq!(5, sum(2i8, 3i8));
        assert_eq!(50, sum(20, 30));
        assert_eq!(2.46, sum(1.23, 1.23));
    }

    fn sum<T: std::ops::Add<T, Output = T>>(x: T, y: T) -> T {
        x + y
    }

    #[derive(Debug)]
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, p: Point<V, W>) -> Point<T, W> {
            Point { x: self.x, y: p.y }
        }
    }

    #[test]
    fn test_generics_struct() {
        let p = Point { x: 5.0, y: 4 };
        println!("p:{:?}", p);

        let p1 = Point { x: 5, y: 10 };
        let p2 = Point {
            x: "Hello",
            y: '中',
        };
        let p3 = p1.mixup(p2);
        assert_eq!(p3.x, 5);
        assert_eq!(p3.y, '中');
    }

    #[test]
    fn test_const_generics() {
        let arr: [i32; 4] = [1, 2, 3, 4];
        print_array(arr);

        let arr: [f32; 2] = [3.0, 4.0];
        print_array(arr);
    }
    fn print_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
        println!("{:?}", arr);
    }
}
