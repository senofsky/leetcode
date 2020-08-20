// Given an array A of integers, return true if and only if it is a valid
// mountain array.
//
// Recall that A is a mountain array if and only if:
//
//     A.length >= 3
//     There exists some i with 0 < i < A.length - 1 such that:
//         A[0] < A[1] < ... A[i-1] < A[i]
//         A[i] > A[i+1] > ... > A[A.length - 1]
// Example 1:
//
// Input: [2,1]
// Output: false
//
// Example 2:
//
// Input: [3,5,5]
// Output: false
//
// Example 3:
//
// Input: [0,3,2,1]
// Output: true

// TODO: This is incredibly ugly. Make this prettier and more functional.

fn valid_mountain_array(a: Vec<i32>) -> bool {
    if a.len() < 3 {
        return false;
    }

    let mut peak_found = false;
    let mut positive_difference_found = false;
    for i in 1..a.len() {
        let difference = a[i] - a[i - 1];
        if difference == 0 {
            return false;
        }
        if peak_found {
            if difference > 0 {
                return false;
            }
        } else {
            if difference < 0 {
                peak_found = true;
            } else {
                positive_difference_found = true;
            }
        }
    }

    if peak_found == false || positive_difference_found == false {
        return false;
    }

    true
}

fn main() {
    let a = vec![2, 1];
    let valid = valid_mountain_array(a);
    println!("{:?}", valid);

    let a = vec![3, 5, 5];
    let valid = valid_mountain_array(a);
    println!("{:?}", valid);

    let a = vec![0, 3, 2, 1];
    let valid = valid_mountain_array(a);
    println!("{:?}", valid);
}
