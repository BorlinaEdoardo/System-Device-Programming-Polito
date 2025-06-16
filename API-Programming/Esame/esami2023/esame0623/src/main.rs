/*

La struct MpMcChannel<E: Send> è una implementazione di un canale su cui possono
scrivere molti produttori e da cui possono attingere valori molti consumatori.
Tale struttura offre i seguenti metodi:
new(n: usize) -> Self //crea una istanza del canale basato su un buffer circolare di "n"
elementi
send(e: E) -> Option<()> //invia l'elemento "e" sul canale. Se il buffer circolare è pieno,
attende
//senza consumare CPU che si crei almeno un posto
libero in cui depositare il valore
//Ritorna:
// - Some(()) se è stato possibile inserire il valore nel
buffer circolare
// - None se il canale è stato chiuso (Attenzione: la
chiusura può avvenire anche
// mentre si è in attesa che si liberi spazio) o se si è
verificato un errore interno
recv() -> Option<E> //legge il prossimo elemento presente sul canale. Se il buffer
circolare è vuoto,
//attende senza consumare CPU che venga depositato
almeno un valore
//Ritorna:
// - Some(e) se è stato possibile prelevare un valore dal
buffer
// - None se il canale è stato chiuso (Attenzione: se,
all'atto della chiusura sono
// già presenti valori nel buffer, questi devono essere
ritornati, prima di indicare
// che il buffer è stato chiuso; se la chiusura avviene
mentre si è in attesa di un valore,
// l'attesa si sblocca e viene ritornato None) o se si è
verificato un errore interno.
shutdown() -> Option<()> //chiude il canale, impedendo ulteriori invii di valori.
//Ritorna:
// - Some(()) per indicare la corretta chiusura
// - None in caso di errore interno all'implementazione
del metodo.
Si implementi tale struttura dati in linguaggio Rust, senza utilizzare i canali forniti dalla
libreria standard né da altre librerie, avendo cura di garantirne la correttezza in
presenza di più thread e di non generare la condizione di panico all'interno dei suoi
metodi.

 */
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::{Condvar, Mutex};

struct Inner<E: Send> {
    que: VecDeque<E>,
    size: usize,
    is_closed: bool,
}

pub struct MpMcChannel<E: Send> {
    inner: Mutex<Inner<E>>,
    cvar_read: Condvar,
    cvar_write: Condvar,
}

impl<E: Send> MpMcChannel<E> {
    pub fn new(n: usize) -> MpMcChannel<E> {
        Self {
            inner: Mutex::new(Inner {
                que: VecDeque::with_capacity(n),
                size: n,
                is_closed: false,
            }),
            cvar_read: Condvar::new(),
            cvar_write: Condvar::new(),
        }
    }

    pub fn send(&self, e: E) -> Option<()> {
        let mut guard = match self.inner.lock() {
            Ok(g) => g,
            Err(_) => return None, // Evita panic su poisoned mutex
        };

        // Controlla se il canale è già chiuso
        if guard.is_closed {
            return None;
        }

        // Attende finché c'è spazio nel buffer o il canale viene chiuso
        while guard.que.len() >= guard.size {
            guard = match self.cvar_write.wait(guard) {
                Ok(g) => g,
                Err(_) => return None, // Evita panic su poisoned mutex
            };

            // Ricontrolla se il canale è stato chiuso durante l'attesa
            if guard.is_closed {
                return None;
            }
        }

        // Notifica i reader se il buffer era vuoto
        let was_empty = guard.que.is_empty();
        guard.que.push_back(e);

        if was_empty {
            self.cvar_read.notify_all();
        }

        Some(())
    }

    pub fn recv(&self) -> Option<E> {
        let mut guard = match self.inner.lock() {
            Ok(g) => g,
            Err(_) => return None, // Evita panic su poisoned mutex
        };

        // Attende finché ci sono elementi o il canale viene chiuso
        while guard.que.is_empty() {
            // Se il canale è chiuso e non ci sono più elementi, ritorna None
            if guard.is_closed {
                return None;
            }

            guard = match self.cvar_read.wait(guard) {
                Ok(g) => g,
                Err(_) => return None, // Evita panic su poisoned mutex
            };
        }

        // A questo punto abbiamo almeno un elemento nel buffer
        let was_full = guard.que.len() == guard.size;
        let result = guard.que.pop_front();

        // Notifica i writer se il buffer era pieno
        if was_full {
            self.cvar_write.notify_all();
        }

        result
    }

    pub fn shutdown(&self) -> Option<()> {
        let mut guard = match self.inner.lock() {
            Ok(g) => g,
            Err(_) => return None, // Evita panic su poisoned mutex
        };

        guard.is_closed = true;

        // Notifica tutti i thread in attesa
        self.cvar_write.notify_all();
        self.cvar_read.notify_all();

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_basic_send_recv() {
        let channel = MpMcChannel::new(3);

        assert_eq!(channel.send(1), Some(()));
        assert_eq!(channel.send(2), Some(()));
        assert_eq!(channel.send(3), Some(()));

        assert_eq!(channel.recv(), Some(1));
        assert_eq!(channel.recv(), Some(2));
        assert_eq!(channel.recv(), Some(3));
    }

    #[test]
    fn test_buffer_full() {
        let channel = Arc::new(MpMcChannel::new(2));

        // Riempi il buffer
        assert_eq!(channel.send(1), Some(()));
        assert_eq!(channel.send(2), Some(()));

        let channel_clone = Arc::clone(&channel);
        let handle = thread::spawn(move || {
            // Questo dovrebbe bloccare finché non c'è spazio
            thread::sleep(Duration::from_millis(100));
            channel_clone.send(3)
        });

        // Leggi un elemento per fare spazio
        thread::sleep(Duration::from_millis(50));
        assert_eq!(channel.recv(), Some(1));

        // Ora il send dovrebbe completarsi
        assert_eq!(handle.join().unwrap(), Some(()));
        assert_eq!(channel.recv(), Some(2));
        assert_eq!(channel.recv(), Some(3));
    }

    #[test]
    fn test_multiple_producers_consumers() {
        let channel = Arc::new(MpMcChannel::new(5));
        let mut handles = vec![];

        // Avvia 3 produttori
        for i in 0..3 {
            let channel_clone = Arc::clone(&channel);
            let handle = thread::spawn(move || {
                for j in 0..10 {
                    let value = i * 10 + j;
                    channel_clone.send(value).unwrap();
                }
            });
            handles.push(handle);
        }

        // Avvia 2 consumatori
        let received = Arc::new(Mutex::new(Vec::new()));
        for _ in 0..2 {
            let channel_clone = Arc::clone(&channel);
            let received_clone = Arc::clone(&received);
            let handle = thread::spawn(move || {
                for _ in 0..15 { // 30 elementi totali / 2 consumatori
                    if let Some(value) = channel_clone.recv() {
                        received_clone.lock().unwrap().push(value);
                    }
                }
            });
            handles.push(handle);
        }

        // Aspetta che tutti i thread finiscano
        for handle in handles {
            handle.join().unwrap();
        }

        let received_values = received.lock().unwrap();
        assert_eq!(received_values.len(), 30);
    }

    #[test]
    fn test_shutdown_behavior() {
        let channel = Arc::new(MpMcChannel::new(3));

        // Aggiungi alcuni elementi
        assert_eq!(channel.send(1), Some(()));
        assert_eq!(channel.send(2), Some(()));

        // Chiudi il canale
        assert_eq!(channel.shutdown(), Some(()));

        // I send successivi dovrebbero fallire
        assert_eq!(channel.send(3), None);

        // Ma dovremmo ancora poter leggere gli elementi esistenti
        assert_eq!(channel.recv(), Some(1));
        assert_eq!(channel.recv(), Some(2));

        // Ora non ci sono più elementi e il canale è chiuso
        assert_eq!(channel.recv(), None);
    }

    #[test]
    fn test_shutdown_with_waiting_threads() {
        let channel = Arc::new(MpMcChannel::new(1));

        let channel_clone = Arc::clone(&channel);
        let reader_handle = thread::spawn(move || {
            // Questo dovrebbe bloccare finché il canale non viene chiuso
            channel_clone.recv()
        });

        let channel_clone2 = Arc::clone(&channel);
        let writer_handle = thread::spawn(move || {
            // Riempi il buffer
            channel_clone2.send(1).unwrap();
            // Questo dovrebbe bloccare finché il canale non viene chiuso
            channel_clone2.send(2)
        });

        thread::sleep(Duration::from_millis(100));

        // Chiudi il canale
        channel.shutdown();

        // I thread dovrebbero sbloccarsi
        assert_eq!(reader_handle.join().unwrap(), Some(1));
        assert_eq!(writer_handle.join().unwrap(), Some(()));
    }
}


fn main() {
    let mut valore = Rc::new(5);
    {
        println!("Value: {:?}", valore);

        let copia = Rc::clone(&valore);
        println!("Copied value: {:?}", copia);

        match Rc::get_mut(&mut valore) {
            Some(v) => *v += 10,
            None => println!("It seems that something had been wrong (case A)"),
        }
    }

    match Rc::get_mut(&mut valore) {
        Some(v) => *v += 10,
        None => println!("It seems that something had been wrong (case B)"),
    }

    println!("The final value is: {:?}", valore);
}


