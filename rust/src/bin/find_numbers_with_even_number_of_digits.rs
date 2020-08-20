// Given an array nums of integers, return how many of them contain an even
// number of digits.
//
// Example 1:
// Input: nums = [12,345,2,6,7896]
// Output: 2
// Explanation:
// 12 contains 2 digits (even number of digits).
// 345 contains 3 digits (odd number of digits).
// 2 contains 1 digit (odd number of digits).
// 6 contains 1 digit (odd number of digits).
// 7896 contains 4 digits (even number of digits).
// Therefore only 12 and 7896 contain an even number of digits.
//
// Example 2:
// Input: nums = [555,901,482,1771]
// Output: 1
// Explanation:
// Only 1771 contains an even number of digits.

// TODO: Benchmark "divide by 10" method against converting the number to a
// string and counting its length

fn number_of_digits_is_even(mut number: i32) -> bool {
    let mut number_of_digits = 1;
    loop {
        number /= 10;
        if number >= 1 {
            number_of_digits += 1;
        } else {
            break;
        }
    }
    number_of_digits % 2 == 0
}

fn find_numbers(nums: Vec<i32>) -> i32 {
    let number_of_even_digits = nums
        .into_iter()
        .filter(|n| number_of_digits_is_even(*n))
        .count();

    // Using `as` may silently fail - it is better to use TryFrom crate
    // https://stackoverflow.com/a/28280042/10967859
    number_of_even_digits as i32
}

fn main() {
    let input = vec![12, 345, 2, 6, 7896];
    let number_of_even_digits = find_numbers(input);
    println!("{}", number_of_even_digits);

    let input = vec![555, 901, 482, 1771];
    let number_of_even_digits = find_numbers(input);
    println!("{}", number_of_even_digits);
}
