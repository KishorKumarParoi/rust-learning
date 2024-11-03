use std::thread;
use std::time::Duration;

fn main() {
    // Create a vector to hold the thread handles
    let mut handles = vec![];

    for i in 0..15 {
        // Create a new thread
        let handle = thread::spawn(move || {
            println!("Thread {} is running", i);
            thread::sleep(Duration::from_secs(1));
            println!("Thread {} has finished", i);
        });

        // Store the thread handle
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    for i in 0..5 {
        println!("Main thread is running {}", i);
        thread::sleep(Duration::from_secs(1));
    }

    println!("All threads have finished execution.");
}
