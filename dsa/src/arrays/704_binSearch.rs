impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        /*
            Problem:
            Search for target in a SORTED array.

            Return its index if found.
            Otherwise return -1.

            ------------------------------------------------

            Since the array is sorted,
            we use Binary Search.

            Binary Search repeatedly cuts
            the search space into HALF.

            ------------------------------------------------

            Example:

            nums = [-1,0,3,5,9,12]
            target = 9

            left = 0
            right = 5

            Middle = 2
            nums[2] = 3

            3 < 9

            Target must be on the RIGHT.

            Move left.

            left = 3
            right = 5

            Middle = 4
            nums[4] = 9

            Found!

            Return 4.

            ------------------------------------------------

            Rules:

            If nums[mid] == target
                Return mid

            If nums[mid] < target
                Search RIGHT half

            If nums[mid] > target
                Search LEFT half

            ------------------------------------------------

            Time Complexity:
            O(log n)

            Space Complexity:
            O(1)
        */

        // Left pointer starts at first index
        let mut left = 0;

        // Right pointer starts at last index
        let mut right = nums.len() as i32 - 1;

        /*
            Continue while search space exists.
        */
        while left <= right {

            /*
                Find middle index.

                Formula:
                left + (right - left) / 2

                This avoids overflow in some languages.
            */
            let mid = left + (right - left) / 2;

            // Get middle element
            let value = nums[mid as usize];

            // Target found
            if value == target {
                return mid;
            }

            /*
                Target is larger.

                Ignore left half.
            */
            else if value < target {
                left = mid + 1;
            }

            /*
                Target is smaller.

                Ignore right half.
            */
            else {
                right = mid - 1;
            }
        }

      
        -1
    }
}