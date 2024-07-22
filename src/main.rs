//Reading a file
//using thiserror crate 

use rust_project2::controlers::file_reader::read_file;
fn main() {
   match read_file(""){
      Ok(content)=>println!("contnent: {:?}",content),
      Err(e)=> println!("error,{}",e)
  
   }
}
