//Reading a file
//using thiserror crate 

use rust_project2::controlers::{file_reader::read_file, write_file::write_file};
fn main() {
   match read_file("/home/glen/Desktop/rust_project2/file.tsx"){
      Ok(content)=>
     match  write_file("new.jsx",content){
      Ok(_)=>println!("File written"),
      Err(e)=>println!("error,{}",e)
     }
      ,
      Err(e)=> println!("error,{}",e)
  
   }
}
