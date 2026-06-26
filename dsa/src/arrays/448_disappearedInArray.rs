impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        /*
            Problem:
            nums contains numbers from 1 to n.

            Some numbers appear twice.
            Some numbers are missing.

            We need to find the missing numbers.

            Example:
            nums = [4,3,2,7,8,2,3,1]

            Numbers present:
            1,2,3,4,7,8

            Missing:
            5,6

            ------------------------------------------------

            Key Idea:

            Since every number is between 1 and n,
            we can use the array itself to mark
            which numbers have appeared.

            How?

            If we see number x,
            go to index x-1
            and make that value NEGATIVE.

            Negative means:
            "This number exists."

            ------------------------------------------------

            Example:

            nums = [4,3,2,7,8,2,3,1]

            Read 4
            Mark index 3 negative.

            Read 3
            Mark index 2 negative.

            Read 2
            Mark index 1 negative.

            ...

            After marking:

            [-4,-3,-2,-7,8,2,-3,-1]

            Positive values remain at:

            index 4
            index 5

            Missing numbers are:

            5
            6

            ------------------------------------------------

            Why abs()?

            Some values become negative while marking.

            Example:

            nums[i] = -7

            abs(-7) = 7

            We still want the original number.

            ------------------------------------------------

            Time Complexity:
            O(n)

            Space Complexity:
            O(1)
            (Returned answer doesn't count.)
        */

        let n = nums.len();

        /*
            First pass:
            Mark visited numbers by making
            their corresponding index negative.
        */
        for i in 0..n {

            // Current number (ignore sign)
            let value = nums[i].abs();

            // Convert number into array index
            let index = (value - 1) as usize;

            /*
                If value at this index is positive,
                make it negative.

                If already negative,
                don't change it again.
            */
            if nums[index] > 0 {
                nums[index] = -nums[index];
            }
        }

        // Store missing numbers
        let mut answer = Vec::new();

        /*
            Second pass:

            Positive value means
            that index was never visited.
        */
        for i in 0..n {

            if nums[i] > 0 {

                // Index 4 -> Number 5
                answer.push((i + 1) as i32);
            }
        }

        answer
    }
}