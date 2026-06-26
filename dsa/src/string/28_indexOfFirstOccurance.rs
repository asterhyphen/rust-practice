impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        /*
            Problem:
            Find first occurrence of "needle" inside "haystack"

            Return starting index, else -1

            ------------------------------------------------
            Idea: Sliding window (simple comparison)

            Example:
            haystack = "sadbutsad"
            needle   = "sad"

            Check substrings of length = needle length:

            "sad"  -> match → return 0
            "adb"  -> no
            "dbu"  -> no
            ...

            ------------------------------------------------
            Steps:
            1. Convert strings to bytes for easy indexing
            2. Loop through haystack
            3. Compare substring with needle
            4. If match → return index

            Time Complexity:
            O(n * m)

            Space Complexity:
            O(1)
        */

        // Convert strings to byte arrays for indexing
        let h = haystack.as_bytes();
        let n = needle.as_bytes();

        let len_h = h.len();
        let len_n = n.len();

        /*
            Edge case:
            If needle is longer than haystack → impossible
        */
        if len_n > len_h {
            return -1;
        }

        /*
            Loop through haystack
            Only go till len_h - len_n
            because beyond that, needle can't fit
        */
        for i in 0..=len_h - len_n {

            /*
                Compare slice of haystack with needle
                h[i..i+len_n] means substring starting at i
            */
            if &h[i..i + len_n] == n {
                return i as i32;
            }
        }

        // If no match found
        -1
    }
}