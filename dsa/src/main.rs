fn missing_number(nums: Vec<i32>) -> i32 {
    // Get the length of the array.
    // If nums has length n, numbers should range from 0 to n.
    let n = nums.len() as i32;

    // Calculate expected sum of numbers from 0 to n.
    let expected_sum = n * (n + 1) / 2;

    // Calculate actual sum of elements in the array.
    let actual_sum: i32 = nums.iter().sum();

    // Missing number is the difference.
    expected_sum - actual_sum
}

fn main() {
    let nums = vec![3, 0, 1];
    let result = missing_number(nums);

    println!("Missing number: {}", result);
}