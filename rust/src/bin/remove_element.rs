// Given an array nums and a value val, remove all instances of that value
// in-place and return the new length.
//
// Do not allocate extra space for another array, you must do this by modifying
// the input array in-place with O(1) extra memory.
//
// The order of elements can be changed. It doesn't matter what you leave beyond
// the new length.
//
// Example 1:
//
// Given nums = [3,2,2,3], val = 3,
//
// Your function should return length = 2, with the first two elements of nums
// being 2.
//
// It doesn't matter what you leave beyond the returned length.
//
// Example 2:
//
// Given nums = [0,1,2,2,3,0,4,2], val = 2,
//
// Your function should return length = 5, with the first five elements of nums
// containing 0, 1, 3, 0, and 4.
//
// Note that the order of those five elements can be arbitrary.
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
// int len = removeElement(nums, val);
//
// // any modification to nums in your function would be known by the caller.
// // using the length returned by your function, it prints the first
// // len elements.
// for (int i = 0; i < len; i++) {
//     print(nums[i]);
// }

fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let array_length = nums.len();
    let mut new_index = 0;

    for index in 0..array_length {
        if nums[index] != val {
            nums[new_index] = nums[index];
            new_index += 1;
        }
    }

    // Using `as` may silently fail - it is better to use TryFrom crate
    // https://stackoverflow.com/a/28280042/10967859
    new_index as i32
}

fn main() {
    let mut nums = vec![3, 2, 2, 3];
    let val = 3;
    let new_length = remove_element(&mut nums, val);
    println!("{:?}, length = {}", nums, new_length);

    let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let val = 2;
    let new_length = remove_element(&mut nums, val);
    println!("{:?}, length = {}", nums, new_length);
}
