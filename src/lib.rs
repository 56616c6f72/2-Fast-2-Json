use std::{error::Error, io::{BufWriter, Write}};
use csv::ReaderBuilder;
use serde_json::Map;
use std::fs::OpenOptions;
pub struct Config {
    pub file_path: String
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let rdr = ReaderBuilder::new()
        .flexible(true)
        .has_headers(false)
        .from_path(config.file_path)?;

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("../twojson-file.json")?;

    let mut file_buffer = BufWriter::with_capacity(26214400,file);
  

    let mut it = rdr.into_records();
    let headers = it.next().unwrap().unwrap();
    let mut json_map = Map::new();


    for line in it {
        
        for (i, value)in line.unwrap().iter().enumerate(){
               // println!("{:?}", value);
                json_map.insert(headers.get(i).unwrap().to_string(), serde_json::from_str(value).unwrap_or_else(|_|value.into()));
            }
        
        writeln!(file_buffer,"{}", serde_json::to_string(&json_map).unwrap()).expect("Buffer store no good.");
    };

    file_buffer.flush().expect("buffer write no good.");
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

