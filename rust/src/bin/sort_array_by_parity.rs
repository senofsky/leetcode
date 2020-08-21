// Given an array A of non-negative integers, return an array consisting of all
// the even elements of A, followed by all the odd elements of A.
//
// You may return any answer array that satisfies this condition.
//
// Example 1:
//
// Input: [3,1,2,4]
// Output: [2,4,3,1]
// The outputs [4,2,3,1], [2,4,1,3], and [4,2,1,3] would also be accepted.

// TODO: Can this be done more functionally?

fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
    let mut sorted_array: Vec<i32> = Vec::new();

    for n in a {
        if n % 2 == 0 {
            sorted_array.insert(0, n);
        } else {
            sorted_array.insert(sorted_array.len(), n)
        }
    }

    sorted_array
}

fn main() {
    let a = vec![3, 1, 2, 4];
    let sorted_array = sort_array_by_parity(a);
    println!("{:?}", sorted_array);
}
