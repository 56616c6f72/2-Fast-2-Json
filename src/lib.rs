use std::{error::Error, io::{BufWriter, Write}};
use csv::ReaderBuilder;
use serde_json::Map;
use std::fs::OpenOptions;


pub fn run(sfile: String,ofile: String,deli: String,tab_deli: bool) -> Result<(), Box<dyn Error>> {
    let tmp_delimiter: u8;
    if tab_deli {
        tmp_delimiter = b'\t';
    } else {
        tmp_delimiter = deli.as_bytes()[0];
    }

    let rdr = ReaderBuilder::new()
    .flexible(true)
    .has_headers(false)
    .delimiter(tmp_delimiter)
    .from_path(sfile)?;

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(ofile)?;

    let mut file_buffer = BufWriter::with_capacity(26214400,file);
  
    let thread_pool = rayon::ThreadPoolBuilder::new()
        .build()
        .unwrap();

    let mut it = rdr.into_records();
    let headers = it.next().unwrap().unwrap();
    let mut json_map = Map::with_capacity(headers.len()); 
 
    thread_pool.install(|| {
    for line in it {

        for (i, value)in line.unwrap().iter().enumerate(){
                json_map.insert(headers.get(i).unwrap().to_string(), serde_json::from_str(value).unwrap_or_else(|_|value.into()));
            }
        writeln!(file_buffer,"{}", serde_json::to_string(&json_map).unwrap()).expect("Buffer store no good.");
        json_map.clear();
      };
    });

    file_buffer.flush().expect("buffer write no good.");
    Ok(())
}