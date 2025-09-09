pub fn brackets_are_balanced(string: &str) -> bool {
    if string.is_empty() {
        return true;
    }

    let mut stack = Vec::new();

    for ch in string.chars() {
        match ch {
            '[' | '{' | '(' => stack.push(ch),
            ']' => {
                if let Some('[') = stack.pop() {
                } else {
                    return false;
                }
            }
            '}' => {
                if let Some('{') = stack.pop() {
                } else {
                    return false;
                }
            }
            ')' => {
                if let Some('(') = stack.pop() {
                } else {
                    return false;
                }
            }
            _ => {}
        }
    }

    stack.is_empty()
}
