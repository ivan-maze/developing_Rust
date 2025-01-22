fn binary_search(array: &[i32], target: i32) -> Option<usize> {    
    let mut init: usize = 0;
    let mut end: usize = array.len();

    while init < end {
        let middle: usize = init + (end-init)/2;
        if array[middle] == target {
            return Some(middle);
        } else if array[middle] < target {
            init = middle + 1;
        } else {
            end = middle;
        }
    }
    None
}

fn main() {
    let sorted_list: [i32; 8] = [1, 3, 5, 7, 9, 11, 13, 15];
    let target: i32 = 7;

    match binary_search(&sorted_list, target) {
        Some(index) => println!("Element found at index: {}", index),
        None => println!("Element not found."),
    }
}