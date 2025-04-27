

// to warm up: the define step by step an adapter for filtering even numbers

pub mod simple_even_iter {
    use std::ops::Rem;
    use std::vec::IntoIter;

    // (1) let start with a simple iterator adapter for just one type, "i32"
    // see the adapter pattern example in the pdf "Adapter Pattern..."
    struct EvenIter<I>{
        inner: I
    }

    impl<I> EvenIter<I> {
        fn new(iter: I) -> Self {
            EvenIter{
                inner: iter
            }
        }
    }

    impl<I> Iterator for EvenIter<I>
    where I: IntoIterator<Item = i32> + Iterator<Item = i32> + Clone,
          <I as Iterator>::Item: Rem<i32>{
        type Item = i32; // <== it will work just for i32

        fn next(&mut self) -> Option<Self::Item> {
            loop{
                if let Some(num) = self.inner.next(){
                    if num % 2 == 0 {
                        return Some(num)
                    }
                }else {
                    return None;
                }
            }
        }
    }

    // if EvenIter works the test will compile and pass
    #[test]
    fn test_simple_even_iter() {
        let v = vec![1, 2, 3, 4, 5];
        // why iter() does not work here?
        let it = EvenIter::new(v.into_iter());

        for i in it {
            println!("i: {}", i);
        }
    }

    // (2) now let's add the adapter to all Iterator<Item=i32> (adavanced)
    trait AddEvenIter: Iterator
    where
        Self: Sized
    {
        // add even() to anyone implementing this trait
        // usage: v.into_iter().even() ....
        fn even(self) -> EvenIter<Self>{
            EvenIter::new(self)
        }
    }

    // (3) add here the generic implemention, you can supply it for all the iterators
    // impl .... ?

    impl<I> AddEvenIter for I where I: Iterator {}

    #[test]
    fn test_adapter() {
        let v = vec![1,2,3,4,5];
        for i in v.into_iter().even() {
            println!("{}", i);
        }
    }

}

pub mod even_iter {
    // (4) more adavanced: implement for all integer types
    // => install the external crate "num" to have some Traits identifying all number types
    use num;

    // the generic parameters I and U are already defined for you in the struct definition

    struct EvenIter<I, U>
    where
        I: Iterator<Item = U> {
        iter: I
    }

    impl<I,U> Iterator for EvenIter<I, U>
    where
        U: num::Integer + Copy,
        I: Iterator<Item = U> {
        type Item = U;

        fn next(&mut self) -> Option<Self::Item> {
            let retval = self.iter.next();
            if let Some(val) = retval {
                if val.is_even() {
                    Some(val)
                } else {
                    self.iter.next()
                }
            } else {
                None
            }
        }

    }

    // (6) once implemented, the test will compile and pass
    #[test]
    fn test_even_iter() {
        let mut v: Vec<u64> = vec![1, 2, 3, 4, 5];
        let mut it = EvenIter { iter: v.into_iter() };
        for i in it {
            println!("i: {}", i);
        }
    }

}

// finally let's implement the grep command
// (1) install the "walkdir" crate for walking over directories using an iterator
// install also the "regex" crate for regular expressions

use std::fs;
use std::io::BufReader;
use std::ops::Deref;
use walkdir;
use regex;
use walkdir::{DirEntry, IntoIter};

// (2) define the match result
struct Match {
    file: String,
    line: usize,
    text: String
}

// (3) test walkdir iterator, see how errors are handled
#[test]
fn test_walk_dir() {
    let wdir = walkdir::WalkDir::new("/tmp");
    for entry in wdir.into_iter() {
        // print the name of the file or an error message
        match entry {
            Ok(e) => println!("File: {}", e.path().display()),
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}

// (3) define the grep adapter for the iterator
// add anything you need implement it
struct GrepIter {
    inner: walkdir::IntoIter,
    grep: regex::bytes::Regex
}

impl GrepIter {
    fn new(iter: walkdir::IntoIter) -> Self {
        GrepIter {
            inner: iter,
            grep: regex::bytes::Regex::new(".*").unwrap()
        }
    }

    fn new_with_grep(iter: walkdir::IntoIter, grep: &str) -> Self {
        GrepIter {
            inner: iter,
            grep: regex::bytes::Regex::new(grep).unwrap()
        }
    }
}

impl Iterator for GrepIter {

    type Item = Result<Match, walkdir::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next(){
            Some(Ok(val)) => {
                let dir:DirEntry = val;
                let file = dir.path().file_name().unwrap().to_str().unwrap().to_string();
                let mut line: usize = 0;
                if let Ok(content) = fs::read_to_string(&dir.path()){
                    for l in content.lines(){
                        if self.grep.is_match(l.as_ref()){
                            return Some(Ok(Match{
                                file,
                                line,
                                text: "Match".to_string(),
                            }));
                        }
                        line += 1;
                    }
                    None
                } else {
                    Some(Ok(Match{
                        file,
                        line: 0,
                        text: "Error".to_string(),
                    }))
                }
            }
            Some(Err(err)) => Some(Err(err)),
            _ => None
        }
    }
}

#[test]
fn test_grep_iter() {
    let wdir = walkdir::WalkDir::new("/tmp");
    let grep_iter = GrepIter::new(wdir.into_iter());
    for entry in grep_iter {
        match entry {
            Ok(m) => { println!("File: {}, Line: {}, Text: {}", m.file, m.line, m.text); }
            Err(e) => { println!("Error: {}", e); }
        }
    }
}


// (5) add grep() to IntoIter  (see the first example in EvenIter for i32)

trait Grep {
    fn grep(self, grep: &str) -> GrepIter;
}

impl Grep for IntoIter {
    fn grep(self, grep: &str) -> GrepIter {
        GrepIter::new_with_grep(self, grep)
    }
}


#[test]
fn test_grep() {
    let wdir = walkdir::WalkDir::new("/tmp");
    let grep_iter = wdir.into_iter().grep("xx");
    for entry in grep_iter {
        match entry {
            Ok(m) => { println!("File: {}, Line: {}, Text: {}", m.file, m.line, m.text); }
            Err(e) => { println!("Error: {}", e); }
        }
    }
}

