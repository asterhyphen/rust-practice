use std::vec::Vec;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        /*
            Problem:
            For every number in nums1,
            find its next greater element in nums2.

            Next greater means:
            - It must be on the RIGHT.
            - It must be the FIRST number larger than it.

            ------------------------------------------------

            Example:

            nums1 = [2,4]
            nums2 = [1,2,3,4]

            For 2:
                Right side = [3,4]
                First greater = 3

            For 4:
                Right side = []
                No greater number

            Answer = [3,-1]

            ------------------------------------------------

            Idea (Brute Force):

            For each number in nums1:

            1. Find it inside nums2.
            2. Move to the right.
            3. Stop at the first greater number.
            4. If none exists, answer is -1.

            ------------------------------------------------

            Time Complexity:
            O(n × m)

            Space Complexity:
            O(1)
            (Ignoring the answer vector)
        */

        // Store final answers
        let mut answer = Vec::new();

        // Check every number in nums1
        for num in nums1 {

            // Default answer is -1
            let mut next = -1;

            // Find where 'num' exists in nums2
            for i in 0..nums2.len() {

                if nums2[i] == num {

                    /*
                        Look only to the RIGHT
                        of the current number.
                    */
                    for j in i + 1..nums2.len() {

                        // First greater number found
                        if nums2[j] > num {
                            next = nums2[j];
                            break;
                        }
                    }

                    // No need to search further
                    break;
                }
            }

            // Save answer for this number
            answer.push(next);
        }

        answer
    }
}