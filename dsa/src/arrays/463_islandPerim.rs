impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        /*
            Problem:
            Find the perimeter of the island.

            1 = Land
            0 = Water

            Only one island exists.

            ------------------------------------------------

            Key Idea:

            Every land cell contributes
            4 sides initially.

                   ----
                  |    |
                  | 1  |
                  |    |
                   ----

            Perimeter = 4

            ------------------------------------------------

            But...

            If two land cells touch,
            they SHARE one edge.

            Example:

            [1][1]

            Each cell has 4 sides.

            Total = 8

            Shared edge removes
            2 sides.

            Final perimeter = 6

            ------------------------------------------------

            So for every land cell:

            Start with +4

            If land exists above,
            subtract 2.

            If land exists on the left,
            subtract 2.

            We only check UP and LEFT
            to avoid subtracting twice.

            ------------------------------------------------

            Example:

            1 1

            First cell:
            +4

            Second cell:
            +4
            Left neighbour exists
            -2

            Total = 6

            ------------------------------------------------

            Time Complexity:
            O(rows × cols)

            Space Complexity:
            O(1)
        */

        // Total perimeter
        let mut perimeter = 0;

        // Number of rows
        let rows = grid.len();

        // Number of columns
        let cols = grid[0].len();

        // Visit every cell
        for i in 0..rows {

            for j in 0..cols {

                // Only process land cells
                if grid[i][j] == 1 {

                    // Every land cell contributes 4 sides
                    perimeter += 4;

                    /*
                        If land exists above,
                        remove shared edge.
                    */
                    if i > 0 && grid[i - 1][j] == 1 {
                        perimeter -= 2;
                    }

                    /*
                        If land exists on the left,
                        remove shared edge.
                    */
                    if j > 0 && grid[i][j - 1] == 1 {
                        perimeter -= 2;
                    }
                }
            }
        }

        perimeter
    }
}