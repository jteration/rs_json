use std::env;
use std::error::Error;
use rs_json::run;
use rs_json::JsonValue;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let start = SystemTime::now();
    let json_value: Result<JsonValue, Box<dyn Error>> = run(&args);
    let end = SystemTime::now();

    let since_the_epoch_start = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let since_the_epoch_end = end
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let in_ms_start = since_the_epoch_start.as_secs() * 1000 +
        since_the_epoch_start.subsec_nanos() as u64 / 1_000_000;

    let in_ms_end = since_the_epoch_end.as_secs() * 1000 +
        since_the_epoch_end.subsec_nanos() as u64 / 1_000_000;

    println!("json_value: {:?}", json_value);
    println!("Start: {:?}, End: {:?}", in_ms_start, in_ms_end);
    println!("Elapsed: {:?}", in_ms_end - in_ms_start);
}
