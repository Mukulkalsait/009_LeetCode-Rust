
// G: -------------------------- logic
// 1. devide by 2 or false
// 2. inisitlise mutable stack
// 3. loop on chars 
// 3.1 if open => push or => 4.(100% colse) so
// 5. => if pop + 6.compair( 7match)  8(or false)
//    * 7: match c = case if != abc false
// 9. check if empty?  
//------------------------------ done

fn main() {
    
    let s:String = String::from("()[]{}");
    let op: bool = true;

    if op == solution(s) { 
        println!("Passed !")
    }
}

fn solution(s:String) -> bool { 
    if s.len() % 2 != 0 {return false;} // Y: 1

    let mut stack_data:Vec<char> = Vec::new(); // Y: 2

    for char in s.chars(){ // Y:  3

        if char == '(' || char == '[' || char == '{'{ stack_data.push(char);} // Y: 3.1

        else { // Y: 4

            if let Some(value_popped_from_stack_if_stack_is_not_empty) = stack_data.pop(){  // Y: 5
                match char { // Y: 6
                    ')' => if value_popped_from_stack_if_stack_is_not_empty != '(' { return false;}, // Y: 7
                    '}' => if value_popped_from_stack_if_stack_is_not_empty != '{' { return false;},
                    ']' => if value_popped_from_stack_if_stack_is_not_empty != '[' { return false;},
                    _   => {} // B: if none.
                }
            }else {return false;} // Y: 8
        }
    }
    stack_data.is_empty() // Y: 9: A Question statement hence :=> if true = return true or return false. 
}


// improvements : 
//
// TAG:  1
// match char {
//     '(' | '[' | '{' => stack_data.push(char),
//     ')' | ']' | '}' => { ... }
//     _ => {}
// }
//
// TAG: 2
// add test cases
