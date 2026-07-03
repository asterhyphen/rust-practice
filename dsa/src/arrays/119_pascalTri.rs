impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        /*
            Problem:
            Return the row_index-th row of Pascal's Triangle.

            Pascal's Triangle:

                    1
                  1   1
                1   2   1
              1   3   3   1
            1   4   6   4   1

            rowIndex starts from 0.

            rowIndex = 3

            Answer:
            [1,3,3,1]

            ------------------------------------------------

            Observation:

            First and last element of every row
            are always 1.

            Every middle element is:

            left_parent + right_parent

            Example:

                  1   2   1
                1   3   3   1

            3 = 1 + 2

            3 = 2 + 1

            ------------------------------------------------

            Idea:

            Start with first row:

            [1]

            Generate one row at a time.

            Every new row starts with all 1's.

            Then calculate the middle values.

            ------------------------------------------------

            Example:

            Start:

            [1]

            Row 1:

            [1,1]

            Row 2:

            [1,2,1]

            Row 3:

            [1,3,3,1]

            Return it.

            ------------------------------------------------

            Time Complexity:
            O(rowIndex²)

            Space Complexity:
            O(rowIndex)
        */

        // First row
        let mut row = vec![1];

        /*
            Generate rows until reaching row_index.

            If row_index = 3

            i = 1
            i = 2
            i = 3
        */
        for i in 1..=row_index as usize {

            /*
                Create a new row filled with 1's.

                Length of current row = i + 1

                Example:

                i = 3

                new_row = [1,1,1,1]
            */
            let mut new_row = vec![1; i + 1];

            /*
                Calculate only middle elements.

                First and last are already 1.
            */
            for j in 1..i {

                /*
                    Formula:

                    new_row[j] =
                    row[j-1] + row[j]
                */
                new_row[j] = row[j - 1] + row[j];
            }

            // Current row becomes previous row
            row = new_row;
        }

        // Return required row
        row
    }
}