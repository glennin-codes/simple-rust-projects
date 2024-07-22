use std::{fs::File, io::Read, path::Path};

use crate::custom_errors::FileError;
pub fn read_file(path:&str)->Result<String,FileError>{
let path=Path::new(path);
if path.components().count() <1 || path.file_name().is_none(){
 
    return Err(FileError::InvalidInput(format!("{:?}",path.display())))
}

let mut file=File::open(path)?;
let mut contents=String::new();
let read=file.read_to_string(&mut contents)?;
println!("read size {:?}",read);
if contents.is_empty(){
    return Err(FileError::Empty)
}

Ok(contents)

}