use std::sync::{Arc, Mutex};
use std::thread;
use itertools::{Itertools, Permutations};

const NUM_THREADS: u8 = 4;

pub fn mk_ops(symbols: &[char], n: usize) -> Vec<String> {
    if n == 0 {
        return vec![String::new()];
    }

    let mut result = vec![];

    for &symbol in symbols {
        for perm in mk_ops(symbols, n - 1) {
            result.push(format!("{}{}", symbol, perm));
        }
    }

    result
}

pub fn prepare(s: &str) -> Vec<String> {

    let mut result = vec![];
    let ops = mk_ops(&['+', '-', '*', '/'], 4);

    for digit in s.chars().permutations(s.len()) {
        for op_seq in &ops {
            let mut s = String::new();
            let mut it_op = op_seq.chars();
            for d in digit.iter() {
                s.push(*d);
                if let Some(op) = it_op.next() {
                    s.push(op);
                }
            }
            result.push(s);
        }
    }
    result
}

#[test]
fn test_mk_ops() {
    let symbols = vec!['+', '-', '*', '/'];
    let n = 4;
    let result = mk_ops(&symbols, n);
    assert_eq!(result.len(), symbols.len().pow(n as u32));

    let res = prepare("23423");
    println!("{} {:?}", res.len(), res.iter().take(n).collect::<Vec<_>>());
}

pub fn verify(v: &[String]) -> Vec<String> {
    let mut res = Arc::new(Mutex::new(Vec::<String>::new()));

    //let mut threads = vec![];

    let mut lower = 0;
    let mut upper = 0;

    thread::scope(|s| {
        for i in 0..NUM_THREADS {
            let res_lock = res.clone();
            lower = (v.len()/NUM_THREADS as usize)*(i as usize);
            upper = (v.len()/NUM_THREADS as usize)*(i as usize +1);
            let mut thread_v = &v[lower..upper];
            s.spawn(move || {
                for v_ref in thread_v.iter() {
                    if verify_one(v_ref) {
                        res_lock.lock().unwrap().push(v_ref.clone());
                    }
                }
            });
        }
    });

    //for thread in threads { thread.join().unwrap(); }
    res.lock().unwrap().to_vec()
}

fn verify_one(s: &String) -> bool {
    let string = s.clone();
    let mut tot:i32 = 0;
    let mut char_iter = string.chars();

    loop {
        if let Some(c) = char_iter.next() {
            match c {
                '+' => tot +=  char_iter.next().unwrap().to_digit(10).unwrap() as i32,
                '-' => tot -= char_iter.next().unwrap().to_digit(10).unwrap() as i32,
                '*' => tot *= char_iter.next().unwrap().to_digit(10).unwrap() as i32,
                '/' => {
                    let next = char_iter.next().unwrap().to_digit(10).unwrap();
                    if next == 0 {panic!("Division by 0")}
                    tot /= next as i32;
                },
                _ => tot += c.to_digit(10).unwrap() as i32,
            }
        } else {
            break;
        }
    }

    tot == 10
}

pub mod test_game{
    use super::*;
    #[test]
    pub fn test_game(){
        let res = verify(&prepare("23423"));
        println!("game test: {}", verify(&prepare("23423")).len());

        print!("[ ");
        for r in res.iter().take(10) {
            print!("{}, ", r);
        }
        println!("...]");
    }
}