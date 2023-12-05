#[cfg(test)]

mod tests {
    #[derive(Debug)]
    struct TrafficLight {
        color: TrafficLightColor,
    }

    impl TrafficLight {
        // 关联函数实现
        pub fn new() -> Self {
            TrafficLight {
                color: TrafficLightColor::Red,
            }
        }

        // &self 其实是 self: &Self 的语法糖
        pub fn get_state(&self) -> String {
            self.color.color()
        }

        pub fn set_state(&mut self, color: TrafficLightColor) {
            self.color = color;
        }
    }

    #[allow(unused)]
    #[derive(Debug)]
    enum TrafficLightColor {
        Red,
        Yellow,
        Green,
    }

    // 为 TrafficLightColor 实现所需的方法
    impl TrafficLightColor {
        pub fn color(&self) -> String {
            match self {
                TrafficLightColor::Red => String::from("red"),
                TrafficLightColor::Yellow => String::from("yellow"),
                TrafficLightColor::Green => String::from("green"),
            }
        }
    }

    #[test]
    fn test_associate_function() {
        let light = TrafficLight::new();
        assert_eq!(light.get_state(), "red");
    }

    #[test]
    fn test_method() {
        let c = TrafficLightColor::Yellow;
        assert_eq!(c.color(), "yellow");

        let mut light = TrafficLight::new();
        light.set_state(TrafficLightColor::Green);
        assert_eq!(light.get_state(), "green");
    }
}
