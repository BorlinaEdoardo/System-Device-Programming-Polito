trait Summarize{
    fn summary(&self) -> String;
}

impl Summarize for f64{
    fn summary(&self) -> String {
        format!("{:.4}",self)
    }
}

fn main() {
    let boh: f64 = 1.0/3.0;
    println!("number: {}, summarized number: {}", boh, boh.summary());
}
