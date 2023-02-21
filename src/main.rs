use std::{process, env};
use twojson::Config;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("In file {}",config.file_path);
    if let Err(e) = twojson::run(config) {
        eprintln!("Application error; {e}");
        process::exit(1);
    }
    eprintln!("Elapsed: {} ms", get_elapsed_time(start_time));
}

fn get_elapsed_time(start_time: Instant) -> String {
    let x = start_time.elapsed();
    ((x.as_secs() * 1_000) + (x.subsec_millis()) as u64).to_string()
}