use std::sync::{Arc, Mutex, Condvar};
use std::{thread, time};
use std::collections::VecDeque;

pub fn run() {
    fn task() { println!("Task Executed") };

    let mut worker_threads = Workers::new(4);
    let mut event_loop = Workers::new(1);

    worker_threads.start();
    event_loop.start();
    worker_threads.post_timeout(task, 5000);

    worker_threads.post(task);
    worker_threads.post(task);

    event_loop.post(task);
    event_loop.post(task);


    worker_threads.stop();
    event_loop.stop();


    while (*worker_threads.is_running.lock().unwrap() && *event_loop.is_running.lock().unwrap())
        || (*worker_threads.is_waiting.lock().unwrap() || *event_loop.is_waiting.lock().unwrap()) {}
    println!("Main thread shutting down!");
}

struct Workers {
    number_of_threads: u16,
    condvar_pair: Arc<(Mutex<bool>, Condvar)>,
    task_queue: Arc<Mutex<VecDeque<fn()>>>,
    is_running: Arc<Mutex<bool>>,
    is_waiting: Arc<Mutex<bool>>,
}

trait WorkerFunctions {
    fn new(number_of_threads: u16) -> Workers;
    fn start(&mut self);
    fn post(&self, task: fn());
    fn post_timeout(&self, task: fn(), millisec: u64);
    fn stop(&mut self);
}

impl WorkerFunctions for Workers {
    fn new(number_of_threads: u16) -> Workers {
        Workers {
            number_of_threads,
            condvar_pair: Arc::new((Mutex::new(false), Condvar::new())),
            task_queue: Arc::new(Mutex::new(VecDeque::new())),
            is_running: Arc::new(Mutex::new(true)),
            is_waiting: Arc::new(Mutex::new(false)),
        }
    }

    fn start(&mut self) {
        println!("started");
        let mut threads = vec!();

        for thread_id in 0..self.number_of_threads {
            let is_running = self.is_running.clone();
            let pair2 = self.condvar_pair.clone();
            let task_queue = Arc::clone(&self.task_queue);

            threads.push(thread::spawn(move || {
                let (lock, cvar) = &*pair2;

                while *is_running.lock().unwrap() {
                    {
                        {
                            // Wait for the thread to start up.
                            let mut started = lock.lock().unwrap();
                            while !*started {
                                println!("Thread {} is sleeping", thread_id);
                                started = cvar.wait(started).unwrap();
                            }
                        }
                    }

                    //Execute next task
                    if *is_running.lock().unwrap() {
                        let mut unlocked_tasks = task_queue.lock().unwrap();
                        if unlocked_tasks.len() > 0 {
                            let task = unlocked_tasks.pop_back().unwrap();
                            task();
                            let mut started = lock.lock().unwrap();
                            if unlocked_tasks.len() == 0 {
                                *started = false;
                            }
                        }
                    } else { break; };
                }
            }));
        }
    }

    fn post(&self, task: fn()) {
        println!("Post");
        let mut unlocked_tasks = self.task_queue.lock().unwrap();
        unlocked_tasks.push_front(task);

        let (lock, cvar) = &*self.condvar_pair;
        let mut started = lock.lock().unwrap();
        *started = true;
        // We notify the condvar that the value has changed.
        cvar.notify_one();
    }

    fn post_timeout(&self, task: fn(), millisec: u64) {
        println!("Post with timeout {}ms", millisec);
        let is_waiting = self.is_waiting.clone();
        thread::spawn(move || {
            {
                *is_waiting.lock().unwrap() = true;
            }
            thread::sleep(time::Duration::from_millis(millisec));
            task();
            *is_waiting.lock().unwrap() = false;
        });
    }

    fn stop(&mut self) {
        loop {
            if self.task_queue.lock().unwrap().len() == 0 {
                println!("Stop!");
                *self.is_running.lock().unwrap() = false;

                let (lock, cvar) = &*self.condvar_pair;
                let mut started = lock.lock().unwrap();
                *started = true;

                // We notify the condvar that the value has changed.
                cvar.notify_all();
                break;
            }
        }
    }
}


