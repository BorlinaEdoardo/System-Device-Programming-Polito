
// version 1
/*trait MySlug{
    fn is_slug(&self)->bool;
    fn to_slug(&self)->String;
}

impl MySlug for String{
    fn to_slug(&self) -> String {
        return slugify(self.as_str());
    }
    fn is_slug(&self)->bool{
        self.eq(&self.to_slug())
    }
}

impl MySlug for &str{
    fn to_slug(&self) -> String {
        slugify(&self)
    }
    fn is_slug(&self)->bool{
        self.eq(&(self.to_slug().as_str()))
    }
}*/

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


// version 2
trait Slug{
    fn is_slug(&self)->bool;
    fn to_slug(&self)->String;
}

impl<T> Slug for T
    where T : AsRef<str> {
        fn to_slug(&self) -> String {
            slugify(self.as_ref())
        }
        fn is_slug(&self)->bool{
            self.as_ref() == slugify(self.as_ref())
        }
    }



// return the "slug" version of the imput string
fn slugify(s: &str) -> String {
    let chars = s.to_string().to_lowercase().chars().collect::<Vec<char>>();
    let mut ret_val = String::new();
    let mut prev_c = '_';
    let mut conv_c;
    for c in chars  {
        conv_c = conv(c);
        if conv_c != '-' || conv_c != prev_c {
            ret_val.push(conv_c);
        }

        prev_c = conv_c;
    }
    return ret_val;
}



fn main() {
    let s1 = String::from("Hello String");
    let s2 = "hello-slice";
    println!("{}", s1.is_slug()); // false
    println!("{}", s2.is_slug()); // true
    let s3: String = s1.to_slug();
    let s4: String = s2.to_slug();
    println!("s3:{} s4:{}", s3, s4); // stampa: s3:hello-string s4:hello-slice
}
