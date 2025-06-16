/*
La struct DelayedExecutor permette di eseguire funzioni in modo asincrono, dopo un certo intervallo di tempo.
Essa offre tre metodi:
new() -> Self crea un nuovo DelayedExecutor
execute<F: FnOnce()+Send+'static>(f:F, delay: Duration) -> bool
se il DelayedExecutor è aperto, accoda la funzione f che dovrà essere eseguita non prima che sia
trascorso un intervallo pari a delay e restituisce true;
se invece il DelayedExecutor è chiuso, restituisce false.
close(drop_pending_tasks: bool) chiude il DelayedExecutor;
se drop_pending_tasks è true, le funzioni in attesa di essere eseguite vengono eliminate, altrimenti
vengono eseguite a tempo debito.
DelayedExecutor è thread-safe e può essere utilizzato da più thread contemporaneamente.
I task sottomessi al DelayedExecutor devono essere eseguiti in ordine di scadenza.
All'atto della distruzione di un DelayedExecutor, tutti i task in attesa sono eliminati, ma se è in corso
un'esecuzione questa viene portata a termine evitando di creare corse critiche.
 */
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::sync::{Arc, Condvar, Mutex};
use std::sync::mpsc::Sender;
use std::thread::{spawn, JoinHandle};
use std::time::{Duration, Instant};

struct Job<F: FnOnce() + Send + 'static>{
    j: F,
    i: Instant,
}

impl <F: FnOnce() + Send + 'static> PartialEq for Job<F>{
    fn eq(&self, other: &Self) -> bool {
        self.i.eq(&other.i)
    }
}

impl <F: FnOnce() + Send + 'static> Eq for Job<F>{}

impl<F: FnOnce() + Send + 'static> PartialOrd for Job<F>{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
        // al contrario perché la heap in rust è una "max heap" -> restituisce il valore maggiore,
        // noi vogliamo il contrario
        other.i.partial_cmp(&self.i)
}
}

impl<F: FnOnce() + Send + 'static> Ord for Job<F>{
    fn cmp(&self, other: &Self) -> Ordering{
        other.i.cmp(&self.i)
    }
}

struct Queue<F: FnOnce() + Send + 'static>{
    q: BinaryHeap<Job<F>>,
    worker: Option<JoinHandle<()>>,
    open: bool,
    stop: bool,
}

struct DelayedExecutor<F: FnOnce() + Send + 'static> {
    queue: Arc<(Mutex<Queue<F>>, Condvar)>,
}

impl <F: FnOnce() + Send + 'static> Clone for DelayedExecutor<F> {
    fn clone(&self) -> Self {
        Self { queue: self.queue.clone() }
    }
}

impl<F: FnOnce() + Send + 'static> DelayedExecutor<F> {
    fn new() -> Self{
        let cv = Condvar::new();

        let mut queue = Arc::new( (
            Mutex::new(
                Queue::<F>{
                    q: BinaryHeap::new(),
                    worker: None,
                    open: true,
                    stop: false,
                }) ,
            cv
            ));
        let queue_clone = queue.clone();
        let worker = spawn( move || {

            loop{
                let mut guard = queue_clone.0.lock().expect("Mutex poisoned");
                if guard.stop || !guard.open && guard.q.is_empty(){
                    if guard.stop {
                        guard.q.clear();
                    }
                    break
                }

                while guard.q.is_empty() && !guard.stop{
                    guard = queue_clone.1.wait(guard).unwrap();
                }
                let job = guard.q.pop().unwrap().j;
                drop(guard);
                job();
            }
        });
        queue.0.lock().expect("Mutex poisoned").worker = Some(worker);
        Self { queue }
    }

    fn execute(&self, f:F, delay: Duration) -> bool{
        let mut guard = self.queue.0.lock().expect("Mutex poisoned");
        if !guard.open {
            return false;
        }

        if guard.q.is_empty(){
            self.queue.1.notify_all();
        }
        guard.q.push( Job{j:f, i: (Instant::now() + delay)});
        true
    }

    fn close(&self, drop_pending_tasks: bool){
        let mut guard = self.queue.0.lock().expect("Mutex poisoned");
        guard.open = false;
        guard.stop = drop_pending_tasks;

        self.queue.1.notify_all();
    }


}

impl <F: FnOnce() + Send + 'static> Drop for DelayedExecutor<F>{
    fn drop(&mut self){
        let mut guard = self.queue.0.lock().expect("Mutex poisoned");
        if let Some(jh) = guard.worker.take() {
            drop(guard);
            jh.join().unwrap();
        }
        // quando l'executor viene droppato, prima si aspetta che il thread al suo interno finisca,
        // tutti gli altri campi verranno droppati in automatico (implementano drop)
    }
}

pub mod test{
    use std::thread::scope;
    use super::*;

    #[test]
    fn test_base () {
        let executor = DelayedExecutor::new();

        let _ = scope(move |s| {
            for i in 0..20 {
                let e_clone = executor.clone();
                s.spawn(move||{
                   e_clone.execute(||{}, Duration::from_millis(i*10));
                });
            }

            s.spawn(move ||{
                let e_clone = executor.clone();
                e_clone.close(false);
            });
        });
    }
}

fn main() {

}
