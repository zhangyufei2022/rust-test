#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    async fn hello_world() {
        println!("Hello world");
    }

    #[test]
    fn test_future() {
        // 返回一个Future, 此时不会打印任何输出，因为函数还没有执行
        let future = hello_world();
        // 执行Future并等待其运行完成，此时会有输出
        block_on(future);
    }
}
