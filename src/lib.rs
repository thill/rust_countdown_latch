//! A synchronization tool that allows one or more threads to wait until a set of operations being performed in other threads completes.
//!
//! A CountDownLatch is initialized with a count. The await method blocks until the current count reaches zero due to invocations of the count_down() method, after which all waiting threads are released and any subsequent invocations of await return immediately.
//!
//! # Example
//!
//! ```rust
//! use countdown_latch::CountDownLatch;
//! use std::sync::Arc;
//! use std::thread;
//! use std::time::Duration;
//!
//! fn main() {
//!     // create a CountDownLatch with count=5
//!     let latch = Arc::new(CountDownLatch::new(5));
//!     // create 5 threads that sleep for a variable amount of time before calling latch.count_down()
//!     for i in 0..5 {
//!         let tlatch = Arc::clone(&latch);
//!         thread::spawn(move || {
//!             thread::sleep(Duration::from_millis(i * 100));
//!             println!("unlatching {}", i);
//!             tlatch.count_down();
//!         });
//!     }
//!     // await completion of the latch
//!     latch.await();
//!     // print done, which will appear in the console after all "unlatching" messages
//!     println!("done");
//! }
//! ```

mod tests;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::Mutex;

/// A CountDownLatch is used to wait for a given number of tasks to be completed which may be running in multiple threads
pub struct CountDownLatch {
    remaining: AtomicUsize,
    tx: mpsc::SyncSender<()>,
    rx: Mutex<mpsc::Receiver<()>>,
}
impl CountDownLatch {
    /// Construct a CountDownLatch with the given count
    pub fn new(count: usize) -> Self {
        let (tx, rx) = mpsc::sync_channel(count);
        Self {
            remaining: AtomicUsize::new(count),
            tx: tx,
            rx: Mutex::new(rx),
        }
    }
    /// Decrement the count by one
    pub fn count_down(&self) {
        // single send on channel
        self.tx.send(()).unwrap();
    }
    /// Get the current count
    pub fn get_count(&self) -> usize {
        // try to drain channel
        let lock = self.rx.try_lock();
        if let Ok(rx) = lock {
            while self.remaining.load(Ordering::SeqCst) > 0 && rx.try_recv().is_ok() {
                self.remaining.fetch_sub(1, Ordering::SeqCst);
            }
        }
        // return remaining count
        return self.remaining.load(Ordering::SeqCst);
    }
    /// Block until the count reaches 0
    pub fn await(&self) {
        // get lock, indefinite wait
        let rx = self.rx.lock().unwrap();
        // while remaining > 0, receive on channel and decrement count
        while self.remaining.load(Ordering::SeqCst) > 0 {
            rx.recv().unwrap();
            self.remaining.fetch_sub(1, Ordering::SeqCst);
        }
    }
}
