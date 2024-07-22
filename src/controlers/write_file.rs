use std::{fs::File, io::Write, path::Path};

use crate::custom_errors::FileError;
pub fn write_file(path:&str,content:String)->Result<&str,FileError>{
    let mut file=File::create(path)?;
    file.write(content.as_bytes())?;
    
    Ok("file was wrriten into successfuly!")
}
