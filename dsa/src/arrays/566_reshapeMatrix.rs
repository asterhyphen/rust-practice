impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        /*
            Problem:
            Change the shape of the matrix while keeping
            all elements in the SAME order.

            Example:

            Original:

            1 2
            3 4

            r = 1
            c = 4

            Result:

            1 2 3 4

            ---------------------------------------------

            Important Condition:

            Reshaping is possible ONLY IF

                total elements remain same

            Example:

            2 x 2 = 4 elements

            Can become:
            1 x 4
            4 x 1
            2 x 2

            Cannot become:
            2 x 4 (needs 8 elements)

            ---------------------------------------------

            Idea:

            Create a new matrix.

            Read original matrix row by row.

            Fill new matrix left to right.

            ---------------------------------------------

            Example:

            Original:

            1 2
            3 4

            Read order:

            1 → 2 → 3 → 4

            Fill:

            [1 2 3 4]

            ---------------------------------------------

            Time Complexity:
            O(m × n)

            Space Complexity:
            O(r × c)
        */

        // Number of rows in original matrix
        let rows = mat.len();

        // Number of columns in original matrix
        let cols = mat[0].len();

        /*
            If total number of elements is different,
            reshaping is impossible.
        */
        if rows * cols != (r as usize * c as usize) {
            return mat;
        }

        // Create result matrix filled with zeros
        let mut result = vec![vec![0; c as usize]; r as usize];

        // Current position in new matrix
        let mut new_row = 0;
        let mut new_col = 0;

        /*
            Traverse original matrix row by row.
        */
        for row in mat {

            for value in row {

                // Place current value into new matrix
                result[new_row][new_col] = value;

                // Move to next column
                new_col += 1;

                /*
                    If column becomes full,
                    move to next row.
                */
                if new_col == c as usize {
                    new_col = 0;
                    new_row += 1;
                }
            }
        }

        result
    }
}