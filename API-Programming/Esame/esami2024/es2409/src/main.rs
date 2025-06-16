use std::sync::{Condvar, Mutex, Arc};
use std::time::Duration;

#[derive(PartialEq, Eq, Debug)]
pub enum WaitResult{
    Success,
    Timeout,
    Cancelled,
}
struct Inner {
    cancelled: bool,
    count: usize,
}

struct CancellableLatch {
    inner: Mutex<Inner>,
    cvar: Condvar,
}

impl CancellableLatch {
    fn new(count: usize) -> CancellableLatch {
        Self{
            inner: Mutex::new(
                Inner{
                    cancelled: false,
                    count
                }
            ),
            cvar: Condvar::new()
        }
    }

    fn count_down(&self){
        let mut lock = self.inner.lock().expect("Error during count down");
        if lock.cancelled {return;}

        if lock.count > 0{
            lock.count -= 1;
        }
        if lock.count == 0{
            self.cvar.notify_all();
        }
    }

    fn cancel(&self){
        let mut lock = self.inner.lock().expect("Error during cancel");

        lock.cancelled = true;
        self.cvar.notify_all();
    }

    fn wait(&self) -> WaitResult{
        let mut lock = self.inner.lock().expect("Error during wait");

        while !lock.cancelled && lock.count > 0 {
            lock = self.cvar.wait(lock).unwrap();
        }

        match lock.cancelled {
            true => { WaitResult::Cancelled },
            false => { WaitResult::Success }
        }
    }

    fn wait_timeout(&self, d: Duration) -> WaitResult{
        let mut lock = self.inner.lock().expect("Error during wait");
        let mut result;

        while !lock.cancelled && lock.count > 0 {
            result = self.cvar.wait_timeout(lock, d).unwrap();

            if result.1.timed_out() {
                return WaitResult::Timeout;
            }
            lock = result.0;
        }

        match lock.cancelled {
            true => { WaitResult::Cancelled },
            false => { WaitResult::Success }
        }

    }
}

pub mod test {
    use std::thread::{scope, sleep};
    use super::*;

    #[test]
    fn test_latch_wait(){
        let num_proc: usize = 5;

        let latch = Arc::new(CancellableLatch::new(num_proc));

        let _ = scope(|s| {
            let waiting = latch.clone();
            s.spawn(move || {
                println!("waiting");
                let res = waiting.wait();
                println!("end of waiting");
                assert_eq!(res, WaitResult::Success);
            });

            for _ in 0..num_proc {
                let working = latch.clone();
                s.spawn(move || {
                    sleep(Duration::from_millis(1));
                    working.count_down();
                });
            }
        });

    }

    #[test]
    fn test_cancel(){
        let num_proc: usize = 5;

        let latch = Arc::new(CancellableLatch::new(num_proc));

        let _ = scope(|s| {
            let waiting = latch.clone();
            s.spawn(move || {
                println!("waiting");
                let res = waiting.wait();
                println!("end of waiting");
                assert_eq!(res, WaitResult::Cancelled);
            });

            for i in 0..num_proc {
                let working = latch.clone();
                s.spawn(move || {
                    sleep(Duration::from_millis(100));
                    if i == 2 {
                        working.cancel();
                    } else {
                        working.count_down();
                    }

                });
            }
        });
    }

    #[test]
    fn test_wait_timeout_ok(){
        let num_proc: usize = 5;

        let latch = Arc::new(CancellableLatch::new(num_proc));

        let _ = scope(|s| {
            let waiting = latch.clone();
            s.spawn(move || {
                println!("waiting");
                let res = waiting.wait_timeout(Duration::from_secs(10));
                println!("end of waiting");
                assert_eq!(res, WaitResult::Success);
            });

            for i in 0..num_proc {
                let working = latch.clone();
                s.spawn(move || {
                    sleep(Duration::from_millis(100));
                    working.count_down();
                });
            }
        });
    }


    #[test]
    fn test_wait_timeout_timeout(){
        let num_proc: usize = 10;

        let latch = Arc::new(CancellableLatch::new(num_proc));

        let _ = scope(|s| {
            let waiting = latch.clone();
            s.spawn(move || {
                println!("waiting");
                let res = waiting.wait_timeout(Duration::from_millis(2));
                println!("end of waiting");
                assert_eq!(res, WaitResult::Timeout);
            });

            for i in 0..num_proc {
                let working = latch.clone();
                s.spawn(move || {
                    sleep(Duration::from_millis(100));
                    working.count_down();
                });
            }
        });
    }
}

fn main() {
    println!("Hello, world!");
}
