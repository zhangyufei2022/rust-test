#[cfg(test)]

mod tests {
    use std::ops::Add;

    #[derive(Debug, PartialEq)]
    struct Point<T: Add<Output = T>> {
        //特征约束：限制类型T必须实现了Add特征
        x: T,
        y: T,
    }

    // 为自定义类型实现Add特征
    // Add特征定义如下：pub trait Add<Rhs = Self> 使用了默认泛型类型参数，即不指定类型的情况下，使用相同类型进行相加运算
    // 也可以写为：impl<T: Add<Output = T>> Add<Self> for Point<T> 或 impl<T: Add<Output = T>> Add<Ponit<T>> for Point<T>
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

    // 也可以为泛型实现特征，使用特征约束有条件地实现方法或特征
    impl<T> Summary for Point<T>
    where
        T: Add<Output = T> + std::fmt::Display,
    {
        fn summarize(&self) -> String {
            format!("Point{{x: {}, y: {}}}", self.x, self.y)
        }
    }

    // // 为特定类型实现此特征
    // impl Summary for Point<i32> {
    //     fn summarize(&self) -> String {
    //         format!("Point{{x: {}, y: {}}}", self.x, self.y)
    //     }
    // }

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
        let mut s = summarize(&p1);
        assert_eq!(s, String::from("Point{x: 3, y: 4}"));

        let p2 = Point {
            x: 3.1_f32,
            y: 4.1_f32,
        };
        s = summarize(&p2);
        println!("{}", s);
        assert_eq!(s, String::from("Point{x: 3.1, y: 4.1}"));
    }

    // 函数返回中的 impl Trait
    fn returns_summarizable() -> impl Summary {
        Point { x: 1, y: 2 }
    }

    #[test]
    fn test_return_impl_trait() {
        let p = returns_summarizable();
        assert_eq!(p.summarize(), String::from("Point{x: 1, y: 2}"));
    }

    // 特征对象
    trait MyTrait {
        fn f(&self) -> Box<dyn MyTrait>;
    }

    impl MyTrait for u32 {
        fn f(&self) -> Box<dyn MyTrait> {
            Box::new(42)
        }
    }

    impl MyTrait for String {
        fn f(&self) -> Box<dyn MyTrait> {
            Box::new(self.clone())
        }
    }

    fn my_function(x: Box<dyn MyTrait>) -> Box<dyn MyTrait> {
        x.f()
    }

    #[test]
    fn test_trait_object() {
        my_function(Box::new(13_u32));
        my_function(Box::new(String::from("abc")));
    }

    trait Pilot {
        fn fly(&self) -> String;
    }

    trait Wizard {
        fn fly(&self) -> String;
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) -> String {
            String::from("This is your captain speaking.")
        }
    }

    impl Wizard for Human {
        fn fly(&self) -> String {
            String::from("Up!")
        }
    }

    impl Human {
        fn fly(&self) -> String {
            String::from("*waving arms furiously*")
        }
    }

    #[test]
    fn test_call_method() {
        let person = Human;
        // 特征和类型上的方法同名，优先调用类型上的方法
        assert_eq!(Pilot::fly(&person), "This is your captain speaking.");
        assert_eq!(Wizard::fly(&person), "Up!");

        assert_eq!(person.fly(), "*waving arms furiously*");

        println!("Success!")
    }

    // 手动实现Copy特征，因为Copy特征依赖Clone特征，因此也必须实现Clone
    // Copy特征定义如下：pub trait Copy: Clone
    impl<T: Copy + Add<Output = T>> Copy for Point<T> {}
    impl<T: Clone + Add<Output = T>> Clone for Point<T> {
        fn clone(&self) -> Self {
            Self {
                x: self.x.clone(),
                y: self.y.clone(),
            }
        }
    }

    #[test]
    fn test_supertrait() {
        let p1 = Point { x: 1i32, y: 1i32 };
        let p2 = p1.clone();
        assert_eq!(p1, p2);
    }
}
