use std::env;
use rs_json::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let json = run(&args);
}
