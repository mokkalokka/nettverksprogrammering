/*
Øving 1:
Finn alle primtall mellom to gitte tall ved hjelp av et gitt
antall tråder.
Skriv til slutt ut en sortert liste av alle primtall som er funnet
Pass på at de ulike trådene får omtrent like mye arbeid
*/

use std::thread;
use std::sync::{Arc, Mutex};
use std::iter::Iterator;

pub fn run() {
    //Settings
    const LOWER_BOUND: u128 = 100_000_000_000_000;
    const UPPER_BOUND: u128 = 10_000_000_000_000_000_000;
    const NUMBER_OF_THREADS: u32 = 100;

    println!("PrimeFinder using Threads created by MokkaLokka\n");
    println!("Running prime search with these settings: \
    \nLower bound: {}\
    \nUpper bound: {}\
    \nNumber of threads: {}\n", LOWER_BOUND, UPPER_BOUND, NUMBER_OF_THREADS);

    //Mutex counter
    let count = Arc::new(Mutex::new(LOWER_BOUND));

    let mut threads = vec![];

    for thread_id in 0..NUMBER_OF_THREADS {
        let count = Arc::clone(&count);
        threads.push(thread::spawn(move || {
            // thread_id & count is copied into thread (Closure move)

            //Locks the global count variable and checks if it's prime, breaks when UPPER_BOUND is reached
            loop {
                let number_to_check;

                //Setting a small scope so the thread can lock, read and increment the mutex counter
                {
                    let mut count = count.lock().unwrap();
                    number_to_check = *count;
                    if (*count + 1) % 2 == 0 && *count != 1 { *count += 2 } else { *count += 1 }
                }

                if number_to_check < UPPER_BOUND {
                    check_if_prime(number_to_check, thread_id);
                } else { break; }
            }
        }));
    }

    //Join all threads
    for thread in threads {
        thread.join().unwrap();
    }
}

fn check_if_prime(num: u128, thread_id: u32) {
    //Finding the floor of sqrt(num) for optimization
    let num_sqrt = (num as f64).sqrt() as u128;

    // returns true if no numbers from 2 to num_sqrt  are divisible by num,
    // num > 2 or num equals 2
    let is_prime: bool = (2..num_sqrt)
        .all(|i| { num % i != 0 })
        && num > 1 || num == 2;

    //Prints the prime and the thread_id
    if is_prime {
        println!("Thread: {} found: {}", thread_id, num)
    }
}