// Given an array of integers A sorted in non-decreasing order, return an array
// of the squares of each number, also in sorted non-decreasing order.
//
// Example 1:
//
// Input: [-4,-1,0,3,10]
// Output: [0,1,9,16,100]
//
// Example 2:
//
// Input: [-7,-3,2,3,11]
// Output: [4,9,9,49,121]

// TODO: Implement sorting algorithm

fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
    let mut squares: Vec<i32> = a.into_iter().map(|n| n.pow(2)).collect();
    squares.sort();
    squares
}

fn main() {
    let input = vec![-4, -1, 0, 3, 10];
    let squares = sorted_squares(input);
    println!("{:?}", squares);

    let input = vec![-7, -3, 2, 3, 11];
    let squares = sorted_squares(input);
    println!("{:?}", squares);
}
