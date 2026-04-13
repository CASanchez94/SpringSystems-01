use std::sync::{Arc, Mutex};
use std::thread;

fn main() {

    let cnt = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..5 {

        let cnt = cnt.clone();

        let thread = thread::spawn(move || {

            for j in 0..10 {
                *cnt.lock().unwrap() += 1;
                println!("Thread {} increment {}", i, j);
            }

        });

        handles.push(thread);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter: {}", *cnt.lock().unwrap());
}