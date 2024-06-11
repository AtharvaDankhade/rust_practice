fn kth_smallest_element(arr: &[i32], k: usize) -> Option<i32> {
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();

    sorted_arr.get(k - 1).cloned()
}

fn main() {
    let arr = [7, 10, 4, 3, 20, 15];
    let k = 3;

    match kth_smallest_element(&arr, k) {
        Some(elem) => println!("The {}th smallest element is: {}", k, elem),
        None => println!("Array is empty or k is out of bounds"),
    }
}

