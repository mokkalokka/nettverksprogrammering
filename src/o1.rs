use std::thread;
use std::sync::{Arc, Mutex};


pub fn run() {
    //Settings
    const LOWER_LIMIT: u32 = 2;
    const UPPER_LIMIT: u32 = 1000;
    const NUMBER_OF_THREADS: u32 = 100;

    println!("Running prime search with these settings: \n\
     Lower limit: {} \n\
      Upper Limit: {}\n\
       Number of threads: {}\n", LOWER_LIMIT, UPPER_LIMIT, NUMBER_OF_THREADS);

    //Counter
    let count = Arc::new(Mutex::new(LOWER_LIMIT));

    let mut threads = vec![];

    for thread_id in 0..NUMBER_OF_THREADS + 1 {
        let count = Arc::clone(&count);
        threads.push(thread::spawn(move || {

            // thread_id & count is copied into thread (Closure move)
            while *count.lock().unwrap() < UPPER_LIMIT {
                let mut count = count.lock().unwrap();

                if (*count + 1) % 2 == 0 { *count += 2 } else { *count += 1 }
                check_if_prime(*count, thread_id);
            }

        }));
    }


    for thread in threads {
        //Join all threads
        thread.join().unwrap();
    }
}

fn check_if_prime(num: u32, thread_id: u32) {
    let mut is_prime: bool = true;

    for i in 2..num {
        if num % i == 0 {
            is_prime = false;
            break;
        }
    }
    if is_prime {
        println!("Thread: {} found, Prime: {}", thread_id, num)
    }
}












