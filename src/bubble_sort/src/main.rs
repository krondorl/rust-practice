fn bubble_sort(array: Vec<i32>) -> Vec<i32> {
    let mut array_copy = array.clone();
    let len = array_copy.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if array_copy[j] > array_copy[j + 1] {
                let temp = array_copy[j];
                array_copy[j] = array_copy[j + 1];
                array_copy[j + 1] = temp;
            }
        }
    }
    array_copy
}

fn main() {
    let array = Vec::from([5i32, 3, 10, 7, 24, 13, 36, 0, 3]);
    println!("Before sorting: {:?}", array);

    let sorted_array = bubble_sort(array);
    let s = format!("{:?}", &sorted_array);
    println!("After sorting: {}", s);
}
