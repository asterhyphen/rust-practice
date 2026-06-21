impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        /*
            Problem:
            Each child has a greed factor.
            Each cookie has a size.

            A child is satisfied only if:

                cookie_size >= greed_factor

            Each cookie can be given to only one child.

            We must maximize the number of happy children.

            ------------------------------------------------

            Example:

            g = [1,2]
            s = [1,2,3]

            Child greed:
            1  2

            Cookies:
            1  2  3

            Answer = 2

            ------------------------------------------------

            Key Idea (Greedy):

            Give the SMALLEST cookie that can satisfy
            the SMALLEST greedy child.

            Why?

            If a small cookie can satisfy a child,
            don't waste a bigger cookie.

            Save bigger cookies for greedier children.

            ------------------------------------------------

            Step 1:
            Sort both arrays.

            Example:

            g = [2,1,3]
            becomes
            [1,2,3]

            s = [3,1,2]
            becomes
            [1,2,3]

            ------------------------------------------------

            Step 2:
            Use two pointers.

            child  -> current child
            cookie -> current cookie

            Compare:

            cookie >= child ?

            YES:
                Give cookie
                Move BOTH pointers

            NO:
                Cookie too small
                Try next larger cookie
                Move cookie pointer only

            ------------------------------------------------

            Example:

            g = [1,2,3]
            s = [1,1]

            child=1
            cookie=1

            1 >= 1 ✔
            happy = 1

            child=2
            cookie=1

            1 >= 2 ✘

            No more cookies

            Answer = 1

            ------------------------------------------------

            Time Complexity:
            O(n log n)
            (sorting)

            Space Complexity:
            O(1)
        */

        // Sort greed factors
        g.sort();

        // Sort cookie sizes
        s.sort();

        // Pointer for children
        let mut child = 0;

        // Pointer for cookies
        let mut cookie = 0;

        // Number of happy children
        let mut happy = 0;

        /*
            Continue while both arrays
            still have elements.
        */
        while child < g.len() && cookie < s.len() {

            /*
                If cookie can satisfy child,
                assign it.
            */
            if s[cookie] >= g[child] {

                happy += 1;

                child += 1;
                cookie += 1;
            }
            else {

                /*
                    Cookie too small.

                    Try a larger cookie.
                */
                cookie += 1;
            }
        }

        // Maximum satisfied children
        happy
    }
}