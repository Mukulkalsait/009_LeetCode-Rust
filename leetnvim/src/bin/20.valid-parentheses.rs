// @leet start

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 != 0 {
            return false;
        } 
        let mut stack_in_vec:Vec<char> = Vec::new(); // Char type vector.
        for char in s.chars(){ //Vector loop 
            if char == '{' || char ==  '[' || char == '('{
                stack_in_vec.push(char);
            }   // if opning type => push.
            else { // else => 1. check if not empty => match or false.
                if let Some(abc) = stack_in_vec.pop() { 
                match char {
                    '}' => if abc !=  '{' { return false; },
                    ']' => if abc !=  '[' { return false; },
                    ')' => if abc !=  '(' { return false; },
                    _ => {}
                    }
                } else {
                    return false;
                }
            }
        }
        // if !stack_in_vec.is_empty(){ // after loop ends if still someting remain ? false
        //     return false;
        // }
        // return true;
        stack_in_vec.is_empty() // this is a bool => 
    }
}



// @leet end

