mod delayed_queue{
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;
    use std::sync::{Condvar, Mutex};
    use std::time::Instant;

    struct Item<T:Send> {
        t:T,
        i:Instant,
    }

    impl<T: Send> PartialEq for Item<T>{
        fn eq(&self, other: &Self) -> bool {
            self.i.eq(&other.i)
        }
    }

    impl<T:Send> Eq for Item<T>{}

    impl<T:Send> PartialOrd for Item<T>{
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            other.i.partial_cmp(&self.i)
        }
    }
    
    impl<T:Send> Ord for Item<T>{
        fn cmp(&self, other: &Self) -> Ordering {
            other.i.cmp(&self.i)
        }
    }

    pub struct DelayedQueue<T: std::marker::Send>{
        data: Mutex<BinaryHeap<Item<T>>>,
        cv: Condvar,
    }

    impl <T:Send> DelayedQueue<T>{
        pub fn new() -> DelayedQueue<T>{
            Self {
                data: Mutex::new(BinaryHeap::new()),
                cv: Condvar::new(),
            }
        }

        pub fn offer(&self, t:T, i:Instant){
            let mut data = self.data.lock().expect("poisoned mutex during offer");
            data.push(Item{t,i});
            drop(data);

            self.cv.notify_all();
        }

        pub fn take(&self) -> Option<T>{
            let mut data =  self.data.lock().expect("poisoned mutex during offer");
            loop {
                let now = Instant::now();
                let first = data.peek();

                if let Some(item) = first {
                    let i = item.i;
                    if i < now {
                        let res = data.pop().unwrap();
                        return Some(res.t);
                    } else {
                        data = self.cv.wait_timeout(data, now.duration_since(now)).expect("Mutex Poisoned").0
                    }
                } else {
                    return None;
                }
            }
        }

        pub fn size(&self) -> usize{
            self.data.lock().expect("poisoned mutex during offer").len()
        }
    }
}

mod test{
    use std::ops::Add;
    use std::time::{Duration, Instant};
    use crate::delayed_queue::{DelayedQueue, };

    #[test]
    fn an_empty_queue_returns_none(){
        let q = DelayedQueue::<i32>::new();
        assert_eq!(q.size(), 0);
    }

    #[test]
    fn items_are_returned_in_order(){
        let q = DelayedQueue::<i32>::new();
        let now = Instant::now();
        q.offer(1500, now.add(Duration::from_millis(10)));
        q.offer(500, now.add(Duration::from_millis(5)));

        assert_eq!(q.take(), Some(500));
        assert_eq!(q.take(), Some(1500));
        assert_eq!(q.take(), None);

    }
}