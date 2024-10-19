// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];

    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // Lock the mutex before updating the shared value
            let mut job_status = status_shared.lock().unwrap();
            job_status.jobs_completed += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Since the print statement should be after all threads are joined,
    // we should print outside of the loop to reflect the final count.
    let completed_jobs = status.lock().unwrap().jobs_completed;
    println!("Total jobs completed: {}", completed_jobs);
    // Notice that with proper synchronization, the output should consistently be 10
}
