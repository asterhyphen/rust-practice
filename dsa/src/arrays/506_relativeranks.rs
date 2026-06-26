impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        /*
            Problem:
            Given athlete scores, assign ranks based on highest score.

            Highest score  -> Gold Medal
            2nd highest    -> Silver Medal
            3rd highest    -> Bronze Medal
            Rest           -> rank number as string

            ------------------------------------------------
            Example:
            score = [10,3,8,9,4]

            Original indices:
            index:  0 1 2 3 4
            score: [10,3,8,9,4]

            Sorted descending:
            [(10,0), (9,3), (8,2), (4,4), (3,1)]

            Assign:
            index 0 -> Gold Medal
            index 3 -> Silver Medal
            index 2 -> Bronze Medal
            index 4 -> "4"
            index 1 -> "5"

            Final:
            ["Gold Medal","5","Bronze Medal","Silver Medal","4"]

            ------------------------------------------------
            Plan:
            1. Store each score with its original index
            2. Sort by score descending
            3. Assign rank strings
            4. Put results back into original positions

            Time Complexity:
            O(n log n) because of sorting

            Space Complexity:
            O(n)
        */

        // Pair each score with its original index
        let mut athletes: Vec<(i32, usize)> = score
            .iter()
            .enumerate()
            .map(|(i, &s)| (s, i))
            .collect();

        /*
            Sort descending by score
            Higher score should come first
        */
        athletes.sort_by(|a, b| b.0.cmp(&a.0));

        // Create answer array filled with empty strings
        let mut answer = vec![String::new(); score.len()];

        /*
            Go through sorted athletes
            rank starts from 0, so:
            rank 0 = 1st place
            rank 1 = 2nd place
            rank 2 = 3rd place
        */
        for (rank, &(_, index)) in athletes.iter().enumerate() {
            answer[index] = match rank {
                0 => "Gold Medal".to_string(),
                1 => "Silver Medal".to_string(),
                2 => "Bronze Medal".to_string(),
                _ => (rank + 1).to_string(),
            };
        }

        // Return final ranked list
        answer
    }
}