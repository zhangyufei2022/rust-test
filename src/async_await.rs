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

    async fn hello_cat() {
        /*
        在async fn函数中使用.await可以等待另一个异步调用的完成。
        但是与block_on不同，.await并不会阻塞当前的线程，而是异步的等待Future A的完成，
        在等待的过程中，该线程还可以继续执行其它的Future B，最终实现了并发处理的效果。
        */
        hello_world().await;
        println!("Hello kitty");
    }

    #[test]
    fn test_await() {
        let future = hello_cat();
        block_on(future);
    }

    struct Song {
        author: String,
        name: String,
    }

    async fn learn_song() -> Song {
        Song {
            author: "周杰伦".to_string(),
            name: String::from("《菊花台》"),
        }
    }

    async fn sing_song(song: Song) {
        println!(
            "给大家献上一首{}的{} ~ {}",
            song.author, song.name, "菊花残，满地伤~ ~"
        );
    }

    async fn learn_and_sing() {
        // 这里使用 .await 来等待学歌的完成，但是并不会阻塞当前线程，该线程在学歌的任务 .await 后，完全可以去执行跳舞的任务
        let song = learn_song().await;

        // 唱歌得在学歌之后
        sing_song(song).await;
    }

    async fn dance() {
        println!("唱到情深处，身体不由自主的动了起来~ ~");
    }

    async fn async_main() {
        // join! 可以并发的处理和等待多个 Future ，若其中一个 Future 被阻塞，那另一个可以拿过线程的所有权继续执行。
        // 若所有Future都被阻塞，就会让出线程所有权，并将其交给 main 函数中的 block_on 执行器
        let future1 = learn_and_sing();
        let future2 = dance();
        futures::join!(future1, future2);
    }

    #[tokio::test]
    async fn test_async_await() {
        block_on(async_main());
    }
}
