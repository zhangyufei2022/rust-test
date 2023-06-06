#[cfg(test)]
mod tests {

    use std::cell::Cell;
    use std::sync::mpsc::{Receiver, Sender};
    use std::sync::{mpsc, Arc, Barrier, Condvar, Mutex};
    use std::thread;
    use thread_local::ThreadLocal;

    // 以下代码会导致cpu 100%，因为线程B中的循环无法结束
    // #[test]
    // fn test_thread() {
    //     // 创建一个线程A
    //     let new_thread = thread::spawn(move || {
    //         // 再创建一个线程B
    //         thread::spawn(move || loop {
    //             println!("I am a new thread.");
    //         })
    //     });

    //     // 等待新创建的线程执行完成
    //     new_thread.join().unwrap();
    //     println!("Child thread is finish!");

    //     // 睡眠一段时间，看子线程创建的子线程是否还在运行
    //     thread::sleep(Duration::from_millis(10));
    // }

    const THREAD_COUNT: usize = 6;
    // cargo test --package hello_world --bin hello_world -- thread::tests::test_barrier --exact --nocapture
    #[test]
    fn test_barrier() {
        let mut handles = Vec::with_capacity(THREAD_COUNT);
        let barrier = Arc::new(Barrier::new(THREAD_COUNT));

        for i in 0..THREAD_COUNT {
            let b = barrier.clone();
            let handle = thread::spawn(move || {
                println!("Thred {}:Before wait", i);
                // 线程屏障，保证所有的线程先执行到此处，然后再继续开始执行后面的代码
                // 注释掉这行则Before和After交替执行
                b.wait();
                println!("Thred {}:After wait", i);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap(); // 等待子线程结束
        }
    }

    #[test]
    fn test_thread_local() {
        let local = Arc::new(ThreadLocal::new());

        for i in 0..THREAD_COUNT {
            // thread-local库，允许每个线程持有值的独立拷贝
            let thread_local = local.clone();
            let handle = thread::spawn(move || {
                let cell = thread_local.get_or(|| Cell::new(0));
                cell.set(cell.get() + i);
            });
            handle.join().unwrap();
        }

        // 所有子线程结束后，可以使用iter_mut和into_iter方法迭代ThreadLocal对象中所有线程的线程局部变量值
        let local = Arc::try_unwrap(local).unwrap();
        let sum = local.into_iter().fold(0, |x, y| x + y.get());
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_condvar() {
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair2 = pair.clone();

        thread::spawn(move || {
            let (lock, condvar) = &*pair;
            let mut start = lock.lock().unwrap();
            println!("Changing value");
            // 子线程获取到锁，并修改其值为true，然后调用条件变量的notify_one或notify_all发送通知
            *start = true;
            condvar.notify_one();
        });

        let (lock, condvar) = &*pair2;
        let mut start = lock.lock().unwrap();
        while !*start {
            println!("Wait......");
            assert_eq!(*start, false);
            // wait方法释放锁并挂起主线程，直到次条件变量收到通知
            start = condvar.wait(start).unwrap();
        }
        println!("Vlaue changed");
        assert_eq!(*start, true);
    }

    #[test]
    fn test_msg_pass() {
        // 同步通道，缓存消息数目为1
        let (tx, rx) = mpsc::sync_channel(1);
        // 多发送者，单接收者
        for i in 0..THREAD_COUNT {
            let tx = tx.clone();
            thread::spawn(move || {
                tx.send(i).unwrap();
                println!("Finished sending");
            });
        }

        /* 注释下面的drop会导致for循环无法结束，原因是：
        通道关闭的条件是所有的发送者被drop或者所有的接收者被drop，此处使用for循环阻塞的从rx迭代器中接收消息，
        因此需要所有发送者都被drop才能结束循环，上面子线程中获取到的是tx的拷贝的所有权，所有子线程结束后所有tx的拷贝会被drop，
        但是tx自身并没有被drop，所以需要手动drop
        */
        drop(tx);
        for (i, received) in rx.iter().enumerate() {
            println!("{} received: {}", i, received);
        }
    }

    enum DataType {
        Int(i32),
        String(String),
    }
    #[test]
    fn test_msg_pass_multi_type() {
        let (tx, rx): (Sender<DataType>, Receiver<DataType>) = mpsc::channel();
        thread::spawn(move || {
            tx.send(DataType::Int(1)).unwrap();
            tx.send(DataType::String("1".to_string())).unwrap();
        });

        for received in rx {
            match received {
                DataType::Int(x) => assert_eq!(x, 1),
                DataType::String(x) => assert_eq!(x, "1".to_string()),
            }
        }
    }

    #[test]
    fn test_mutex() {
        let counter = Arc::new(Mutex::new(0));
        for _ in 0..THREAD_COUNT {
            let counter = counter.clone();
            thread::spawn(move || {
                let mut count = counter.lock().unwrap();
                *count += 1;
            })
            .join()
            .unwrap();
        }

        assert_eq!(*counter.lock().unwrap(), THREAD_COUNT);
    }
}
