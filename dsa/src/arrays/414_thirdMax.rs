impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        /*
            Problem:
            Return the 3rd DISTINCT maximum number.

            If there are less than 3 distinct numbers,
            return the maximum number.

            Example 1:
            nums = [3,2,1]

            Distinct numbers:
            3,2,1

            1st max = 3
            2nd max = 2
            3rd max = 1

            Answer = 1

            ------------------------------------------------

            Example 2:

            nums = [2,2,3,1]

            Distinct numbers are:
            3,2,1

            Duplicate 2 is ignored.

            Answer = 1

            ------------------------------------------------

            Idea:

            Instead of sorting,
            keep track of only the top 3 distinct numbers.

            We'll store:

            first  -> largest number
            second -> second largest
            third  -> third largest

            ------------------------------------------------

            We use Option<i32> because initially
            we don't know any maximums.

            None  -> no value yet
            Some(x) -> value exists

            ------------------------------------------------

            Time Complexity:
            O(n)

            Space Complexity:
            O(1)
        */

        // Store top 3 distinct maximum numbers
        let mut first: Option<i32> = None;
        let mut second: Option<i32> = None;
        let mut third: Option<i32> = None;

        // Visit every number once
        for num in nums {

            // Ignore duplicates
            if first == Some(num) || second == Some(num) || third == Some(num) {
                continue;
            }

            /*
                If current number becomes the largest:

                Shift:
                first -> second
                second -> third
            */
            if first.is_none() || num > first.unwrap() {
                third = second;
                second = first;
                first = Some(num);
            }

            /*
                Else if current number becomes
                second largest.
            */
            else if second.is_none() || num > second.unwrap() {
                third = second;
                second = Some(num);
            }

            /*
                Else if current number becomes
                third largest.
            */
            else if third.is_none() || num > third.unwrap() {
                third = Some(num);
            }
        }

        /*
            If third maximum exists,
            return it.

            Otherwise return first maximum.
        */
        if let Some(value) = third {
            value
        } else {
            first.unwrap()
        }
    }
}