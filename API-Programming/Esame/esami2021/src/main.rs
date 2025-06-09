
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
    use std::any::Any;
    use std::collections::VecDeque;
    use std::sync::mpsc::{Receiver, Sender};
    use std::sync::{mpsc, Arc, Mutex};

    struct Inner<F>{
        que: Sender<Box<F>>,
        is_closed: bool,
    }

    struct SingleThreadExecutor<F: FnOnce() + Send + 'static + ?Sized> {
        que: Sender<Box<F>>,
        is_closed: Arc<Mutex<bool>>,
    }

    impl<F: FnOnce() + Send + 'static> Clone for SingleThreadExecutor<F>{
        fn clone(&self) -> Self {
            Self{
                que: self.que.clone(),
                is_closed: self.is_closed.clone(),
            }
        }
    }

    impl <F: ?Sized + FnOnce() + Send + 'static> SingleThreadExecutor<F> {
        pub fn new() -> (Self, Receiver<Box<F>>){
            let (tx, rx) : (Sender<Box<F>>, Receiver<Box<F>>) = mpsc::channel();
            let ste = Self{
                que: tx,
                is_closed: Arc::new(Mutex::new(false)),
            };

            (ste, rx)
        }
        pub fn submit(&self, task: F) -> Result<(), String>{
            if *self.is_closed.lock().unwrap() {
                Err("submit task already closed".to_owned())
            } else {
                if self.que.send(Box::new(task)).is_err(){
                    return Err("submit task failed".to_owned());
                }
                Ok(())
            }
        }
        pub fn close(&mut self){
            let mut_guard = self.is_closed.lock().unwrap();
            *mut_guard = true;
        }
    }

    #[test]
    fn test_single_thread(){
        let (ex, receiver) = SingleThreadExecutor::<dyn FnOnce() + Send >::new();
    }
}

 */


use std::sync::{mpsc, Arc, Mutex};

pub struct SingleThreadExecutor {
    sender: mpsc::Sender<Box<dyn FnOnce() + Send + 'static>>,
    is_closed: Arc<Mutex<bool>>,
}

impl SingleThreadExecutor {
    /// Crea un nuovo esecutore e restituisce sia l'esecutore che il ricevitore dei task.
    pub fn new() -> (
        Self,
        mpsc::Receiver<Box<dyn FnOnce() + Send + 'static>>,
    ) {
        let (tx, rx) = mpsc::channel();
        let is_closed = Arc::new(Mutex::new(false));

        let executor = Self {
            sender: tx,
            is_closed,
        };

        (executor, rx)
    }

    /// Invia un task al thread esecutore.
    pub fn submit<F>(&self, task: F) -> Result<(), String>
    where
        F: FnOnce() + Send + 'static,
    {
        let is_closed = self.is_closed.lock().unwrap();
        if *is_closed {
            Err("Executor is closed".to_string())
        } else {
            self.sender
                .send(Box::new(task))
                .map_err(|_| "Failed to send task".to_string())
        }
    }

    /// Chiude l'esecutore: non accetta più nuovi task.
    pub fn close(&self) {
        let mut closed = self.is_closed.lock().unwrap();
        *closed = true;
        // Nota: quando tutti i `sender` vengono drop, il `Receiver` riceverà Err.
    }
}


#[cfg(test)]
mod tests_channelz {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_external_thread_executes_task() {
        let (executor, rx) = SingleThreadExecutor::new();
        let result = Arc::new(Mutex::new(0));

        let result_clone = Arc::clone(&result);
        executor
            .submit(move || {
                *result_clone.lock().unwrap() = 123;
            })
            .unwrap();

        let handle = thread::spawn(move || {
            while let Ok(task) = rx.recv() {
                task();
            }
        });

        executor.close();
        drop(executor);
        handle.join().unwrap();

        assert_eq!(*result.lock().unwrap(), 123);
    }

    #[test]
    fn test_submit_after_close_fails() {
        let (executor, _rx) = SingleThreadExecutor::new();
        executor.close();
        let res = executor.submit(|| println!("non dovrebbe eseguire"));
        assert!(res.is_err());
    }
}



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

/* 2 settembre 2021

    In un sistema concorrente sono presenti due gruppi di thread, detti rispettivamente produttori
    e consumatori, che si appoggiano ad una struttura dati condivisa per passarsi dati generici di
    tipo T. La struttura dati è thread-safe ed implementa il concetto di buffer circolare: al suo
    interno ospita un array di N elementi (con N specificato a livello di tipo) nel quale vengono
    depositati i valori ricevuti dai singoli produttori, in attesa che vengano passati ad un
    consumatore qualunque che ne faccia richiesta. I dati sono trattati in modalità FIFO (First-In-
    First-Out).
    • pub fn insert(&self, t: T) {…} → Inserisce un elemento. Se il buffer risulta pieno nel
    momento in cui un produttore prova ad inserire un nuovo valore, l'operazione si blocca,
    senza consumare CPU, in attesa che si crei uno spazio a seguito della conclusione di
    un'operazione di lettura da parte di un consumatore.
    • pub fn extract(&self) -> T {…} → Estrae un elemento. Analogamente, se un
    consumatore prova a leggere un valore mentre il buffer è vuoto, deve attendere, senza
    consumare CPU, che un produttore inserisca un nuovo valore.
    Per generalità, accanto alle operazioni base di inserimento ed estrazione di un valore, la
    struttura dati offre anche una coppia di operazioni con attesa limitata temporalmente
    • pub fn try_insert_for(&self, t: T, d: Duration) -> TimeoutResult {…} → Cerca di inserire
    un elemento, se non riesce entro l'intervallo duration, restituisce false; in caso di
    successo, restituisce true.
    • pub fn try_extract_for(&self, d: Duration) -> (Option<T>, TimeoutResult) {…} → Cerca
    di estrarre un elemento, se non riesce entro l'intervallo duration restituisce un Option
    vuoto; altrimenti restituisce un Option con l'elemento estratto.
    Si implementi la classe generica di seguito proposta utilizzando le funzionalità offerte dal
    linguaggio Rust, aggiungendo le parti eventualmente necessarie per ottenere il
    comportamento descritto.

    difficoltà: non male dai
    tempo: senza test una mezzoretta, + test and debug 50 minuti

 */

pub mod circular_buffer{
    use std::collections::VecDeque;
    use std::sync::{Arc, Condvar, Mutex};
    use std::thread::scope;
    use std::time::Duration;

    struct Inner<T>{
        buf: VecDeque<T>,
        dim: usize,
    }

    pub struct CircularBuffer<T: Send + 'static>{
        inner: Mutex<Inner<T>>,
        cvar_full: Condvar,
        cvar_empty: Condvar,
    }

    pub type TimeoutResult = bool;

    impl<T: Send + 'static> CircularBuffer<T> {
        pub fn new(dim: usize) -> Self{
            Self{
                inner: Mutex::new(
                    Inner{
                        buf: VecDeque::with_capacity(dim),
                        dim
                    }
                ),
                cvar_full: Condvar::new(),
                cvar_empty: Condvar::new(),
            }
        }

        pub fn insert(&self, t: T) {
            let mut mut_guard = self.inner.lock().expect("Mutex poisoned");

            while(mut_guard.buf.len() >= mut_guard.dim){
                mut_guard = self.cvar_full.wait(mut_guard).expect("Mutex poisoned");
            }

            mut_guard.buf.push_back(t);
            self.cvar_empty.notify_one();
        }

        pub fn extract(&self) -> T {
            let mut mut_guard = self.inner.lock().expect("Mutex poisoned");

            while(mut_guard.buf.is_empty()){
                mut_guard = self.cvar_empty.wait(mut_guard).expect("Mutex poisoned");
            }

            let res = mut_guard.buf.pop_front().expect("None is invalid");
            self.cvar_full.notify_one();

            res
        }


        // with duration
        pub fn try_insert_for(&self, t: T, d: Duration) -> TimeoutResult {
            let mut mut_guard = self.inner.lock().expect("Mutex poisoned");
            let mut wait_result;
            while(mut_guard.buf.len() >= mut_guard.dim){
                wait_result = self.cvar_full.wait_timeout(mut_guard, d).unwrap();
                if ! wait_result.1.timed_out() {
                    mut_guard = wait_result.0;
                } else {
                    return false
                }
            }

            mut_guard.buf.push_back(t);
            self.cvar_empty.notify_one();

            true
        }


        pub fn try_extract_for(&self, d: Duration) -> (Option<T>, TimeoutResult) {
            let mut mut_guard = self.inner.lock().expect("Mutex poisoned");
            let mut wait_result;
            while(mut_guard.buf.is_empty()){
                wait_result = self.cvar_empty.wait_timeout(mut_guard, d).unwrap();
                if ! wait_result.1.timed_out() {
                    mut_guard = wait_result.0;
                }else {
                    return (None, false);
                }
            }

            let res = mut_guard.buf.pop_front().expect("None is invalid");
            self.cvar_full.notify_one();

            (Some(res), true)
        }


    }

    #[test]
    pub fn test_circular_buffer(){

        let b = Arc::new(CircularBuffer::new(3));
        let prod = b.clone();

        let _ = scope(|scope| {
            // producer
            scope.spawn(move ||{
                for i in 0..=5{
                    prod.insert(i);
                }

            });

            // consumers
            for _  in 0..=5 {
                let cons = b.clone();
                scope.spawn(move ||{
                    println!("{}", cons.extract())
                });
            }

        });
    }


    #[test]
    pub fn test_circular_buffer_duration(){

        let b = Arc::new(CircularBuffer::new(3));
        let prod = b.clone();

        let _ = scope(|scope| {
            // producer
            scope.spawn(move ||{
                for i in 0..=5{
                    prod.try_insert_for(i, Duration::from_millis(100));
                }

            });

            // consumers
            for _  in 0..=5 {
                let cons = b.clone();
                scope.spawn(move ||{
                    println!("{}", cons.try_extract_for(Duration::from_millis(2000)).0.unwrap());
                });
            }

        });
    }

    /* 18 ottobre 2021

    La struttura generica Processor<T> consente a un insieme di thread produttori di inviare oggetti
    istanza del tipo T (che si assume copiabile) a un thread consumatore, il cui comportamento è
    definito tramite una funzione fornita come parametro del costruttore.
    Il costruttore di tale struttura riceve, come parametro, una funzione (o una closure) che accetta
    un argomento di tipo T e restituisce (). La struttura fornisce una coda per gestire gli oggetti inviati
    dai produttori e offre i seguenti metodi:
    1. Metodo send(&self, item: T): Permette ai produttori di sottomettere un oggetto da
    elaborare. L'oggetto viene inserito in una coda in attesa che il thread consumatore
    esterno lo elabori. Se il metodo close(...) è stato invocato, eventuali chiamate a send(...)
    devono generare un errore o produrre un comportamento indefinito, a scelta
    dell'implementazione.
    2. Metodo close(&self): Segnala la fine dell'accettazione di nuovi elementi. Dopo
    l'invocazione di questo metodo:
    o Non sarà più possibile inviare nuovi dati tramite send(...).
    o Il metodo non ritorna fino a quando la coda non è vuota e tutte le operazioni di
    elaborazione sono state completate.
    La struttura Processor<T> deve garantire la sincronizzazione tra i produttori e il consumatore
    esterno utilizzando primitive di sincronizzazione di Rust. La logica di elaborazione e il ciclo di
    vita del thread consumatore devono essere gestiti esternamente.
    Viene fornita la seguente dichiarazione di struct Rust come punto di partenza:

     */
    /*
    pub struct Processor<T>{

    }

    impl<T: Send + 'static> Processor<T> {
        fn new<F>(f: F) -> Self where F: Fn(T) + Send + 'static {}
        fn send(&self, item: T) {}
        fn close(&self) {}
    }

     */
}

fn main() {
    println!("Hello, world!");
}
