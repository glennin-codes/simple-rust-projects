
use project_02::{Task, Time,TodoList,Control};
use serde::{Serialize, Deserialize};
use chrono::{self, DateTime, Duration, NaiveDateTime, Utc};

    fn main() {



















        let current: chrono::DateTime<Utc>=Utc::now();
        let current_time =current.format("%d/%m/%Y %H:%M");
        println!("current:{}",current_time);
     let later_time=current + Duration::hours(3);

     let task_a=Task{
        id:2,
        title:format!("coding"),
        description:String::from("ilove codding am gonna do it daily"),
        completed:false,
        time:Time{
            id:2,
            started:current_time.to_string(),
            ended:later_time.format("%d/%m/%Y %H:%M:%S").to_string()

        }
    };


        let time_str=&task_a.time.ended;
        let time_parsed = NaiveDateTime::parse_from_str(time_str, "%d/%m/%Y %H:%M:%S").unwrap();
        let time_utc=DateTime::<Utc>::from_utc(time_parsed, Utc);
        println!("time utc:{}",time_utc);
        //calculate 
        let complex_duration=Duration::days(3)
            + Duration::hours(5)
            + Duration::minutes(3)
            + Duration::seconds(20);

        let time_differ=time_utc - complex_duration;
        println!("time difference :{}",time_differ.format("%d/%m/%Y %H:%M:%S"));
     println!("naive date : {}",time_parsed);



let mut  list=TodoList::load_from_file("task.json")?;
// println!("list {:#?}",list);



TodoList::add_task(&mut list, String::from("ilove codding am gonna do it daily"), format!("coding"), Time{
    id:2,
    started:current_time.to_string(),
    ended:later_time.format("%d/%m/%Y %H:%M:%S").to_string()

});
TodoList::add_task(&mut list, String::from("ilove Reading am gonna do it daily"), format!("reading"), Time{
    id:2,
    started:current_time.to_string(),
    ended:later_time.format("%d/%m/%Y %H:%M:%S").to_string()

});
TodoList::add_task(&mut list, String::from("ilove going to the gym am gonna do it daily"), format!("gym"), Time{
    id:2,
    started:current_time.to_string(),
    ended:later_time.format("%d/%m/%Y %H:%M:%S").to_string()

});


TodoList::complete_task(&mut list, 1);
let serialised=serde_json::to_string_pretty(&list).unwrap();

println!("serialised= {}",serialised);
let saved = TodoList::save_file( &list, "tasks.json");
  match saved{
    Ok(()) => println!("saved succesfuly"),
    Err(e) => println!("and error was found while saving and writing file : {}",e),
  }

TodoList::list_tasks(&list);

    } 

