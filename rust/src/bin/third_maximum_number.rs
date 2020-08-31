// Given a non-empty array of integers, return the third maximum number in this
// array. If it does not exist, return the maximum number. The time complexity
// must be in O(n).
//
// Example 1:
//
// Input: [3, 2, 1]
//
// Output: 1
//
// Explanation: The third maximum is 1.
//
// Example 2:
//
// Input: [1, 2]
//
// Output: 2
//
// Explanation: The third maximum does not exist, so the maximum (2) is returned
// instead.
//
// Example 3:
//
// Input: [2, 2, 3, 1]
//
// Output: 1
//
// Explanation: Note that the third maximum here means the third maximum
// distinct number. Both numbers with value 2 are both considered as second
// maximum.

// TODO: Solve without sort() and use a more efficient algorithm

fn third_max(nums: Vec<i32>) -> i32 {
    let mut cnums = nums.clone();
    cnums.sort();
    cnums.dedup();

    if cnums.len() < 3 {
        return cnums[cnums.len() - 1];
    }

    let mut first = cnums[0];
    let mut second = cnums[0];
    let mut third = cnums[0];
    for i in 0..cnums.len() {
        if cnums[i] > first {
            third = second;
            second = first;
            first = cnums[i];
        } else if cnums[i] > second {
            third = second;
            second = cnums[i];
        } else if cnums[i] > third {
            third = cnums[i];
        }
    }
    third
}

fn main() {
    let input = vec![3, 2, 1];
    let third_maximum_number = third_max(input);
    println!("{}", third_maximum_number);

    let input = vec![1, 1, 2];
    let third_maximum_number = third_max(input);
    println!("{}", third_maximum_number);

    let input = vec![2, 2, 3, 1];
    let third_maximum_number = third_max(input);
    println!("{}", third_maximum_number);
}
