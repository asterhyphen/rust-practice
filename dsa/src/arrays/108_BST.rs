use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {

        /*
            Problem:

            Convert a sorted array into
            a height-balanced BST.

            Example:

            nums = [-10,-3,0,5,9]

            Middle element:

                    0

            Left half:

                [-10,-3]

            Right half:

                [5,9]

            Repeat the same idea
            for every half.

            ------------------------------------------------

            Why middle element?

            Choosing the middle keeps the tree balanced.

            If we always choose first element,

            1
             \
              2
               \
                3

            Tree becomes skewed.

            Choosing middle gives:

                    2
                  /   \
                 1     3

            Balanced.

            ------------------------------------------------

            Algorithm

            1. Pick middle element.
            2. Make it root.
            3. Left half becomes left subtree.
            4. Right half becomes right subtree.
            5. Repeat recursively.

            ------------------------------------------------

            Time Complexity:

            O(n)

            Space Complexity:

            O(log n)
            (recursive call stack)
        */

        Self::build(&nums, 0, nums.len() as i32 - 1)
    }

    fn build(
        nums: &Vec<i32>,
        left: i32,
        right: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {

        /*
            Base Case

            If left crosses right,

            no numbers remain.

            Return empty tree.
        */
        if left > right {
            return None;
        }

        // Find middle element
        let mid = left + (right - left) / 2;

        /*
            Create current root node
            using middle value.
        */
        let root = Rc::new(
            RefCell::new(
                TreeNode::new(nums[mid as usize])
            )
        );

        /*
            Build left subtree
            using left half.
        */
        root.borrow_mut().left =
            Self::build(nums, left, mid - 1);

        /*
            Build right subtree
            using right half.
        */
        root.borrow_mut().right =
            Self::build(nums, mid + 1, right);

        Some(root)
    }
}