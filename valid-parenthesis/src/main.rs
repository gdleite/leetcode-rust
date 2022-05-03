fn main() {
    dbg!(is_valid("[](){}".to_string()));
}

pub fn is_valid(s: String) -> bool {
    let mut open_brackets_stack = Vec::new();
        for c in s.chars() {
            match c {
                '{' | '(' | '[' => open_brackets_stack.push(c),
                '}' | ')' | ']' => {
                    if let Some(prev) = open_brackets_stack.pop() {
                        match (prev, c) {
                            ('{', '}') | ('(', ')') | ('[', ']') => continue,

                            _ => return false 
                        }
                    } else {
                        return false
                    }
                },
                // else continue
                _ => continue
            }
        }
    return open_brackets_stack.len() == 0 
}