impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        /*
            Problem:
            Buy once and sell once.

            Buy must happen BEFORE sell.

            We want maximum profit.

            ------------------------------------------------

            Example:

            prices = [7,1,5,3,6,4]

            Buy at 1
            Sell at 6

            Profit = 6 - 1 = 5

            ------------------------------------------------

            Brute Force:

            Try every buy day with every sell day.

            Time Complexity:
            O(n²)

            Too slow.

            ------------------------------------------------

            Better Idea:

            While moving from left to right:

            Keep track of the LOWEST price seen so far.

            For each day:

            profit = current_price - lowest_price

            Update maximum profit if needed.

            ------------------------------------------------

            Example:

            prices = [7,1,5,3,6,4]

            Day 1:
            min_price = 7

            Day 2:
            min_price = 1

            Day 3:
            profit = 5 - 1 = 4

            Day 4:
            profit = 3 - 1 = 2

            Day 5:
            profit = 6 - 1 = 5   <- best

            Day 6:
            profit = 4 - 1 = 3

            Answer = 5

            ------------------------------------------------

            Time Complexity:
            O(n)

            Space Complexity:
            O(1)
        */

        // Lowest stock price seen so far
        let mut min_price = prices[0];

        // Best profit found so far
        let mut max_profit = 0;

        // Loop through every price
        for price in prices {

            /*
                If current price is smaller than
                the lowest price we've seen,
                update min_price.
            */
            if price < min_price {
                min_price = price;
            }

            /*
                Profit if we bought at min_price
                and sold today.
            */
            let profit = price - min_price;

            /*
                Keep the largest profit.
            */
            if profit > max_profit {
                max_profit = profit;
            }
        }

        // Return best profit found
        max_profit
    }
}