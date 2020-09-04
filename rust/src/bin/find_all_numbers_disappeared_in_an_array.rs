// Given an array of integers where 1 ≤ a[i] ≤ n (n = size of array), some
// elements appear twice and others appear once.
//
// Find all the elements of [1, n] inclusive that do not appear in this array.
//
// Could you do it without extra space and in O(n) runtime? You may assume the
// returned list does not count as extra space.
//
// Example:
//
// Input:
// [4,3,2,7,8,2,3,1]
//
// Output:
// [5,6]

fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut disappeared_numbers_mapping = vec![false; nums.len()];
    for number in nums {
        disappeared_numbers_mapping[(number - 1) as usize] = true;
    }

    let mut disappeared_numbers: Vec<i32> = Vec::new();
    for i in 0..disappeared_numbers_mapping.len() {
        if disappeared_numbers_mapping[i] == false {
            disappeared_numbers.push((i + 1) as i32);
        }
    }

    disappeared_numbers
}

fn main() {
    let input = vec![4, 3, 2, 7, 8, 2, 3, 1];
    let disappeared_numbers = find_disappeared_numbers(input);
    println!("{:?}", disappeared_numbers);
}
