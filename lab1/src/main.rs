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

    let s_in_chars: Vec<char> = SUBS_I.chars().collect();
    let s_out_chars: Vec<char> = SUBS_O.chars().collect();

    if let Some(index) = s_in_chars.iter().position(|&x| x == c) {
        return s_out_chars[index];
    }

    // Handle non-matching characters
    if c.is_ascii_alphanumeric() {
        return c;
    } else {
        return '-';
    }
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

    // ex 2 testing
    #[test]
    fn test_conv_valid_char(){
        assert_eq!('a', conv('a'));
        assert_eq!('1', conv('1'));
    }

    #[test]
    fn test_conv_invalid_char(){
        assert_eq!('-', conv('.'));
        assert_eq!('-', conv(','));
    }

    #[test]
    fn test_conv_accent_chars(){
        assert_eq!('e', conv('è'));
        assert_eq!('e', conv('ę'));
    }
}




fn main() {
    // ex 1
    run_pangram();
}

