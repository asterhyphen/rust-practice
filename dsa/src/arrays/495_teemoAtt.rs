impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        /*
            Problem:
            Every attack poisons Ashe for "duration" seconds.

            Poison interval:

            [attack_time, attack_time + duration - 1]

            If another attack happens BEFORE poison ends,
            the timer resets.

            We need the TOTAL poisoned time.

            ------------------------------------------------

            Example 1:

            timeSeries = [1,4]
            duration = 2

            Attack at 1:

            Poison:
            [1,2]

            Attack at 4:

            Poison:
            [4,5]

            They do NOT overlap.

            Total:

            2 + 2 = 4

            ------------------------------------------------

            Example 2:

            timeSeries = [1,2]
            duration = 2

            Attack at 1:

            [1,2]

            Attack at 2:

            [2,3]

            Notice:

            Second attack starts BEFORE
            first poison completely ends.

            We DO NOT add another full duration.

            Only one NEW second (3) is added.

            Total:

            3 seconds

            ------------------------------------------------

            Key Idea:

            Look at consecutive attacks.

            gap = current_attack - previous_attack

            If gap >= duration

                No overlap.

                Add full duration.

            Else

                Overlap exists.

                Only add the gap.

            Finally,
            add duration for the LAST attack.

            ------------------------------------------------

            Time Complexity:
            O(n)

            Space Complexity:
            O(1)
        */

        // If no duration, poison lasts 0 seconds
        if duration == 0 {
            return 0;
        }

        // Total poisoned time
        let mut total = 0;

        /*
            Compare every attack
            with the next attack.
        */
        for i in 0..time_series.len() - 1 {

            // Time difference between attacks
            let gap = time_series[i + 1] - time_series[i];

            /*
                If gap is smaller than duration,
                poison overlaps.

                Add only the gap.

                Otherwise,
                add full duration.
            */
            total += gap.min(duration);
        }

        /*
            Last attack always contributes
            full duration.
        */
        total + duration
    }
}