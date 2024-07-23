//Reading a file
//using thiserror crate 

use rust_project2::controlers::{command_arg::get_file_paths, file_reader::read_file, write_file::write_file};
fn main() {
   let( input_path,output_path)=get_file_paths();
   match read_file(input_path){
      Ok(content)=>
     match  write_file(output_path,content){
      Ok(_)=>println!("File written"),
      Err(e)=>println!("error,{}",e)
     }
      ,
      Err(e)=> println!("error,{}",e)
  
   }
}
