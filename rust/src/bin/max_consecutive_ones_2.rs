// Given a binary array, find the maximum number of consecutive 1s in this array
// if you can flip at most one 0.
//
// Example 1:
//
// Input: [1,0,1,1,0]
// Output: 4
// Explanation: Flip the first zero will get the the maximum number of
// consecutive 1s.
//     After flipping, the maximum number of consecutive 1s is 4.
//
// Note:
//
//     The input array will only contain 0 and 1. The length of input array is a
//     positive integer and will not exceed 10,000
//
// Follow up:
// What if the input numbers come in one by one as an infinite stream? In other
// words, you can't store all numbers coming from the stream as it's too large
// to hold in memory. Could you solve it efficiently?

// TODO: Refactor to make prettier and more effecient

fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut max_consecutive_ones = 0;
    let mut number_of_consecutive_ones = 0;
    let mut zeros: Vec<usize> = Vec::new();

    for i in 0..nums.len() {
        if nums[i] == 0 {
            zeros.push(i);
        }
    }

    if zeros.len() == 0 {
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
    } else {
        for index in zeros {
            for i in 0..nums.len() {
                if nums[i] == 1 || index == i {
                    number_of_consecutive_ones += 1;
                    if number_of_consecutive_ones > max_consecutive_ones {
                        max_consecutive_ones = number_of_consecutive_ones;
                    }
                } else {
                    number_of_consecutive_ones = 0;
                }
            }
        }
    }
    max_consecutive_ones
}

fn main() {
    let input = vec![1, 0, 1, 1, 0];
    let number_of_consecutive_ones = find_max_consecutive_ones(input);
    println!("{}", number_of_consecutive_ones);
}
