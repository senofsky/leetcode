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
    let length = (m + n) as usize;
    for i in 0..n as usize {
        if nums1[i] >= nums2[i] {
            for j in (i+1..length).rev() {
                nums1[j] = nums1[j - 1];
            }
            nums1[i] = nums2[i];
        } else {
            for j in i..length {
                if nums1[j] >= nums2[i] {
                    for z in (j+1..length).rev() {
                        nums1[z] = nums1[z - 1];
                    }
                    nums1[j] = nums2[i];
                    break;
                } else if j == i + m as usize {
                    nums1[j] = nums2[i];
                    break;
                }
            }
        }
    }
}

fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![0, 1, 2];
    merge(&mut nums1, 3, &mut nums2, 3);
    println!("{:?}", nums1);

    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    merge(&mut nums1, 3, &mut nums2, 3);
    println!("{:?}", nums1);

    let mut nums1 = vec![0, 0, 0, 0, 0];
    let mut nums2 = vec![1, 2, 3, 4, 5];
    merge(&mut nums1, 0, &mut nums2, 5);
    println!("{:?}", nums1);

    let mut nums1 = vec![-1, 3, 0, 0, 0, 0, 0];
    let mut nums2 = vec![0, 0, 1, 2, 3];
    merge(&mut nums1, 2, &mut nums2, 5);
    // expect [-1,0,0,1,2,3,3]
    println!("{:?}", nums1);
}
