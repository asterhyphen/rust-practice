use std::collections::HashSet;

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        /*
            Problem:
            Return only those words that can be typed
            using letters from ONE keyboard row.

            Keyboard rows:

            Row 1 : qwertyuiop
            Row 2 : asdfghjkl
            Row 3 : zxcvbnm

            ------------------------------------------------

            Example:

            "Dad"

            Lowercase:
            "dad"

            d -> Row 2
            a -> Row 2
            d -> Row 2

            Same row ✔

            Keep it.

            ------------------------------------------------

            Example:

            "Hello"

            Lowercase:
            "hello"

            h -> Row 2
            e -> Row 1

            Different rows ✘

            Ignore it.

            ------------------------------------------------

            Idea:

            1. Store each keyboard row in a HashSet.
            2. Convert each word to lowercase.
            3. Check if ALL characters belong to
               row1 OR row2 OR row3.
            4. If yes, add the original word
               to the answer.

            ------------------------------------------------

            Time Complexity:
            O(total characters)

            Space Complexity:
            O(1)
        */

        // Store all letters of first row
        let row1: HashSet<char> = "qwertyuiop".chars().collect();

        // Store all letters of second row
        let row2: HashSet<char> = "asdfghjkl".chars().collect();

        // Store all letters of third row
        let row3: HashSet<char> = "zxcvbnm".chars().collect();

        // Store valid words
        let mut answer = Vec::new();

        // Check every word
        for word in words {

            // Convert to lowercase
            let lower = word.to_lowercase();

            /*
                all() returns true only if
                every character satisfies
                the given condition.
            */
            if lower.chars().all(|c| row1.contains(&c))
                || lower.chars().all(|c| row2.contains(&c))
                || lower.chars().all(|c| row3.contains(&c))
            {
                answer.push(word);
            }
        }

        answer
    }
}