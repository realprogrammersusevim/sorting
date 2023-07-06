pub fn bubblesort(arr: Vec<u32>) -> Vec<u32> {
    let mut arr = arr;
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 1..arr.len() {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                swapped = true;
            }
        }
    }
    arr
}
