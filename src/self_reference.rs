use std::{marker::PhantomPinned, pin::Pin};

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

#[derive(Debug)]
struct SelfRef2 {
    value: String,
    pointer: *const String,
    _marker: PhantomPinned, // 这个标记可以让类型自动实现 !UnPin 特征
}

impl SelfRef2 {
    fn new(value: &str) -> Pin<Box<Self>> {
        let a = SelfRef2 {
            value: value.to_string(),
            pointer: std::ptr::null(),
            _marker: PhantomPinned,
        };

        let mut boxed = Box::pin(a);
        let self_ptr: *const String = &boxed.value;
        unsafe { boxed.as_mut().get_unchecked_mut().pointer = self_ptr };
        boxed
    }

    fn value(self: Pin<&Self>) -> &str {
        &self.get_ref().value
    }

    fn pointer(self: Pin<&Self>) -> &String {
        unsafe { &*(self.get_ref().pointer) }
    }
}

#[cfg(test)]
mod tests {
    use std::ptr::NonNull;

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

    #[test]
    fn test_change_mem() {
        let mut test1 = SelfRef::new("test1".to_string());
        test1.init();
        let mut test2 = SelfRef::new("test2".to_string());
        test2.init();

        println!("value: {}, pointer: {}", test1.value(), test1.pointer());
        assert_eq!(test1.value(), "test1");
        assert_eq!(test1.pointer(), "test1");

        std::mem::swap(&mut test1, &mut test2);

        test1.value = "I've totally changed now!".to_string();
        println!(
            "test1 value: {}, pointer: {}",
            test1.value(),
            test1.pointer()
        );
        println!(
            "test2 value: {}, pointer: {}",
            test2.value(),
            test2.pointer()
        );
        assert_eq!(test2.value(), "test1");
        assert_eq!(test2.pointer(), "I've totally changed now!");
        assert_eq!(test1.value(), "I've totally changed now!");
        assert_eq!(test1.pointer(), "test1");
    }

    #[test]
    fn test_pin() {
        let test1 = SelfRef2::new("test1");
        println!(
            "test1 value: {}, pointer: {}",
            test1.as_ref().value(),
            test1.as_ref().pointer()
        );

        // 因为自定义类型 SelfRef2 实现了 !UnPin 特征，所以以下代码编译不过
        // let mut test2 = SelfRef2::new("test2");
        // std::mem::swap(&mut *test1, &mut *test2);
    }
}
