use std::sync::Arc;
use crate::barrier::CyclicBarrier;

pub mod barrier {
    use std::sync::{Arc, Condvar, Mutex};

    struct Item {
        waiting: usize,
        total: usize,
        out_open: bool,
        in_open: bool,
    }
    pub struct CyclicBarrier {
        barrier: Mutex<Item>,
        external: Condvar,
        internal: Condvar,
    }

    impl CyclicBarrier {
        pub fn new(n_threads: usize) -> Self {
            Self{
                barrier: Mutex::new(Item{
                    waiting: 0,
                    total: n_threads,
                    out_open: true,
                    in_open: false,
                }),
                external: Condvar::new(),
                internal: Condvar::new(),
            }
        }

        pub fn wait(&self) {
            let mut barrier_guard = self.barrier
                .lock()
                .expect("Couldn't get lock on barrier");


            // if the external barrier is open, wait for it
            while ! barrier_guard.out_open {
                barrier_guard = self.external
                    .wait(barrier_guard)
                    .expect("Couldn't get barrier");
            }

            barrier_guard.waiting += 1;

            if barrier_guard.waiting < barrier_guard.total{

                // wait for the internal barrier to open
                while !barrier_guard.in_open {
                    barrier_guard =  self.internal
                        .wait(barrier_guard)
                        .expect("Couldn't get barrier");
                }

                 
            } else {
                // the last thread that get the lock close the outer barrier
                barrier_guard.out_open = false;
                // and notify all the waiting threads
                barrier_guard.in_open = true;
                self.internal.notify_all();
            }
            barrier_guard.waiting -= 1;
            if barrier_guard.waiting == 0 {
                // the last thread to release the lock open the outer barrier
                barrier_guard.in_open = false;
                barrier_guard.out_open = true;
                self.external.notify_all();
            }
        }
    }
}

fn main() {
    const NTHREADS: usize = 16;

    let abarrrier = Arc::new(CyclicBarrier::new(NTHREADS));
    let mut vt = Vec::new();
    for i in 0..NTHREADS {
        let cbarrier = abarrrier.clone();
        vt.push(std::thread::spawn(move || {
            for j in 0..10 {
                cbarrier.wait();
                println!("after barrier {} {}", i, j);
            }
        }));
    }
    for t in vt {
        t.join().unwrap();
    }
}
