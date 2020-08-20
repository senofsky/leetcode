// Given two sorted integer arrays nums1 and nums2, merge nums2 into nums1 as
// one sorted array.
//
// Note:
//
//     The number of elements initialized in nums1 and nums2 are m and n
//     respectively.
//     You may assume that nums1 has enough space (size that is equal to m + n)
//     to hold additional elements from nums2.
//
// Example:
//
// Input:
// nums1 = [1,2,3,0,0,0], m = 3
// nums2 = [2,5,6],       n = 3
//
// Output: [1,2,2,3,5,6]

fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    // TODO: Study Merge Sort Algorithm
}

fn main() {
    let mut nums1 = vec![1,2,3,0,0,0];
    let mut nums2 = vec![2,5,6];
    merge(&mut nums1, 3, &mut nums2, 3);
    println!("{:?}", nums1);
}
