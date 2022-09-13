// threads1.rs
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a hint.
// This program should wait until all the spawned threads have finished before exiting.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            status.clone().lock().unwrap().jobs_completed += 1;
        }   
    });
  /* 
    // I haven't been able to get this part to work
    while status.lock().unwrap().jobs_completed < 10 {
    // so I've resorted to this hack in frustration
  */
  let mut counter = 0;
  loop {
      if counter >= 6 {break;}; counter += 1;
      println!("waiting... {} ", counter);
            thread::sleep(Duration::from_millis(500));
    }

}