impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &Vec<i32>, n: i32) {
        /*
            Problem:
            Merge nums2 into nums1.

            nums1 has extra space at the end.

            Example:

            nums1 = [1,2,3,0,0,0]
                     |---m---|

            nums2 = [2,5,6]

            Result:
            [1,2,2,3,5,6]

            ------------------------------------------------

            Key Idea:

            Fill nums1 from the BACK.

            Why?

            If we start from the front, we'll overwrite
            values in nums1 that we still need.

            Example:

            nums1 = [1,2,3,0,0,0]
            nums2 = [2,5,6]

            Compare largest elements first:

            3 and 6

            Put 6 at the last position.

            Then compare:

            3 and 5

            Put 5.

            Then compare:

            3 and 2

            Put 3.

            Continue until done.

            ------------------------------------------------

            Pointers:

            i = last valid element in nums1
            j = last element in nums2
            k = last position in nums1

            Example:

            nums1 = [1,2,3,0,0,0]

            i = 2  (value 3)
            j = 2  (value 6)
            k = 5  (last index)

            ------------------------------------------------

            Time Complexity:
            O(m + n)

            Space Complexity:
            O(1)
        */

        // Last valid element in nums1
        let mut i = m - 1;

        // Last element in nums2
        let mut j = n - 1;

        // Last position available in nums1
        let mut k = m + n - 1;

        /*
            Keep going until all elements
            from nums2 are placed.

            If nums2 is exhausted,
            remaining nums1 elements are already correct.
        */
        while j >= 0 {

            /*
                If nums1 still has elements
                AND nums1[i] is bigger,
                place nums1[i] at position k.
            */
            if i >= 0 && nums1[i as usize] > nums2[j as usize] {

                nums1[k as usize] = nums1[i as usize];

                i -= 1;
            } else {

                /*
                    Otherwise place nums2[j]
                    at position k.
                */
                nums1[k as usize] = nums2[j as usize];

                j -= 1;
            }

            // Move to next position from the back
            k -= 1;
        }
    }
}