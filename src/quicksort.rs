use std::thread;

pub fn quicksort(arr: Vec<u32>) -> Vec<u32> {
    if arr.len() > 1 {
        let pivot = arr[arr.len() / 2];
        let mut left: Vec<u32> = vec![];
        let mut right: Vec<u32> = vec![];

        for i in arr[1..arr.len()].iter() {
            if i > &pivot {
                left.push(i.to_owned());
            } else {
                right.push(i.to_owned());
            }
        }

        let mut sorted: Vec<u32> = Vec::new();

        if left.len() >= 15000 && right.len() >= 15000 {
            let left_handle = thread::spawn(move || quicksort(left));
            let right_handle = thread::spawn(move || quicksort(right));

            sorted.append(&mut left_handle.join().unwrap());
            sorted.append(&mut vec![pivot]);
            sorted.append(&mut right_handle.join().unwrap());
        } else {
            sorted.append(&mut quicksort(left));
            sorted.append(&mut vec![pivot]);
            sorted.append(&mut quicksort(right));
        }

        sorted
    } else {
        arr
    }
}
