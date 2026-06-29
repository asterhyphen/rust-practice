impl Solution {
    pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
        /*
            Problem:
            Pair up the numbers such that
            the sum of the smaller number
            in each pair is maximum.

            ------------------------------------------------

            Example:

            nums = [1,4,3,2]

            After sorting:

            [1,2,3,4]

            Pair them as:

            (1,2)
            (3,4)

            Smaller numbers are:

            1
            3

            Sum = 4

            ------------------------------------------------

            Why sort?

            Suppose:

            [1,2,3,4]

            If we pair:

            (1,4)
            (2,3)

            Minimums:

            1 + 2 = 3

            But if we pair neighbours:

            (1,2)
            (3,4)

            Minimums:

            1 + 3 = 4

            This is always the maximum.

            ------------------------------------------------

            Algorithm:

            1. Sort the array.
            2. Pick every alternate element
               starting from index 0.
            3. Add them.

            ------------------------------------------------

            Example:

            nums = [6,2,6,5,1,2]

            Sorted:

            [1,2,2,5,6,6]

            Pairs:

            (1,2)
            (2,5)
            (6,6)

            Pick:

            1 + 2 + 6 = 9

            ------------------------------------------------

            Time Complexity:
            O(n log n)
            (sorting)

            Space Complexity:
            O(1)
        */

        // Sort numbers in ascending order
        nums.sort();

        // Store final answer
        let mut sum = 0;

        /*
            Visit every second element.

            Indices:
            0,2,4,6,...

            These are the smaller numbers
            in each pair.
        */
        for i in (0..nums.len()).step_by(2) {

            sum += nums[i];
        }

        sum
    }
}