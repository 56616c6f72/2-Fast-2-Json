use std::process;
use std::time::Instant;
use clap::Parser;
use chrono::Utc;
use colored::Colorize;

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

Convert csv to json in no time, with minimal ram/memory usage!")]
struct Args {
    /// CSV file with headers to operate on. i.e., ./processes/me.csv 
    source_file: String,
    /// JSON file path to save to. i.e., ./save/me/here.json
    output_file: String,
    /// CSV Delimiter 
    #[arg(short, default_value_t = ','.to_string())]
    delimiter: String,
    /// Tab delimited CSV. If provided, overrides the -d, --delimiter flag [default: false]
    #[arg(short, default_value_t = false)]
    tab_delimited: bool
}

fn main() {
    let start_time = Instant::now();
    let args = Args::parse();
    let s_datetime = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f %z");
    
    if args.delimiter.len() > 1 {
        eprintln!("{} {}","[-] Delimiter cannot be multiple characters! You provided:".red(),args.delimiter);
        process::exit(1);
    }
    
    eprintln!("{} {}","[+] Start date and time:".green(), s_datetime);
    eprintln!("{} {}","[+] CSV file path:".green(),args.source_file);
    eprintln!("{} {}","[+] JSON file path to be created:".green(),args.output_file);
    if let Err(e) = twojson::run(args.source_file, args.output_file, args.delimiter, args.tab_delimited) {
        eprintln!("{} {e}","[-] Application error:".red());
        process::exit(1);
    }
    eprintln!("{} {} ms","[+] Processing completed in:".green(), get_elapsed_time(start_time));
    eprintln!("{} {}","[+] End date and time:".green(), Utc::now().format("%Y-%m-%d %H:%M:%S%.3f %z"));
}

fn get_elapsed_time(start_time: Instant) -> String {
    let x = start_time.elapsed();
    ((x.as_secs() * 1_000) + (x.subsec_millis()) as u64).to_string()
}