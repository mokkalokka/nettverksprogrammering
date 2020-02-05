
/*
Øving 1:
Finn alle primtall mellom to gitte tall ved hjelp av et gitt
antall tråder.
Skriv til slutt ut en sortert liste av alle primtall som er funnet
Pass på at de ulike trådene får omtrent like mye arbeid
*/

use std::thread;
use std::sync::{Arc, Mutex};

pub fn run() {
    //Settings
    const LOWER_LIMIT: u32 = 0;
    const UPPER_LIMIT: u32 = 100;
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

                check_if_prime(*count, thread_id);
                if (*count + 1) % 2 == 0 && *count != 1 { *count += 2 } else { *count += 1 }
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

    for i in 3..num {
        if num == 2 {break}
        if num % i == 0 {
            is_prime = false;
            break;
        }
    }

    if num < 2 { is_prime = false;}

    if is_prime {
        println!("Thread: {} found, Prime: {}", thread_id, num)
    }
}












