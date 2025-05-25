use std::sync::{Arc, Mutex};
use std::thread;

pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn find_primesv1(limit: u64, n_threads: u64) -> Vec<u64> {
    let shared_num = Arc::new(Mutex::new(2));
    let shared_vec = Arc::new(Mutex::new(Vec::<u64>::new()));

    let mut threads = vec![];

    for _ in 0..n_threads {
        let num_clone = Arc::clone(&shared_num);
        let vec_clone = Arc::clone(&shared_vec);

        threads.push(thread::spawn(move || {
            loop {
                let current_num = {
                    let mut num_guard = num_clone.lock().unwrap();
                    if *num_guard >= limit {
                        break;
                    }
                    let current = *num_guard;
                    *num_guard += 1;
                    current
                };

                if is_prime(current_num) {
                    let mut vec_guard = vec_clone.lock().unwrap();
                    vec_guard.push(current_num);
                }
            }
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }


    let mut result = shared_vec.lock().unwrap().clone();
    result
}


pub fn find_primesv2(limit: u64, n_threads: u64) -> Vec<u64> {
    let mut result = Arc::new(Mutex::new(Vec::<u64>::new()));
    let mut threads = vec![];

    for i in 0..(n_threads) {
        let mut result_guard = result.clone();
        threads.push(thread::spawn(move || {
            let mut num = i;
            while num <= limit {

                if is_prime(num) {
                    let mut v = result_guard.lock().unwrap();
                    v.push(num);
                }
                num += n_threads;
            }


        }))
    }
    for thread in threads {
        thread.join().unwrap()
    }

    let mut res = result.lock().unwrap().to_vec();
    // res.sort();
    res

}

pub mod primes_tests {
    use std::time::Instant;
    use super::*;
    #[test]
    fn test_find_primesv1() {
        let start = Instant::now();
        let vec = find_primesv1(1000000, 3);
        let duration = start.elapsed();

        println!("find_primesv1 (with shared variable):");
        println!("  - Primi trovati: {}", vec.len());
        println!("  - Tempo impiegato: {:?}", duration);
        println!("  - Tempo in ms: {:.2}", duration.as_millis());
    }

    #[test]
    fn test_find_primesv2() {
        let start = Instant::now();
        let vec = find_primesv2(1000000, 3);
        let duration = start.elapsed();

        println!("find_primesv2 (without shared variable):");
        println!("  - Primi trovati: {}", vec.len());
        println!("  - Tempo impiegato: {:?}", duration);
        println!("  - Tempo in ms: {:.2}", duration.as_millis());
    }

    #[test]
    fn benchmark_comparison() {
        println!("=== BENCHMARK COMPARISON ===");

        // Test versione 1
        let start1 = Instant::now();
        let vec1 = find_primesv1(100_000_000, 16);
        let duration1 = start1.elapsed();

        // Test versione 2
        let start2 = Instant::now();
        let vec2 = find_primesv2(100_000_000, 16);
        let duration2 = start2.elapsed();

        println!("Versione 1 (shared vec): {} primi in {:?}", vec1.len(), duration1);
        println!("Versione 2 (local vecs): {} primi in {:?}", vec2.len(), duration2);

        let speedup = duration1.as_nanos() as f64 / duration2.as_nanos() as f64;
        if speedup > 1.0 {
            println!("Versione 2 è {:.2}x più veloce", speedup);
        } else {
            println!("Versione 1 è {:.2}x più veloce", 1.0 / speedup);
        }

        // Verifica che i risultati siano uguali
        assert_eq!(vec1.len(), vec2.len(), "Le due versioni dovrebbero trovare lo stesso numero di primi");
    }
}
