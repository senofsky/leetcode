// Given an array nums, write a function to move all 0's to the end of it while
// maintaining the relative order of the non-zero elements.
//
// Example:
//
// Input: [0,1,0,3,12]
// Output: [1,3,12,0,0]
//
// Note:
//
//     You must do this in-place without making a copy of the array.
//     Minimize the total number of operations.

// TODO: Can use a similar method to remove_element.rs where we skip
// zeroes while indexing. Check if that requires less operations.

fn move_zeroes(nums: &mut Vec<i32>) {
    for i in 0..nums.len() {
        if nums[i] == 0 {
            for j in i + 1..nums.len() {
                if nums[j] != 0 {
                    nums[i] = nums[j];
                    nums[j] = 0;
                    break;
                }
            }
        }
    }
}

fn main() {
    let mut nums = vec![0, 1, 0, 3, 12];
    move_zeroes(&mut nums);
    println!("{:?}", nums);
}
