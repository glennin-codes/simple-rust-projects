use std::{fs::File, io::Write, path::{Path, PathBuf}};

use crate::custom_errors::FileError;
pub fn write_file(path:PathBuf,content:String)->Result< & 'static str  ,FileError>{
    let path=Path::new(&path);
    
    let mut file=File::create(path)?;
    file.write(content.as_bytes())?;
    
    Ok("file was wrriten into successfuly!")
}
