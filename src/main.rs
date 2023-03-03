use std::process;
use std::time::Instant;
use clap::Parser;

#[derive(Parser)]
#[command(name = "2 Fast 2 Json")]
#[command(version = "1.0")]
#[command(about = r"
██████╗ ███████╗ █████╗  ██████╗████████╗  ██████╗      ██╗ ██████╗ █████╗ ███╗  ██╗
╚════██╗██╔════╝██╔══██╗██╔════╝╚══██╔══╝  ╚════██╗     ██║██╔════╝██╔══██╗████╗ ██║
  ███╔═╝█████╗  ███████║╚█████╗    ██║       ███╔═╝     ██║╚█████╗ ██║  ██║██╔██╗██║
██╔══╝  ██╔══╝  ██╔══██║ ╚═══██╗   ██║     ██╔══╝  ██╗  ██║ ╚═══██╗██║  ██║██║╚████║
███████╗██║     ██║  ██║██████╔╝   ██║     ███████╗╚█████╔╝██████╔╝╚█████╔╝██║ ╚███║
╚══════╝╚═╝     ╚═╝  ╚═╝╚═════╝    ╚═╝     ╚══════╝ ╚════╝ ╚═════╝  ╚════╝ ╚═╝  ╚══╝
Faster than a souped-up muscle car and more reliable than a family you can count on.

Convert csv to json in no time!")]
struct Args {
    /// CSV file to operate on. i.e., ./processes/me.csv
    source_file: String,
    /// JSON file path to save to. i.e., ./save/me/here.json
    output_file: String,
    /// CSV Delimiter 
    #[arg(short, long,default_value_t = ','.to_string())]
    delimiter: String,
}

fn main() {
    let start_time = Instant::now();
    let args = Args::parse();

    println!("In file {}",args.source_file);

    if let Err(e) = twojson::run(args.source_file, args.output_file, args.delimiter) {
        eprintln!("Application error; {e}");
        process::exit(1);
    }
    eprintln!("Elapsed: {} ms", get_elapsed_time(start_time));
}

fn get_elapsed_time(start_time: Instant) -> String {
    let x = start_time.elapsed();
    ((x.as_secs() * 1_000) + (x.subsec_millis()) as u64).to_string()
}