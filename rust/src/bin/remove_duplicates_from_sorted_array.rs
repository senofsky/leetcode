// Given a sorted array nums, remove the duplicates in-place such that each
// element appear only once and return the new length.
//
// Do not allocate extra space for another array, you must do this by modifying
// the input array in-place with O(1) extra memory.
//
// Example 1:
//
// Given nums = [1,1,2],
//
// Your function should return length = 2, with the first two elements of nums
// being 1 and 2 respectively.
//
// It doesn't matter what you leave beyond the returned length.
//
// Example 2:
//
// Given nums = [0,0,1,1,1,2,2,3,3,4],
//
// Your function should return length = 5, with the first five elements of nums
// being modified to 0, 1, 2, 3, and 4 respectively.
//
// It doesn't matter what values are set beyond the returned length.
//
// Clarification:
//
// Confused why the returned value is an integer but your answer is an array?
//
// Note that the input array is passed in by reference, which means modification
// to the input array will be known to the caller as well.
//
// Internally you can think of this:
//
// // nums is passed in by reference. (i.e., without making a copy)
// int len = removeDuplicates(nums);
//
// // any modification to nums in your function would be known by the caller.
// // using the length returned by your function, it prints the first len
// // elements.
// for (int i = 0; i < len; i++) {
//     print(nums[i]);
// }

// TODO: Compare against other submissions
//     pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
//     if nums.is_empty() {
//         return 0;
//     }
//
//     let array_length = nums.len();
//     let mut new_index = 0;
//
//     for index in 0..array_length {
//         if nums[new_index] != nums[index] {
//             new_index += 1;
//             nums[new_index] = nums[index];
//         }
//     }
//
//     (new_index + 1) as i32

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let array_length = nums.len();
    let mut new_index = 1;

    for index in 1..array_length {
        if nums[index] != nums[index - 1] {
            nums[new_index] = nums[index];
            new_index += 1;
        }
    }

    new_index as i32
}

fn main() {
    let mut nums = vec![1, 1, 2];
    let new_length = remove_duplicates(&mut nums);
    println!("{:?}, length = {}", nums, new_length);

    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let new_length = remove_duplicates(&mut nums);
    println!("{:?}, length = {}", nums, new_length);
}
