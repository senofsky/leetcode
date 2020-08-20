// Given a binary array, find the maximum number of consecutive 1s in this array.
//
// Example 1:
//
// Input: [1,1,0,1,1,1]
// Output: 3
// Explanation: The first two digits or the last three digits are consecutive 1s.
//     The maximum number of consecutive 1s is 3.
//
// Note:
//
//     The input array will only contain 0 and 1.
//     The length of input array is a positive integer and will not exceed 10,000

pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut max_consecutive_ones = 0;
    let mut number_of_consecutive_ones = 0;
    for number in nums {
        if number == 1 {
            number_of_consecutive_ones += 1;
            if number_of_consecutive_ones > max_consecutive_ones {
                max_consecutive_ones = number_of_consecutive_ones;
            }
        } else {
            number_of_consecutive_ones = 0;
        }
    }
    max_consecutive_ones
}

fn main() {
    let input = vec![1, 1, 0, 1, 1, 1];
    let number_of_consecutive_ones = find_max_consecutive_ones(input);
    println!("{}", number_of_consecutive_ones);
}
