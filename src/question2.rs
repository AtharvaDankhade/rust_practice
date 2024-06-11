fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() as isize - 1;
    let mut result = None;

    while left <= right {
        let mid = left + (right - left) / 2;
        let mid_index = mid as usize;

        if arr[mid_index] == target {
            result = Some(mid_index);
            right = mid - 1;
        } else if arr[mid_index] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    result
}

fn main() {
    let arr = [1, 2, 2, 2, 3, 4, 5, 6];
    let target = 2;

    match find_first_occurrence(&arr, target) {
        Some(index) => println!("The first occurrence of {} is at index {}", target, index),
        None => println!("{} is not present in the array", target),
    }
}
