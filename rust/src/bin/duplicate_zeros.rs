// Given a fixed length array arr of integers, duplicate each occurrence of
// zero, shifting the remaining elements to the right.
//
// Note that elements beyond the length of the original array are not written.
//
// Do the above modifications to the input array in place, do not return
// anything from your function.
//
// Example 1:
//
// Input: [1,0,2,3,0,4,5,0]
// Output: null
// Explanation: After calling your function, the input array is modified to:
// [1,0,0,2,3,0,0,4]
//
// Example 2:
//
// Input: [1,2,3]
// Output: null
// Explanation: After calling your function, the input array is modified to:
// [1,2,3]

fn duplicate_zeros(arr: &mut Vec<i32>) {
    let array_length = arr.len();
    let mut index = 0;

    while index < array_length {
        if arr[index] == 0 {
            arr.insert(index, 0);
            index += 2;
        } else {
            index += 1;
        }
    }
    arr.truncate(array_length);
}

fn main() {
    let mut array = vec![1, 0, 2, 3, 0, 4, 5, 0];
    duplicate_zeros(&mut array);
    println!("{:?}", array);

    let mut array = vec![1, 2, 3];
    duplicate_zeros(&mut array);
    println!("{:?}", array);
}
