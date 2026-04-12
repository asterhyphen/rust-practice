use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        
        // Convert nums1 into a HashSet
        // This removes duplicate values automatically
        let set1: HashSet<i32> = nums1.into_iter().collect();

        // Convert nums2 into a HashSet
        let set2: HashSet<i32> = nums2.into_iter().collect();

        // Find common elements between set1 and set2
        // intersection() gives values present in both sets
        // cloned() converts references like &2 into actual values 2
        // collect() stores them into a Vec<i32>
        set1.intersection(&set2).cloned().collect()
    }
}