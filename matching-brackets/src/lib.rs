pub fn brackets_are_balanced(string: &str) -> bool {
    let mut store = Vec::new();

    for char in string.chars() {
        match char {
            '(' | '[' | '{' => store.push(char),

            ')' => {
                if store.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if store.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if store.pop() != Some('{') {
                    return false;
                }
            }
            _ => {}
        }
    }
    // Return true is empty as bracket is balanced
    store.is_empty()
}
