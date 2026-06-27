impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        /*
            Problem:
            Find the maximum number of consecutive 1's.

            Consecutive means:
            One after another without interruption.

            ------------------------------------------------

            Example:

            nums = [1,1,0,1,1,1]

            Consecutive groups:

            1 1     -> length = 2

            1 1 1   -> length = 3

            Maximum = 3

            ------------------------------------------------

            Idea:

            Keep two variables:

            current -> current streak of 1's

            maximum -> longest streak found

            Traverse the array once.

            If current number is 1:
                Increase current streak.

            If current number is 0:
                Reset current streak to 0.

            After every 1,
            update the maximum streak.

            ------------------------------------------------

            Example:

            nums = [1,1,0,1,1,1]

            current = 0
            maximum = 0

            Read 1
            current = 1
            maximum = 1

            Read 1
            current = 2
            maximum = 2

            Read 0
            current = 0

            Read 1
            current = 1

            Read 1
            current = 2

            Read 1
            current = 3
            maximum = 3

            Answer = 3

            ------------------------------------------------

            Time Complexity:
            O(n)

            Space Complexity:
            O(1)
        */

        // Stores the current consecutive count of 1's
        let mut current = 0;

        // Stores the maximum consecutive count found
        let mut maximum = 0;

        // Traverse every element in the array
        for num in nums {

            // If current number is 1
            if num == 1 {

                // Increase current streak
                current += 1;

                /*
                    If current streak becomes larger
                    than previous maximum,
                    update maximum.
                */
                if current > maximum {
                    maximum = current;
                }

            } else {

                /*
                    Current number is 0.

                    Streak breaks,
                    so reset current count.
                */
                current = 0;
            }
        }

        // Return the longest streak
        maximum
    }
}