/*
Un paradigma frequentemente usato nei sistemi reattivi è costituito dall’astrazione detta Looper

Quando viene creato, un Looper crea una coda di oggetti generici di tipo Message ed un thread.
Il thread attende - senza consumare cicli di CPU - che siano presenti messaggi nella coda, li
estrae a uno a uno nell’ordine di arrivo, e li elabora. Il costruttore di Looper riceve due parametri,
entrambi di tipo (puntatore a) funzione: process(…) e cleanup(). La prima è una funzione
responsabile di elaborare i singoli messaggi ricevuti attraverso la coda; tale funzione accetta un
unico parametro in ingresso di tipo Message e non ritorna nulla; La seconda è funzione priva di
argomenti e valore di ritorno e verrà invocata dal thread incapsulato nel Looper quando esso
starà per terminare.
Looper offre un unico metodo pubblico, thread safe, oltre a quelli di servizio, necessari per
gestirne il ciclo di vita: send(msg), che accetta come parametro un oggetto generico di tipo
Message che verrà inserito nella coda e successivamente estratto dal thread ed inoltrato alla
funzione di elaborazione. Quando un oggetto Looper viene distrutto, occorre fare in modo che il
thread contenuto al suo interno invochi la seconda funzione passata nel costruttore e poi termini.
Si implementi, utilizzando il linguaggio Rust o C++, tale astrazione tenendo conto che i suoi metodi
dovranno essere thread-safe.
*/

use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};
use std::mem;

#[derive(Debug)]
pub enum Message{
    Msg(String)
} 

pub struct Looper{
    sender: Sender<Message>,
    thread: Option<JoinHandle<()>>
}

impl Looper{
    pub fn new<T, Q>(process: T, cleanup: Q) -> Looper 
    where 
        T: Fn(Message) + Send + 'static, 
        Q: Fn() + Send + 'static 
    {
        let (sender, receiver) = mpsc::channel::<Message>();
        let handle = thread::spawn(move || {
            loop {
                match receiver.recv() {
                    Ok(msg) =>{ process(msg); cleanup();},
                    Err(_) => break
                }
            }
            
        });
        Looper { sender, thread: Some(handle) }
    }

    pub fn send(&self, msg: Message) {
        // If send fails, the receiver is already closed (thread terminated)
        let _ = self.sender.send(msg);
    }
}

impl Drop for Looper {
    fn drop(&mut self) {
        // Explicitly drop the sender to close the channel and signal thread to exit
        // We need to replace it with a dummy sender to satisfy the borrow checker
        let (dummy_sender, _) = mpsc::channel();
        let _ = mem::replace(&mut self.sender, dummy_sender);
        
        // Now wait for the thread to finish
        if let Some(handle) = self.thread.take() {
            let _ = handle.join();
        }
    }
}

fn main() {
    // Test 1: Basic functionality
    let looper = Looper::new(
        |msg| println!("Processing: {:?}", msg),
        || println!("Cleanup called")
    );
    
    looper.send(Message::Msg("Hello".to_string()));
    looper.send(Message::Msg("World".to_string()));
    
    // Test 2: Multiple messages
    let looper2 = Looper::new(
        |msg| {
            if let Message::Msg(s) = msg {
                println!("Received: {}", s);
            }
        },
        || println!("Looper2 cleanup")
    );
    
    for i in 0..5 {
        looper2.send(Message::Msg(format!("Message {}", i)));
    }
    
    // Test 3: Cleanup verification
    {
        let looper3 = Looper::new(
            |_| {},
            || println!("Looper3 cleanup - this should print when dropped")
        );
        looper3.send(Message::Msg("test".to_string()));
    } // looper3 drops here
    
    println!("All tests completed");
    
    // Give some time for async processing
    std::thread::sleep(std::time::Duration::from_millis(100));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    #[test]
    fn test_message_processing() {
        let processed = Arc::new(Mutex::new(Vec::new()));
        let processed_clone = processed.clone();
        
        let looper = Looper::new(
            move |msg| {
                if let Message::Msg(s) = msg {
                    processed_clone.lock().unwrap().push(s);
                }
            },
            || {}
        );
        
        looper.send(Message::Msg("test1".to_string()));
        looper.send(Message::Msg("test2".to_string()));
        
        std::thread::sleep(Duration::from_millis(50));
        
        let messages = processed.lock().unwrap();
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0], "test1");
        assert_eq!(messages[1], "test2");
    }

    #[test]
    fn test_cleanup_called() {
        let cleanup_called = Arc::new(Mutex::new(false));
        let cleanup_clone = cleanup_called.clone();
        
        {
            let _looper = Looper::new(
                |_| {},
                move || {
                    *cleanup_clone.lock().unwrap() = true;
                }
            );
        } // Looper drops here
        
        std::thread::sleep(Duration::from_millis(50));
        assert!(*cleanup_called.lock().unwrap());
    }

    #[test]
    fn test_thread_safety() {
        let counter = Arc::new(Mutex::new(0));
        let counter_clone = counter.clone();
        
        let looper = Arc::new(Looper::new(
            move |_| {
                *counter_clone.lock().unwrap() += 1;
            },
            || {}
        ));
        
        let handles: Vec<_> = (0..10).map(|i| {
            let looper_clone = looper.clone();
            std::thread::spawn(move || {
                looper_clone.send(Message::Msg(format!("msg{}", i)));
            })
        }).collect();
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        std::thread::sleep(Duration::from_millis(100));
        assert_eq!(*counter.lock().unwrap(), 10);
    }
}
