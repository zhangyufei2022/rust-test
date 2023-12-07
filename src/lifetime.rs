#[cfg(test)]

mod tests {
    // 没有生命周期标注不能通过编译，因为编译器无法得知返回值的生命周期
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // `print_refs` 有两个引用参数，它们的生命周期 `'a` 和 `'b` 至少得跟函数活得一样久
    fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("x is {} and y is {}", x, y);
    }

    fn failed_borrow<'a>() {
        let _x = 12;

        // ERROR: `_x` 活得不够久does not live long enough
        // let y: &'a i32 = &_x;
        // 在函数内使用 `'a` 将会报错，原因是 `&_x` 的生命周期显然比 `'a` 要小
        // 你不能将一个小的生命周期强转成大的
        let _y: &i32 = &_x;
    }

    #[test]
    fn test_lifetime_in_fn() {
        let a = "asd";
        let b = "asdfg";
        let res = longest(&a, &b);
        assert_eq!(res, b);

        let (four, nine) = (4, 9);
        print_refs(&four, &nine);
        // 这里，four 和 nice 的生命周期必须要比函数 print_refs 长

        failed_borrow();
        // `failed_borrow`  没有传入任何引用去限制生命周期 `'a`，因此，此时的 `'a` 生命周期是没有任何限制的，它默认是 `'static`
    }

    // `i32` 的引用必须比 `Borrowed` 活得更久
    #[derive(Debug)]
    struct Borrowed<'a>(&'a i32);

    // 类似的，下面两个引用也必须比结构体 `NamedBorrowed` 活得更久
    #[derive(Debug)]
    struct NamedBorrowed<'a> {
        x: &'a i32,
        y: &'a i32,
    }

    #[derive(Debug)]
    enum Either<'a> {
        Num(i32),
        Ref(&'a i32),
    }

    #[test]
    fn test_lifetime_in_stuct() {
        let x = 18;
        let y = 15;

        let single = Borrowed(&x);
        let double = NamedBorrowed { x: &x, y: &y };
        assert_eq!(*double.x, 18);
        assert_eq!(*double.y, 15);
        let reference = Either::Ref(&x);
        let number = Either::Num(y);

        println!("x is borrowed in {:?}", single);
        println!("x and y are borrowed in {:?}", double);
        println!("x is borrowed in {:?}", reference);
        println!("y is *not* borrowed in {:?}", number);
    }

    use std::{slice::from_raw_parts, str::from_utf8_unchecked};

    fn get_memory_location() -> (usize, usize) {
        // “Hello World” 是字符串字面量，因此它的生命周期是 `'static`.
        // 但持有它的变量 `string` 的生命周期就不一样了，它完全取决于变量作用域，对于该例子来说，也就是当前的函数范围
        let string = "Hello World!";
        let pointer = string.as_ptr() as usize;
        let length = string.len();
        (pointer, length)
        // `string` 在这里被 drop 释放
        // 虽然变量被释放，无法再被访问，但是数据依然还会继续存活
    }

    fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
        // 使用裸指针需要 `unsafe{}` 语句块
        unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
    }

    #[test]
    fn test_static_lifetime() {
        let (pointer, length) = get_memory_location();
        let message = get_str_at_location(pointer, length);
        println!(
            "The {} bytes at 0x{:X} stored: {}",
            length, pointer, message
        );
        assert_eq!(message, "Hello World!");
        /*
        上面代码有两点值得注意：
        1. &'static 的引用确实可以和程序活得一样久，因为我们通过 get_str_at_location 函数直接取到了对应的字符串
        2. 持有 &'static 引用的变量，它的生命周期受到作用域的限制
        */
    }

    #[derive(Debug, Clone)]
    struct Config {
        a: String,
        b: String,
    }

    // 声明一个全局变量，并且初始化为None，后续在运行中给它赋新值
    static mut CONFIG: Option<&mut Config> = None;

    // 这里需要返回一个具有'static生命周期的Config
    fn init() -> Option<&'static mut Config> {
        let config = Box::new(Config {
            a: "A".to_string(),
            b: "B".to_string(),
        });
        // Box::leak，它可以消费掉Box并且强制目标值从内存中泄漏，然后将其变为'static生命周期
        Some(Box::leak(config))
    }

    #[test]
    fn test_static_with_box() {
        unsafe {
            CONFIG = init();

            println!("{:?}", CONFIG);
            assert_eq!(CONFIG.is_some(), true);

            let config = CONFIG.as_deref().unwrap();
            assert_eq!(config.a, "A".to_string());
            assert_eq!(config.b, "B".to_string());
        }
    }
}
