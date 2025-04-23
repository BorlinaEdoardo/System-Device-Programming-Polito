// WARNING: 
// - the lifetimes are not set correctly, you have to set them to make it compile
// - you have also to implemment missing functions and fix the code
// - *** see test test functions in the code for usage examples

use std::fs::File;
use std::io;
use std::io::{BufRead, Read};

use regex::{ Regex};

// (1) LineEditor: implement functionality
#[derive(Debug, Clone)]
pub struct LineEditor {
    lines: Vec<String>,
}

impl LineEditor {
    pub fn new(s: String) -> Self {
        LineEditor {
            lines: s.lines().map(|x| x.to_string()).collect(),
        }
    }

    // create a new LineEditor from a file
    pub fn from_file(file_name: &str) -> Result<Self, io::Error> {
        let mut file = File::open(file_name)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(LineEditor::new(contents))
    }

    pub fn all_lines(&self) -> Vec<&str> {
        let mut result = Vec::with_capacity(self.lines.len());
        for s in &self.lines {
            result.push(s.as_str());
        }
        result
    }

    pub fn replace(&mut self, line: usize, start: usize, end: usize, subst: &str) {
        if let Some(l) = self.lines.get_mut(line) {
            let new_line = format!("{}{}{}", &l[..start], subst, &l[end..]);
            *l = new_line;
        }
    }
}



// (2) Match contains the information about the match. Fix the lifetimes
// repl will contain the replacement.
// It is an Option because it may be not set yet, or it may be skipped
#[derive(Debug, Clone)]
struct Match<'a> {
    pub line: usize,
    pub start: usize,
    pub end: usize,
    pub text: &'a str,
    pub repl: Option<String>,
}



// use the crate "regex" to find the pattern and its method find_iter for iterating over the matches
// modify if necessary, this is just an example for using a regex to find a pattern
fn find<'a, 'b>(lines: &'b Vec<&'a str>, pattern: &'a str) -> Vec<Match<'a>> {
    let mut matches = Vec::new();
    let re = Regex::new(pattern).unwrap();
    for (line_idx, line) in lines.iter().enumerate() {
        for mat in re.find_iter(line) {
            matches.push(Match {
                line: line_idx,
                start: mat.start(),
                end: mat.end(),
                text: & line[mat.start()..mat.end()],
                repl: None,
            });
        }
    }
    matches
}





// (3) Fix the lifetimes of the FindReplace struct
// (4) implement the Finder struct
#[derive(Clone)]
struct FindReplace<'a> {
    lines: Vec<&'a str>,
    pattern: String,
    matches: Vec<Match<'a>>,
}

impl<'a> FindReplace<'a> {
    pub fn new(lines: Vec<&'a str>, pattern: &'a str) -> FindReplace<'a> {
        let mut fr = FindReplace {
            lines: lines.clone(),
            pattern: pattern.to_string(),
            matches: Vec::new(), // inizialmente vuoto
        };
        fr.matches = find(&(fr).lines, pattern); // ora possiamo usare fr.lines
        fr
    }

    // return all the matches
    pub fn matches(&self) -> &Vec<Match> {
        &self.matches
    }



    // apply a function to all matches and allow to accept them and set the repl
    // useful for promptig the user for a replacement
    pub fn apply(&mut self, fun: impl Fn(&mut Match) -> bool) {
        self.matches.iter_mut().for_each(|m| {fun(m);});
    }
}


//(5) how FindReplace should work together with the LineEditor in order
// to replace the matches in the text
#[test]
fn test_find_replace() {
    let s = "Hello World.\nA second line full of text.";
    let mut editor = LineEditor::new(s.to_string());

    let mut lines = editor.all_lines();
    let mut finder = FindReplace::new(lines.clone(), "ll");

    // find all the matches and accept them
    finder.apply(|m| {
        println!("{} {} {} {}", m.line, m.start, m.end, m.text);
        m.repl = Some("--".to_string());
        true
    });

    // now let's replace the matches
    // why this loop won't work?
    //for m: Match in finder.matches() {
    //    editor.replace(/* add match */);
    //}

    // alternate method: why this one works?

    let subs: Vec<(usize, usize, usize, String)> = finder
        .matches()
        .iter()
        .map(|m| (m.line, m.start, m.end, m.repl.clone().unwrap()))
        .collect();

    for (line, start, end, subst) in subs {
        editor.replace(line, start, end, &subst);
    }

    println!("{}", editor.lines.join("\n"));
}



// (6) sometimes it's very expensive to find all the matches at once before applying 
// the changes
// we can implement a lazy finder that finds just the next match and returns it
// each call to next() will return the next match
// this is a naive implementation of an Iterarator

#[derive(Debug, Clone, Copy)]
struct FinderPos {
    pub line: usize,
    pub offset: usize,
}

struct LazyFinder<'a> {
    lines: Vec<& 'a str>,
    pattern: String,
    pos: Option<FinderPos>,
}

impl<'a> LazyFinder<'a> {
    pub fn new(lines: Vec<& 'a str>, pattern: & 'a str) -> Self {
        LazyFinder{
            lines,
            pattern: pattern.to_string(),
            pos: None,
        }
    }

    pub fn next(&mut self) -> Option<Match> {
        // remember:
        // return None if there are no more matches
        // return Some(Match) if there is a match
        // each time save the position of the match for the next call
        let re = Regex::new(self.pattern.as_str()).unwrap();
        let start:usize = if self.pos.is_none() { 0 }  else { self.pos.unwrap().line };
        for (line_id, line) in self.lines[start..].iter().enumerate() {
            let m = re.find(line.as_ref());
            if m.is_some() {
                if self.pos.is_none() || m?.start() != self.pos.unwrap().offset {
                    self.pos = Some(
                        FinderPos {
                            line: line_id,
                            offset: m?.start(),
                        }
                    );
                    return Some(Match {
                        line: line_id,
                        start: m?.start(),
                        end: m?.end(),
                        text: &line[m?.start()..m?.end()],
                        repl: None,
                    });
                }
            }
        }
        // if no match is found return none
        None
    }
}


// (7) example of how to use the LazyFinder
#[test]
fn test_lazy_finder() {
    let s = "Hello World.\nA second line full of text.";
    let mut editor = LineEditor::new(s.to_string());

    let lines = editor.all_lines();
    let mut finder = LazyFinder::new(lines, "ll");

    // find all the matches and accept them 
    while let Some(m) = finder.next() {
        println!("{} {} {} {}", m.line, m.start, m.end, m.text);
    }
}


// (8) now you have everything you need to implement the real Iterator
#[derive(Clone)]
struct FindIter<'a> {
    lines: Vec<&'a str>,
    re:   Regex,
    line: usize,
    offs: usize,
}

impl<'a> FindIter<'a> {
    pub fn new(lines: Vec<&'a str>, pattern: &str) -> Self {
        FindIter {
            lines,
            re:   Regex::new(pattern).unwrap(),
            line: 0,
            offs: 0,
        }
    }
}

impl<'a> Iterator for FindIter<'a> {
    type Item = Match<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.line < self.lines.len() {
            let hay = self.lines[self.line];
            // try to find a match at or after offs
            if let Some(mat) = self.re.find_at(hay, self.offs) {
                let start = mat.start();
                let end   = mat.end();
                // prepare the Match<'a>
                let m = Match {
                    line: self.line,
                    start,
                    end,
                    text:  &hay[start..end],
                    repl:  None,
                };
                // bump offs so the next call continues after this match
                self.offs = end;
                return Some(m);
            }
            // no more on this line → advance to next
            self.line += 1;
            self.offs  = 0;
        }
        None
    }
}

// (9) test the find iterator
#[test]
fn test_find_iter() {
    let s = "Hello World.\nA second line full of text.";
    let mut editor = LineEditor::new(s.to_string());

    let lines = editor.all_lines();
    let mut finder = FindIter::new(lines, "ll");

    // find all the matches and accept them 
    for m in finder {
        println!("{} {} {} {}", m.line, m.start, m.end, m.text);
    
    }
}


