impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        /*
            Problem idea:
            Every number appears exactly twice except one number.

            We use XOR operation (^).

            XOR rules:
            ----------------
            a ^ a = 0
            a ^ 0 = a

            Example:
            nums = [4,1,2,1,2]

            Step by step:
            0 ^ 4 = 4
            4 ^ 1 = 5
            5 ^ 2 = 7
            7 ^ 1 = 6
            6 ^ 2 = 4

            Duplicate numbers cancel out:
            1 ^ 1 = 0
            2 ^ 2 = 0

            So only single number remains: 4

            Why this works:
            ----------------
            Since duplicates appear twice,
            each pair removes itself.
        */

        // result starts at 0
        let mut result = 0;

        /*
            Loop through each number in nums.

            for num in nums means:
            take each element one by one from array.
        */
        for num in nums {

            /*
                XOR current number into result.

                ^= means:
                result = result ^ num
            */
            result ^= num;
        }

        /*
            After all XOR operations,
            only unique number is left.
        */
        result
    }
}