#[cfg(test)]

mod tests {
    #[test]
    fn test_match() {
        let boolean = true;

        // match是表达式，可以用于赋值
        let binary = match boolean {
            true => 1,
            false => 0,
        };

        assert_eq!(binary, 1);

        let msgs = [
            Message::Quit,
            Message::Move { x: 1, y: 3 },
            Message::ChangeColor(255, 255, 0),
            Message::Write("write nothing".to_string()),
        ];

        for msg in msgs {
            show_message(msg);
        }

        // matches!宏
        let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];
        for ab in alphabets {
            assert!(matches!(ab, 'a'..='z'|'A'..='Z'|'0'..='9'))
        }
    }

    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    fn show_message(msg: Message) {
        match msg {
            // 从模式中取出绑定的值
            Message::Move { x: a, y: b } => {
                assert_eq!(a, 1);
                assert_eq!(b, 3);
                println!("Message::Move{{a:{}, b:{}}}", a, b);
            }
            Message::ChangeColor(_, g, b) => {
                assert_eq!(g, 255);
                assert_eq!(b, 0);
                println!("Message::ChangeColor(g:{}, b:{})", g, b);
            }
            Message::Write(ref s) => {
                println!("Message::Write({})", *s);
            }
            _ => {
                println!("Message::Quit");
            }
        }
    }

    #[test]
    fn test_if_let() {
        let num = Some(5);
        if let Some(n) = num {
            assert_eq!(n, 5);
        }
    }

    #[test]
    fn test_shadowing() {
        let age = Some(30);
        if let Some(age) = age {
            // age发生了变量遮蔽，匹配出来的age是整数，不是Option枚举
            assert_eq!(age, 30);
        } // 整数age的作用域结束
        assert_eq!(age, Some(30));

        match age {
            // match也会发生变量遮蔽
            Some(age) => {
                println!("age is a new variable, it's value is {}", age);
                assert_eq!(age, 30);
            }
            _ => (),
        }
        // 为避免变量遮蔽，最好是采用不同的变量名
    }
}
