use std::fs::File;
use std::io::{BufReader, Error};

pub fn get_file_handle(filename: &str) -> Result<BufReader<File>, Error>
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file))
}
