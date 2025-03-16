use std::{env, fs};

// ex1 functions
fn is_pangram(counts: &[u32]) -> bool {
    // create a simble table for each letter in the alphabet
    let mut pangram = 26;

    for val in counts {
        if(*val > 0){
            pangram -= 1;
        }
    }

    return pangram == 0;
}

fn read_file(file_name: &String) -> [u32; 26]{
    let file_content = fs::read_to_string(file_name).unwrap();
    let char_vec: Vec<char> = file_content.chars().collect();
    let mut vec:[u32; 26] = [0; 26];

    for c in char_vec {
        let unicode_val = c as u32 - 'a' as u32;
        vec[unicode_val as usize] += 1;
    }

    return vec;
}

// call this function from main
// load here the contents of the file
pub fn run_pangram() {
    // read file content
    let args: Vec<String> = env::args().collect();
    if( is_pangram(&read_file(&args[1])) ){
        println!("Pangram reads from file {}", args[1]);
    } else{
        println!("No Pangram reads from file {}", args[1]);
    }
}

// Ex 2 functions
// Convert not allowed charatcter
fn conv(c: char) -> char {
    const SUBS_I: &str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
    const SUBS_O: &str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";

    let mut ret_val:char = 'a' as char;
    let s_in = SUBS_I.to_string();
    let s_out = SUBS_O.to_string();

    let index = s_in.find(c);

    if (index.is_none()){
            if(('a'..='z').contains(&c) || ('1'..='9').contains(&c)) {
                ret_val = c;
            } else {
                ret_val = '-';
            }
    } else {
        ret_val = s_out.chars().collect::<Vec<_>>()[index.unwrap()];
    }

    return ret_val;
}

// return the "slug" version of the imput string
fn slugify(s: &str) -> String {
    return "Not Yet Implemented".to_string();
}



// ex 1 testing
#[cfg(test)] // this is a test module
mod tests
{
    // tests are separated modules, you must import the code you are testing
    use super::*;

    // ex 1 test functions:
    #[test]
    fn test_file_reading(){
        let filename = "./sentence.txt".to_string();
        let char_vec: [u32; 26] = read_file(&filename);

        println!("{:?}", char_vec);
    }

    #[test]
    fn test_all_ones() {
        let counts = [1; 26];
        assert!(is_pangram(&counts));
    }

    #[test]
    fn test_some_zeros() {
        let mut counts = [0; 26];
        counts[0] = 0;
        counts[1] = 0;
        assert!(!is_pangram(&counts));
    }

    #[test]
    fn test_increasing_counts() {
        let mut counts = [0; 26];
        for i in 0..26 {
            counts[i] = i as u32 + 1;
        }
        assert!(is_pangram(&counts));
    }

    #[test]
    fn test_wrong_size()  {
        let counts = [1; 25];
        assert!(!is_pangram(&counts));
    }
}

// ex 2 testing
#[cfg(test2)]
mod tests{
    #[test]
    fn test_conv_valid_char(){
        let c = 'a';
        assert_eq!('a', conv(c));
    }


}


fn main() {
    // ex 1
    run_pangram();
}

