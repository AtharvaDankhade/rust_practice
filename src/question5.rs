fn median(arr: &[i32]) -> i32 {
    let len = arr.len();
    if len == 0 {
        panic!("Array cannot be empty");
    }

    if len % 2 == 1 {
        arr[len / 2] 
    } else {
        let mid1 = arr[len / 2 - 1];
        let mid2 = arr[len / 2];
        (mid1 + mid2) / 2
    }
}

fn main() {
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = [1, 2, 3, 4, 5, 6];

    println!("Median of arr1: {}", median(&arr1));
    println!("Median of arr2: {}", median(&arr2));
}
