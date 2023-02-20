use std::error::Error;
use csv::ReaderBuilder;
//use std::{thread, time::Duration};
use serde_json::{Map, Value};
// use std::str::from_utf8;

pub struct Config {
    pub file_path: String
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
   // let contents = fs::read_to_string(config.file_path)?;
    let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .from_path(config.file_path)?;

    // Cloning headers so I can access later. Probably this is not the best way of doing this. Reference is not implemented for iterators...
    let header = rdr.headers()?.clone();
    let mut json_map = Map::new();

    rdr.records().for_each(|line|{
        let mut header_index = 0;
        for value in line
            .expect("Didn't get ByteRecord for a line.")
            .iter(){
            json_map.insert(header.get(header_index).unwrap().to_string(), Value::String(value.to_string()));
            header_index +=1;
        }
        println!("{}", serde_json::to_string(&json_map).unwrap());
      //  thread::sleep(Duration::from_millis(2000));
    });

    Ok(())
}

impl Config{
    pub fn build(mut args: impl Iterator<Item= String>) -> Result<Config, &'static str> {
        args.next();
    
        let file_path = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        Ok(Config {  
            file_path 
        })
    }
}

