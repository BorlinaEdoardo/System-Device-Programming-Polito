/*

tempo: from 16:46 to ho perso il conto ma non troppo, dai
 */
use std::sync::{Arc, Condvar, Mutex, WaitTimeoutResult};
use std::time::Duration;

pub struct CountDownLock{
    countdown: Mutex<usize>,
    cv: Condvar
}

impl CountDownLock{
    pub fn new(n: usize)->Self{
        Self{
            countdown: Mutex::new(n),
            cv: Condvar::new()
        }
    }

    pub fn count_down(&self){
        let mut mut_guard = self.countdown.lock().expect("error during lock in count_down");

        if *mut_guard > 0 {
            *mut_guard -= 1;
        } else if *mut_guard == 0 {
            self.cv.notify_all();
        }

    }

    pub fn wait(&self){
        let mut mut_guard = self.countdown.lock().expect("error during lock in wait");

        while *mut_guard > 0 {
            mut_guard = self.cv.wait(mut_guard).expect("error during wait");
        }
    }

    pub fn wait_timeout(&self, d: Duration) -> std::sync::WaitTimeoutResult{
        let mut mut_guard = self.countdown.lock().expect("error during lock in wait_timout");
        let mut res;

        (mut_guard, res) = self.cv.wait_timeout(mut_guard, d).unwrap();

        res
    }

}



fn main() {
    println!("Hello, world!");
}
