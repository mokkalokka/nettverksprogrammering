//use std::thread;

use std::sync::{Arc, Mutex};

pub fn run(){
    let t:  Vec<fn()> = vec!();
    let mut tasks = Arc::new(Mutex::new(t));

    fn task1(){println!("Task 1 Executed")}
    fn task2(){println!("Task 2 Executed")};


    fn post(tasks :&mut Arc<Mutex<Vec<fn()>>>, task: fn()){
        let mut unlocked_tasks = tasks.lock().unwrap();
        unlocked_tasks.push(task);

/*        let ut_iter = unlocked_tasks.iter();
        for t in ut_iter{
            t();
        }*/


    }

    post(&mut tasks, task1);
    post(&mut tasks, task2);

}