#[cfg(test)]
mod tests {
    use crate::CountDownLatch;
    use std::sync::Arc;
    use std::thread;
    use std::time::{Duration, Instant};

    #[test]
    fn one_thread_await() {
        let latch = Arc::new(CountDownLatch::new(5));
        assert_eq!(latch.get_count(), 5);

        for i in 0..5 {
            let latch = Arc::clone(&latch);
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(i * 100));
                println!("unlatch {}", i);
                latch.count_down();
            });
        }

        let timeout = Instant::now() + Duration::from_secs(2);
        while Instant::now() < timeout && latch.get_count() > 0 {
            thread::sleep(Duration::from_millis(10));
        }
        assert_eq!(latch.get_count(), 0);

        latch.await();
    }

    #[test]
    fn multi_thread_await() {
        let delayed_latch = Arc::new(CountDownLatch::new(5));
        for i in 0..5 {
            let delayed_latch = Arc::clone(&delayed_latch);
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(i * 100));
                println!("delayed unlatch {}", i);
                delayed_latch.count_down();
            });
        }

        let awaited_latch = Arc::new(CountDownLatch::new(3));
        for i in 0..3 {
            let delayed_latch = Arc::clone(&delayed_latch);
            let awaited_latch = Arc::clone(&awaited_latch);
            thread::spawn(move || {
                delayed_latch.await();
                println!("awaited unlatch {}", i);
                awaited_latch.count_down();
            });
        }

        awaited_latch.await();
        assert_eq!(delayed_latch.get_count(), 0);
        assert_eq!(awaited_latch.get_count(), 0);
    }
}
