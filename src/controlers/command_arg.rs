use std::{env,path:: PathBuf};

pub fn get_file_paths()->(PathBuf,PathBuf){
    let args: Vec<String>=env::args().collect();

let input_file: PathBuf=if args.len() >1{
    PathBuf::from(&args[1])
}else{
   env::current_dir().unwrap().join("file.txt")
};
let outpu_file:PathBuf=if args.len() > 2{
    PathBuf::from(&args[2])
}else{
 env::current_dir().unwrap().join("new.txt")
};
(input_file,outpu_file)
}