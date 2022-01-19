# CountDownLatch
A synchronization tool that allows one or more threads to wait until a set of operations being performed in other threads completes.

A CountDownLatch is initialized with a count. The await method blocks until the current count reaches zero due to invocations of the count_down() method, after which all waiting threads are released and any subsequent invocations of await return immediately.

A CountDownLatch is thread-safe. Multiple threads can invoke `await()` to wait for multiple threads to `count_down()`.

## Example
```
use countdown_latch::CountDownLatch;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    // create a CountDownLatch with count=5
    let latch = Arc::new(CountDownLatch::new(5));

    // create 5 threads that sleep for a variable amount of time before calling latch.count_down()
    for i in 0..5 {
        let tlatch = Arc::clone(&latch);
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(i * 100));
            println!("unlatching {}", i);
            tlatch.count_down();
        });
    }

    // await completion of the latch
    latch.await();

    // print done, which will appear in the console after all "unlatching" messages
    println!("done");
}
```
