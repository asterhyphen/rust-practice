/*The algorithm finds the number that appears more than half the time in a list.

While going through the list, it keeps removing pairs of different numbers.
Since the majority number occurs the most, it cannot be completely removed — so it is the one left at the end.*/
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut candidate = 0;

    for num in nums {
        if count == 0 {
            candidate = num;
        }

        if num == candidate {
            count += 1;
        } else {
            count -= 1;
        }
    }

    candidate
}
