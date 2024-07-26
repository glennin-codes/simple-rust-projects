use std::{fs::File, io::{self, Read, Write}, path::Path, str::Bytes};

use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description:String,
    pub completed:bool,
    pub time:Time


}
#[derive(Debug,Serialize, Deserialize)]
pub struct TodoList{
   pub tasks:Vec<Task>

}
#[derive(Debug,Serialize, Deserialize)]
pub struct Time{
    pub id:u32,
    pub started: String,
    pub ended:String

}
pub trait Control{
 fn new()->Self;
 fn add_task(&mut self,description:String,title:String,time:Time);
 fn complete_task(&mut self,id:u32);
fn list_tasks(&self);
fn save_file(&self,file:&str)->io::Result<()>;
}

pub trait Loadable: Sized {
    fn load_from_file(file_name: &str) -> io::Result<Self>;
}
impl Loadable for TodoList{
    fn load_from_file(file_name:&str)->io::Result<Self> {
        let path=Path::new(file_name);
        if path.exists(){
            let mut file =File::open(path)?;
            let mut contents=String::new();
           file.read_to_string(&mut contents)?;
           let todo_list:TodoList=serde_json::from_str(&contents)?;
           Ok(todo_list);
    
        }esle{
            Ok(TodoList::new())
        }
}

impl Control for  TodoList{
     fn new()->Self{
    TodoList { tasks: Vec::new() }
    }
    
     fn add_task(&mut self,description:String,title:String,time:Time) {
        let id =self.tasks.len() as u32 +1 ;
       self.tasks.push(Task{
        id,
        description,
        title:title.clone(),
        time,
        completed:false
     });
     println!("task :{} added succesfuly ",title)
    }
    

     fn complete_task(&mut self,id:u32) {
       if let Some(task)= self.tasks.iter_mut().find(|t| t.id == id){
          task.completed=true;
          println!("task was completed successfuly")
       }else {
           println!("task not found");
       }
     }
     fn list_tasks(&self) {
         for task in &self.tasks{
            println!(
            "{}. {} {} [{}]",
            task.id,
            task.title,
            task.description,
            if task.completed {"X"} else {""}

            )
         }
     }
     fn save_file(&self,file_name:&str)->io::Result<()> {
         let json=serde_json::to_string(self)?;
let mut  file=File::create(file_name)?;
file.write_all(json.as_bytes())?;
Ok(())

     }

}
    



