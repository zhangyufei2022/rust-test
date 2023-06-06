#[derive(Debug)]
struct SelfRef {
    value: String,
    // 使用裸指针，而不是引用，因此不用受引用规则和生命周期的限制，但是取值的时候需要使用unsafe代码
    pointer: *mut String,
}

impl SelfRef {
    fn new(value: String) -> Self {
        SelfRef {
            value: value,
            pointer: std::ptr::null_mut(),
        }
    }

    fn init(&mut self) {
        self.pointer = &mut self.value;
    }

    fn value(&self) -> &str {
        &self.value
    }

    fn pointer(&self) -> &String {
        assert!(!self.pointer.is_null(), "Should call init() first!");
        unsafe { &*self.pointer }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_ref() {
        let mut a = SelfRef::new(String::from("Hello "));
        assert!(a.pointer.is_null());

        a.init();
        assert!(!a.pointer.is_null());

        a.value.push_str("world");
        // 裸指针取值时需要用unsafe代码
        (unsafe { &mut *a.pointer }).push('!');
        assert_eq!(a.value(), "Hello world!");
        println!("pointer:{:p}", a.pointer());
    }
}
