impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        /*
            Problem:
            Given a sorted array with unique numbers,
            group consecutive numbers into ranges.

            Rules:

            If range has one number:

                "7"

            If range has multiple numbers:

                "4->6"

            ------------------------------------------------

            Example:

            nums = [0,1,2,4,5,7]

            Consecutive numbers:

            0,1,2

            Range:

            "0->2"

            ---------------------

            Next:

            4,5

            Range:

            "4->5"

            ---------------------

            Last:

            7

            Range:

            "7"

            Answer:

            ["0->2","4->5","7"]

            ------------------------------------------------

            Key Idea:

            Every range has:

            start
            end

            Initially:

            start = current number

            Move forward while numbers
            remain consecutive.

            When sequence breaks,

            Save the range.

            Then start a new range.

            ------------------------------------------------

            Example:

            nums =

            [0,1,2,4,5,7]

             ↑

            start = 0

            Keep moving:

            1 consecutive

            2 consecutive

            4 breaks sequence

            Save:

            "0->2"

            Start again from 4.

            ------------------------------------------------

            Time Complexity:
            O(n)

            Space Complexity:
            O(1)
            (excluding answer vector)
        */

        // Stores all ranges
        let mut answer = Vec::new();

        // Current index
        let mut i = 0;

        /*
            Process every number.
        */
        while i < nums.len() {

            // Beginning of current range
            let start = nums[i];

            /*
                Move while consecutive numbers exist.

                Example:

                2 followed by 3

                3 == 2 + 1
            */
            while i + 1 < nums.len()
                && nums[i + 1] == nums[i] + 1
            {
                i += 1;
            }

            // Last number in current range
            let end = nums[i];

            /*
                One number only.

                Example:

                7

                Output:

                "7"
            */
            if start == end {
                answer.push(start.to_string());
            }

            /*
                Multiple numbers.

                Example:

                4

                6

                Output:

                "4->6"
            */
            else {
                answer.push(format!("{}->{}", start, end));
            }

            // Move to next range
            i += 1;
        }

        answer
    }
}