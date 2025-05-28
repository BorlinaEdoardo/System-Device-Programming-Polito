use std::{thread::{self, sleep}, time::Duration};

use latch::CountDownLatch;



mod latch{
    use std::sync::{Arc, Condvar, Mutex};

    pub enum LatchError{
        Error(String)
    }

    pub struct CountDownLatch{
        count: Arc<(Mutex<usize>, Condvar)>
    }

    impl Clone for CountDownLatch{

        fn clone(&self) -> Self {
            Self { count: self.count.clone() }
        }
    }

    impl CountDownLatch {

        pub fn new(n: usize) -> Self {
            Self { count: Arc::new((Mutex::new(n), Condvar::new())) }
        }

        // wait zero aspetta al massimo timeout ms

        // se esce per timeout ritorna Err altrimenti Ok

        pub fn wait_zero(&self, timeout: Option<std::time::Duration>) -> Result<(),LatchError>{
            let (count_mutex, cv) = &*self.count.clone();

            let mut c = count_mutex.lock().unwrap();

            while *c > 0 {
                let res = cv.wait_timeout(c, timeout.unwrap()).unwrap();
                if res.1.timed_out() {
                    return Err(LatchError::Error("Timed out".to_string()));
                }
                c = res.0;
            }



            return Ok(());

        }

        pub fn count_down(&self) {
            let (count_mutex, cv) = &*self.count.clone();
            let mut count = count_mutex.lock().unwrap();
            if *count > 0{
                *count = *count - 1
            } else if *count == 0 {
                cv.notify_all();
            }
        }

    }
}

pub fn doSomeWork(msg: &str){
    sleep(Duration::from_millis(50));
    println!("{}", msg);
}

pub fn demo_latch() {

    let mut handles = vec![];

    let mut wait_driver = CountDownLatch::new(1);
    let mut latch = CountDownLatch::new(10);

    for _ in 0..10 {
        let latch_thread = wait_driver.clone();
        let latch_signal = latch.clone();

        let h = thread::spawn(move || {

            latch_thread.wait_zero(Some(Duration::from_secs(3)));
            doSomeWork("(2) lavoro che necessita driver");
            latch_signal.count_down();

            doSomeWork("(3) altro lavoro che non necessita driver");

        });

        handles.push(h);

    }

    doSomeWork("(1) prepapara il driver");
    wait_driver.clone().count_down();

    // wait for the threads
    latch.wait_zero(Some(Duration::from_secs(3)));
    doSomeWork("(4) rilascia il driver");

    for h in handles {

        let _ = h.join();

    }

}

fn main(){
    demo_latch();
}