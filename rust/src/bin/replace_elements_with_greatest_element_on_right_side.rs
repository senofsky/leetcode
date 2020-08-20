// Given an array arr, replace every element in that array with the greatest
// element among the elements to its right, and replace the last element with
// -1.
//
// After doing so, return the array.
//
// Example 1:
//
// Input: arr = [17,18,5,4,6,1]
// Output: [18,6,6,6,1,-1]

// TODO: This is incredibly ugly. Make this prettier and more functional.

fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
    let mut array = arr.clone();

    for i in 0..array.len() - 1 {
        array[i] = array[i + 1];
        for j in i + 2..array.len() {
            if array[j] > array[i] {
                array[i] = array[j]
            }
        }
    }
    let len = array.len() - 1;
    array[len] = -1;

    array
}

fn main() {
    let arr = vec![17, 18, 5, 4, 6, 1];
    let new_array = replace_elements(arr);
    println!("{:?}", new_array);
}
