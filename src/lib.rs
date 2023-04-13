use std::io::{BufWriter, Write};
use csv::ReaderBuilder;
use serde_json::Map;
use std::fs::OpenOptions;
use anyhow::{Context, Result};

pub fn run(sfile: String,ofile: String,deli: String,tab_deli: bool) -> Result<(), anyhow::Error> {
    let tmp_delimiter: u8 = if tab_deli {
        b'\t'
    } else {
        deli.as_bytes()[0]
    };

    let csv_reader = ReaderBuilder::new()
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
    let mut csv_iterator = csv_reader.into_records();
    
    let headers = match csv_iterator.next() {
                                    Some(record) => match record { Ok(value) => value, _ => anyhow::bail!("Failed to read the first StringRecord(header)!")},
                                    _ =>    anyhow::bail!("Failed to read the CSV iterator!"),
                                };

    let mut json_map = Map::with_capacity(headers.len()); 
 
    for line in csv_iterator {
        for (i, value)in line.context("Failed to iterate over csv lines!")?.iter().enumerate(){
                if !headers.get(i).with_context(|| format!("Failed to read header index {}!", i))?.to_string().is_empty(){
                    json_map.insert(headers.get(i).with_context(|| format!("Failed to read header index {}!", i))?.to_string(), serde_json::from_str(value).unwrap_or_else(|_|value.into()));
                }
            }
        writeln!(file_buffer,"{}", serde_json::to_string(&json_map).context("Failed to convert json to string before writting to buffer.")?).context("Writing to file_buffer failed!")?;
        json_map.clear();
    };


    file_buffer.flush().context("Flushing file_buffer to disk failed!")?;
    Ok(())
}