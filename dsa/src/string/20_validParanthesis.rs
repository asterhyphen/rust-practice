impl Solution {
    pub fn is_valid(s: String) -> bool {
        /*
            Problem:
            Check whether brackets are properly matched.

            Rules:

            '(' matches ')'
            '[' matches ']'
            '{' matches '}'

            ------------------------------------------------

            Idea: Use a STACK.

            A stack works like a pile of books.

            Operations:

            push() -> Put a book on top.

                    3
                    2
                    1

            pop() -> Remove the top book.

                    2
                    1

            We always remove the LAST item added.

            This is called LIFO
            (Last In, First Out)

            ------------------------------------------------

            Algorithm:

            If opening bracket:
                Push it onto stack.

            If closing bracket:
                Check top of stack.

                If it matches,
                remove it.

                Else,
                return false.

            At the end:

            Stack should be empty.

            ------------------------------------------------

            Example:

            s = "([])"

            Read '('

            Stack:
            (

            Read '['

            Stack:
            (
            [

            Read ']'

            Matches '['

            Stack:
            (

            Read ')'

            Matches '('

            Stack:
            empty

            Answer = true

            ------------------------------------------------

            Example:

            s = "([)]"

            Stack:

            (
            [

            Read ')'

            Top is '['

            Doesn't match.

            Answer = false

            ------------------------------------------------

            Time Complexity:
            O(n)

            Space Complexity:
            O(n)
        */

        // Empty stack
        let mut stack = Vec::new();

        // Read each character
        for ch in s.chars() {

            // Opening brackets
            if ch == '(' || ch == '[' || ch == '{' {

                // Push into stack
                stack.push(ch);
            }
            else {

                /*
                    Pop the top element.

                    pop() returns:

                    Some(value)
                    OR

                    None
                    if stack is empty.
                */
                let top = stack.pop();

                /*
                    Check whether the popped
                    opening bracket matches
                    the current closing bracket.
                */
                if (ch == ')' && top != Some('(')) ||
                   (ch == ']' && top != Some('[')) ||
                   (ch == '}' && top != Some('{')) {

                    return false;
                }
            }
        }

        /*
            Stack must be empty.

            Otherwise some opening brackets
            never got closed.
        */
        stack.is_empty()
    }
}