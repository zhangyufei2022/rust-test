#[cfg(test)]

mod tests {
    #[test]
    fn test_position_param() {
        assert_eq!(format!("{}{}", 1, 2), "12");
        assert_eq!(
            format!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob"),
            "Alice, this is Bob. Bob, this is Alice"
        );
        assert_eq!(format!("{1}{}{0}{}", 1, 2), "2112");
    }

    #[test]
    fn test_named_param() {
        assert_eq!(format!("{argument}", argument = "test"), "test");

        assert_eq!(format!("{name}{}", 1, name = 2), "21");
        assert_eq!(format!("{a} {c} {b}", a = "a", b = 'b', c = 3), "a 3 b");

        // 具名参数必须放在其它参数后面
        assert_eq!(format!("{abc} {0}", 2, abc = "def"), "def 2");
    }

    #[test]
    fn test_string_align() {
        // 下面两个都是通过 5 个空格来填充
        assert_eq!(format!("Hello {:5}!", "x"), "Hello x    !");
        assert_eq!(format!("Hello {:1$}!", "x", 5), "Hello x    !");
        assert_eq!(format!("Hello {1:0$}!", 5, "x"), "Hello x    !");
        assert_eq!(format!("Hello {0:width$}!", "x", width = 5), "Hello x    !");

        // 左对齐
        assert_eq!(format!("Hello {:<5}!", "x"), "Hello x    !");
        // 右对齐
        assert_eq!(format!("Hello {:>5}!", "x"), "Hello     x!");
        // 居中对齐
        assert_eq!(format!("Hello {:^5}!", "x"), "Hello   x  !");

        // 左对齐，并使用 `&` 填充
        assert_eq!(format!("Hello {:&<5}!", "x"), "Hello x&&&&!");
    }

    #[test]
    fn test_float() {
        let v = 3.1415926;

        println!("{:.1$}", v, 4); // same as {:.4} => 3.1416

        assert_eq!(format!("{:.2}", v), "3.14");
        assert_eq!(format!("{:+.2}", v), "+3.14");
        assert_eq!(format!("{:.0}", v), "3");
    }

    #[test]
    fn main() {
        let s = "Hello, world!";

        assert_eq!(format!("{0:.5}", s), "Hello");
        assert_eq!(format!("Hello {:.*}!", 3, "abcdefg"), "Hello abc!");
    }

    #[test]
    fn test_numbers() {
        assert_eq!(format!("{:#b}", 27), "0b11011");
        assert_eq!(format!("{:#o}", 27), "0o33");
        assert_eq!(format!("{:#x}", 27), "0x1b");
        assert_eq!(format!("{:#X}", 27), "0x1B");

        println!("{:x}!", 27); // 没有前缀的十六进制 => 1b

        println!("{:#010b}", 27); // 使用 0 来填充二进制，宽度为 10 => 0b00011011

        println!("{:2e}", 1000000000); // => 1e9
        println!("{:2E}", 1000000000); // => 1E9
    }

    fn get_person() -> String {
        String::from("sunface")
    }

    #[test]
    fn test_get_value() {
        let person = get_person();
        assert_eq!(format!("Hello, {person}"), "Hello, sunface");
    }
}
