use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

pub fn sleepsort(arr: Vec<u32>) -> Vec<u32> {
    let sorted = Arc::new(Mutex::new(vec![]));
    let mut handles: Vec<_> = vec![];
    for &i in &arr {
        let sorted = Arc::clone(&sorted);
        let handle = thread::spawn(move || {
            thread::sleep(time::Duration::from_millis(i as u64));
            let mut sorting = sorted.lock().unwrap();
            sorting.push(i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap()
    }

    let mut done: Vec<u32> = vec![];

    for i in sorted.lock().unwrap().iter() {
        done.push(i.to_owned());
    }

    done
}
