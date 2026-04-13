impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        /*
            Problem:
            Every number appears 3 times except one number,
            which appears exactly once.

            Example:
            [2,2,3,2]

            Answer = 3

            ------------------------------------------------
            Main Idea: Count bits position by position
            ------------------------------------------------

            Every integer is made of bits (0s and 1s).

            Example:
            2 = 0010
            3 = 0011

            If we count how many times each bit is set (1),
            then:

            - Bits belonging to repeated numbers appear 3 times
            - Their count becomes divisible by 3

            Only the unique number leaves remainder.

            ------------------------------------------------
            Example:
            nums = [2,2,3,2]

            Binary:
            2 = 0010
            2 = 0010
            3 = 0011
            2 = 0010

            Count each bit:
            bit 0: 1 → remainder 1
            bit 1: 4 → remainder 1
            bit 2+: 0

            Rebuild number:
            0011 = 3

            ------------------------------------------------
            Time Complexity:
            O(32 * n) = O(n)

            Space Complexity:
            O(1)
        */

        let mut result = 0;

        /*
            Check all 32 bit positions in integer
            i = current bit position
        */
        for i in 0..32 {
            let mut count = 0;

            /*
                Count how many numbers have ith bit set
            */
            for &num in &nums {
                if ((num >> i) & 1) == 1 {
                    count += 1;
                }
            }

            /*
                If count % 3 != 0,
                unique number has this bit set
            */
            if count % 3 != 0 {
                result |= 1 << i;
            }
        }

        /*
            Final rebuilt unique number
        */
        result
    }
}