impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        /*
            Problem:
            Generate the first num_rows of Pascal's Triangle.

            Example:

            numRows = 5

            Output:

            [
             [1],
             [1,1],
             [1,2,1],
             [1,3,3,1],
             [1,4,6,4,1]
            ]

            ------------------------------------------------

            Pascal's Triangle Rule:

            First element = 1
            Last element  = 1

            Middle element:

            current[j] =
            previous[j-1] + previous[j]

            ------------------------------------------------

            Example:

            Previous Row:

            [1,2,1]

            New Row:

            First = 1

            Middle:

            1+2 = 3
            2+1 = 3

            Last = 1

            Result:

            [1,3,3,1]

            ------------------------------------------------

            Idea:

            Keep storing every row
            inside the answer vector.

            For every new row:

            1. Create a row full of 1's.
            2. Calculate middle elements.
            3. Store it in answer.

            ------------------------------------------------

            Time Complexity:
            O(numRows²)

            Space Complexity:
            O(numRows²)
        */

        // Stores the complete Pascal Triangle
        let mut triangle: Vec<Vec<i32>> = Vec::new();

        /*
            Generate each row.

            Example:

            numRows = 5

            i:

            0
            1
            2
            3
            4
        */
        for i in 0..num_rows as usize {

            /*
                Create current row filled with 1's.

                Example:

                i = 3

                row becomes

                [1,1,1,1]
            */
            let mut row = vec![1; i + 1];

            /*
                Calculate only middle elements.

                First and last are already 1.
            */
            if i >= 2 {

                for j in 1..i {

                    /*
                        Previous row:

                        triangle[i-1]

                        Formula:

                        row[j] =
                        previous[j-1] + previous[j]
                    */
                    row[j] =
                        triangle[i - 1][j - 1]
                        + triangle[i - 1][j];
                }
            }

            // Store current row
            triangle.push(row);
        }

        // Return complete triangle
        triangle
    }
}