
/* 15 febbraio
La struttura generica Exchanger<T> permette a due thread di scambiarsi un valore di tipo T.
Essa offre esclusivamente il metodo pubblico fn exchange(&self, t: T) -> Option<T>, che
blocca il thread chiamante senza consumare CPU fino a che un altro thread non invoca lo
stesso metodo sulla stessa istanza. Quando questo avviene, il metodo restituisce l’oggetto
passato come parametro dal thread opposto.
Si implementi tale struttura, usando la libreria standard di Rust

tempo impiegato: 50 min :(
 */
pub mod exchange {
    use std;
    use std::sync::{Arc, Condvar, Mutex};
    use std::thread::scope;

    pub struct Exchange<T> {
        item: Arc<(Mutex<Option<T>>, Condvar)>,
    }

    impl<T> Clone for Exchange<T>{
        fn clone(&self) -> Self {
            Self{item: self.item.clone()}
        }
    }

    impl <T> Exchange<T>{
        pub fn new() -> Self{
            Self{ item: Arc::new((Mutex::new(None), Condvar::new())) }
        }

        pub fn exchange(&self, t: T) -> Option<T>{
            let e = self.clone().item;
            let (mut_e, cv) = &*e;
            let mut mutex_guard = mut_e.lock().expect("Mutex poisoned");
            if (*mutex_guard).is_some() {
                let res = (*mutex_guard).take();
                *mutex_guard = Option::Some(t);
                cv.notify_one();
                res
            } else {
                *mutex_guard = Some(t);
                loop {
                    mutex_guard = cv.wait(mutex_guard).expect("Mutex poisoned");
                    if (*mutex_guard).is_some() {
                        break;
                    }
                }
                let res = (*mutex_guard).take();
                res
            }
        }
    }

    #[test]
    fn test_exchange(){
        let exchange = Exchange::new();
        let scoper = scope(|scope| {
            for i in 0..=1 {
                let e = exchange.clone();
                scope.spawn(move ||{
                    let res = e.exchange(i);
                    println!("thread {i} : {:?}", res);
                });
            }

        });
    }
}

// 19 giugno 2021
/*
Si implementi una struttura SingleThreadExecutor<F: FnOnce() + Send + 'static> che realizzi il
concetto di ThreadPool basato su un singolo thread.
• Coda dei compiti: La struttura deve utilizzare una coda per accodare i compiti da
eseguire. I compiti vengono rappresentati come funzioni o chiusure che soddisfano i
tratti FnOnce() + Send.
• Metodo submit(...): Attraverso questo metodo, è possibile affidare un compito
all'istanza di SingleThreadExecutor. I compiti vengono accodati e saranno disponibili per
l'elaborazione. Se l'esecutore è stato chiuso, eventuali tentativi di invocare submit(...)
devono restituire un errore.
• Metodo close(): Questo metodo deve impedire l'ulteriore accodamento di compiti.
Dopo la chiamata a close(), eventuali invocazioni di submit(...) devono fallire con un
errore. Tuttavia, i compiti già accodati devono poter essere eseguiti.
Si implementi tale classe usando le funzionalità offerte dalla libreria standard di Rust,
definendo tutte le parti eventualmente mancanti nella definizione della classe. Si faccia
attenzione al fatto che il codice che può essere sottomesso all'esecutore è arbitrario e può
contenere richieste di sottomissione di ulteriori compiti allo stesso esecutore.
difficoltà: Canali, sparatemi
tempo :
 */
/*
pub mod single_thread_ex{
    struct SingleThreadExecutor<F: FnOnce() + Send + 'static> {

    }
    impl <F: FnOnce()> SingleThreadExecutor<F> {
        pub fn new() -> (Self, Receiver<Box<F>>);
        pub fn submit(&self, task: F) -> Result<(), String>;
        pub fn close(&mut self);
    }
}
*/

/* 5 luglio 2021
Si implementi in linguaggio Rust una struttura generica Buffer<T> che modelli una struttura dati
condivisa tra due thread concorrenti, uno produttore e uno consumatore. La struttura deve
consentire al produttore di inserire valori e notificare la terminazione della produzione, mentre
il consumatore può richiedere valori in modalità FIFO, rispettando le seguenti regole:
• Metodo next(...): Il thread produttore utilizza questo metodo per aggiungere un nuovo
valore al buffer. Se è stata invocata una chiamata a terminate() o fail(...), il metodo deve
fallire lanciando un'eccezione e il buffer deve rimanere inalterato.
• Metodo terminate(): Il thread produttore utilizza questo metodo per notificare che non
saranno disponibili ulteriori valori. Dopo questa chiamata, il buffer non accetta più
nuovi valori. Se non ci sono valori nel buffer, il metodo consume() deve restituire None.
• Metodo fail(...): Il thread produttore utilizza questo metodo per notificare un errore e
indicare che non saranno disponibili ulteriori valori. Dopo questa chiamata, il buffer non
accetta più nuovi valori. Se non ci sono valori nel buffer, il metodo consume() deve
restituire un errore.
• Metodo consume(): Il thread consumatore utilizza questo metodo per prelevare un
valore dal buffer in modalità FIFO. Se non ci sono valori disponibili, il metodo si blocca
in attesa di nuovi valori o di una condizione di terminazione, senza consumare cicli di
CPU. Se è stato invocato terminate() e non ci sono valori, restituisce None. Se è stato
invocato fail(...) e non ci sono valori, rilancia l'eccezione specificata.

difficoltà: severo ma giusto
tempo: 28 minuti senza test + 29 minuti di test + debug :(
 */

pub mod buffer{
    use std::any::Any;
    use std::collections::{vec_deque, BinaryHeap, VecDeque};
    use std::error::Error;
    use std::sync::{Arc, Condvar, Mutex};
    use std::task::Context;
    use std::thread;
    use std::thread::{sleep, Thread};
    use std::time::Duration;

    enum BufError{
        ERROR(String),
    }

    struct Inner<T>{
        buf: VecDeque<T>,
        closed: bool,
        error: Option<Box<dyn Any + Send>>
    }

    struct Buffer<T>{
        inner: Mutex<Inner<T>>,
        cvar: Condvar,
    }


    impl <T: Send + std::cmp::Ord> Buffer<T> {
        pub fn new() -> Self{
            Self{
                inner: Mutex::new(
                    Inner{
                        buf: VecDeque::new(),
                        closed: false,

                        error: None,
                    }
                ),
                cvar: Condvar::new(),
            }
        }
        // Metodi relativi alla produzione di valori
        pub fn next(&self, value: T){
            let mut mut_guard = self.inner.lock().expect("Mutex poisoned");

            if(mut_guard.closed){
                panic!("The channel is closed");
            }

            if mut_guard.buf.is_empty(){
                self.cvar.notify_one();
            }

            mut_guard.buf.push_back(value);
        }
        pub fn terminate(&self){
            let mut mut_guard = self.inner.lock().expect("Mutex poisoned");
            self.cvar.notify_all();
            mut_guard.closed = true
        }
        pub fn fail(&self, error: Box<dyn Any + Send>){
            let mut mut_guard = self.inner.lock().expect("Mutex poisoned");

            mut_guard.closed = true;
            mut_guard.error = Some(error);
        }
        // Metodo relativo al consumo di valori
        pub fn consume(&self) -> Result<Option<T>, Box<dyn Any + Send>>{
            let mut mut_guard = self.inner.lock().expect("Mutex poisoned");
            while(mut_guard.buf.is_empty()){
                mut_guard = self.cvar.wait(mut_guard).expect("Mutex poisoned");
            }

            if mut_guard.closed && mut_guard.buf.is_empty() {
                Ok(None)
            } else if mut_guard.error.is_some(){
                let err = mut_guard.error.take().expect("Mutex poisoned");
                Err(err)
            } else {
                Ok(mut_guard.buf.pop_front())
            }
        }
    }

    #[test]
    fn test_buffer(){
        let b = Arc::new(Buffer::new());
        let b_prod = b.clone();
        let b_cons = b.clone();

        let producer = thread::spawn(move || {
            for i in 0..10 {
                b.next(i);
                if i == 5{
                    sleep(Duration::from_millis(2000));

                }
            }
            b.terminate();
        });

        let consumer = thread::spawn(move || {
            for i in 0..10 {
                let t = b_cons.consume().unwrap();
                println!("thread consumer : {:?}", t);
            }
        });
        producer.join().unwrap();
        consumer.join().unwrap();
    }
}

fn main() {
    println!("Hello, world!");
}
