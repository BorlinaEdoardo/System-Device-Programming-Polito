use std::sync::{mpsc, Arc, Mutex};
use std::thread::JoinHandle;

type Job = Box<dyn FnOnce() + Send + 'static>;


/// Impl 1 con i canali
// i thread della pool esistono solo fintanto che la pool esiste
struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

struct Worker {
    id: usize, // usiamo un id nostro, diverso da quello del SO
    handle: JoinHandle<()>
}

impl ThreadPool {
    // prepara un vettore con tanti thread, che ancora non sanno cosa devono fare, ed un canale
    // questi thread devono leggere dal canale, ma ciò è difficile perché i canali standard sono single receiver
    // => devo fare na robba complicata
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(size);

        // Il ricevitore lo ficco dentro un arc e dentro un mutex
        // così posso clonarlo
        let rx = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            let rx = Arc::clone(&rx);
            let w = Worker{
                id,
                handle: std::thread::spawn(move || {
                    if let Ok(job) = rx.lock().unwrap().recv() {
                        job;
                    }
                })
            };
            workers.push(w);
        };

        Self {
            workers,
            sender
        }
    }

    fn execute(&self, job: Job){

        for worker in &self.workers {
            let _ = worker.handle.join();
        }
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        let (tx, rx) = mpsc::channel::<Job>();
        self.sender = tx;
        let mut workers: Vec<Job> = vec![];
        std::mem::swap(&mut self.workers, &mut self.workers);

        for worker in workers.iter_mut() {
            worker.join().unwrap();
        }
    }
}

fn main() {

}
