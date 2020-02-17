/*
use std::thread;

pub struct Workers {
    workers: Vec<Worker>,
}

impl Workers {
    pub fn new(size: usize) -> Workers {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        Workers {
            workers
        }
    }

    fn post(&self)  {
        println!("Posted")
    }

    fn start(){
        println!("Started!")
    }
}


struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker {
            id,
            thread,
        }
    }
}

pub fn run() {
    let worker_threads = Workers::new(4);
    worker_threads.post();
}




*//*

using namespace std;
list<function<void()>> tasks;
mutex tasks_mutex; // tasks mutex needed
void post_tasks() {
for (int i = 0; i < 10; i++) {
lock_guard<mutex> lock(tasks_mutex);
tasks.emplace_back([i] {
cout << "task " << i
<< " runs in thread "
<< this_thread::get_id()
<< endl;
});
}
}
void run_tasks_in_worker_threads() {
vector<thread> worker_threads;
for (int i = 0; i < 4; i++) {
worker_threads.emplace_back([] {
while (true) {
function<void()> task;
{
lock_guard<mutex> lock(tasks_mutex);
// TODO: use conditional variable
if (!tasks.empty()) {
task = *tasks.begin(); // Copy task for later use
tasks.pop_front(); // Remove task from list
}
}
if (task)
task(); // Run task outside of mutex lock
}
});
}
for (auto &thread : worker_threads)
thread.join();
}
int main() {
post_tasks();
run_tasks_in_worker_threads();
}

*//*


*//*
Workers worker_threads(4);
Workers event_loop(1);
worker_threads.start(); // Create 4 internal threads
event_loop.start(); // Create 1 internal thread
worker_threads.post([] {
// Task A
});
worker_threads.post([] {
// Task B
// Might run in parallel with task A
});
event_loop.post([] {
// Task C
// Might run in parallel with task A and B
});
event_loop.post([] {
// Task D
// Will run after task C
// Might run in parallel with task A and B
});*//*
*/
