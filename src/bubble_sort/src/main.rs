fn bubble_sort(array: Vec<i32>) -> Result<Vec<i32>, String> {
    if array.is_empty() {
        return Err(String::from("Error: the vector (array) is empty."));
    }
    let mut array_copy = array;
    let len = array_copy.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if array_copy[j] > array_copy[j + 1] {
                array_copy.swap(j, j + 1);
            }
        }
    }
    Ok(array_copy)
}

fn main() {
    let array = Vec::from([5i32, 3, 10, 7, 24, 13, 36, 0, 3]);
    println!("Before sorting: {array:?}");

    let sorted_array = bubble_sort(array);
    match sorted_array {
        Ok(val) => {
            let s = format!("{val:?}");
            println!("After sorting: {s}");
        }
        Err(e) => {
            print!("{e}");
        }
    }
}
