struct NumArray {
    // prefix[i] stores sum of elements from index 0 to i-1
    // i.e., prefix[0] = 0, prefix[1] = nums[0], prefix[2] = nums[0] + nums[1], etc.
    prefix: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        // Create prefix array of size n+1 to handle edge cases easily
        let mut prefix = vec![0; nums.len() + 1];

        // Build prefix sum array
        for i in 0..nums.len() {
            // prefix[i + 1] = sum of elements from nums[0] to nums[i]
            prefix[i + 1] = prefix[i] + nums[i];
        }

        NumArray { prefix }
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        // To get sum from index left to right:
        // subtract sum of elements before left from sum up to right
        // i.e., sum(left → right) = prefix[right+1] - prefix[left]
        
        self.prefix[right as usize + 1] - self.prefix[left as usize]
    }
}