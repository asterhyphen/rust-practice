use std::collections::HashMap;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        /*
            Problem:
            Return the intersection of two arrays.

            Unlike "Intersection of Two Arrays I",
            duplicates ARE allowed.

            Example:

            nums1 = [1,2,2,1]
            nums2 = [2,2]

            Answer:

            [2,2]

            ------------------------------------------------

            Example:

            nums1 = [4,9,5]
            nums2 = [9,4,9,8,4]

            Common numbers:

            4
            9

            Answer:

            [4,9]

            ------------------------------------------------

            Key Idea:

            Count how many times each number
            appears in nums1.

            Then scan nums2.

            If a number still has a remaining count,
            add it to the answer
            and decrease its count.

            ------------------------------------------------

            Example:

            nums1:

            [1,2,2,1]

            Frequency Map:

            1 -> 2
            2 -> 2

            Scan nums2:

            [2,2]

            First 2

            Count = 2

            Answer:
            [2]

            Count becomes 1

            -------------------------

            Second 2

            Count = 1

            Answer:

            [2,2]

            Count becomes 0

            Finished.

            ------------------------------------------------

            Time Complexity:

            O(n + m)

            Space Complexity:

            O(n)
        */

        // Stores:
        // number -> frequency
        let mut map = HashMap::new();

        /*
            Count frequency of every number
            in nums1.
        */
        for num in nums1 {

            /*
                entry(num)

                If num exists,
                return its value.

                Otherwise create it.

                or_insert(0)

                Start count from 0.
            */
            *map.entry(num).or_insert(0) += 1;
        }

        // Store intersection
        let mut answer = Vec::new();

        /*
            Traverse nums2.

            If number exists in map
            and frequency > 0,

            it belongs in the answer.
        */
        for num in nums2 {

            /*
                get_mut()

                Returns mutable reference
                to the stored frequency.

                We use mutable because
                we need to decrease it.
            */
            if let Some(count) = map.get_mut(&num) {

                if *count > 0 {

                    answer.push(num);

                    // One occurrence used
                    *count -= 1;
                }
            }
        }

        answer
    }
}