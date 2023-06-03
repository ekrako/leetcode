// is valid parentheses with for loop
#![allow(dead_code)]
pub fn is_valid_for_loop(s: String) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => unreachable!(),
        }
    }
    stack.is_empty()
}

// first solution  zeev set me
pub fn is_valid_zeev(s: String) -> bool {
    let mut temp_char:char;
    let open: [char; 3] = ['(', '{', '['];
    let close: [char; 3] = [')', '}', ']'];
    let couples: [(char, char); 3] = [('(', ')'), ('{', '}'), ('[', ']')];

    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        if open.contains(&c) {
            stack.push(c);
        } else if close.contains(&c) {
            if stack.len() == 0 {
                return false;
            }
            temp_char = stack.pop().unwrap();
            if !couples.contains(&(temp_char, c)) {
                return false;
            }
        }
    }
    if stack.len() > 0 {
        return false;
    } 
    return true;
}

pub fn is_valid_match(s: String) -> bool {
    let open: [char; 3] = ['(', '{', '['];
    let close: [char; 3] = [')', '}', ']'];
    let couples: [(char, char); 3] = [('(', ')'), ('{', '}'), ('[', ']')];

    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        if open.contains(&c) {
            stack.push(c);
        } else if close.contains(&c) {
            match stack.pop() {
                Some(temp_char) => {
                    if !couples.contains(&(temp_char, c)) {
                        return false;
                    }
                },
                None => return false,
            }
        }
    }
    stack.is_empty()
}

pub fn is_valid_open_close(s: String) -> bool {
    let open: [char; 3] = ['(', '{', '['];
    let close: [char; 3] = [')', '}', ']'];

    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        if open.contains(&c) {
            stack.push(close[open.iter().position(|&x| x == c).unwrap()]);
        } else if close.contains(&c) {
            match stack.pop() {
                Some(temp_char) => {
                    if temp_char != c {
                        return false;
                    }
                },
                None => return false,
            }
        }
    }
    stack.is_empty()
}

pub fn is_valid_couples(s: String) -> bool {
    let couples: [(char, char); 3] = [('(', ')'), ('{', '}'), ('[', ']')];

    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '{' | '[' => stack.push(couples.into_iter().find(|&x| x.0 == c).unwrap().1),
            ')' | '}' | ']' => {
                match stack.pop() {
                    Some(temp_char) => {
                        if temp_char != c {
                            return false;
                        }
                    },
                    None => return false,
                }
            }   ,
            _ => unreachable!(),
        };
    }
    stack.is_empty()
}

pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        match c {
            '(' => stack.push(')'),
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            ')' | '}' | ']' => {
                match stack.pop() {
                    Some(temp_char) => {
                        if temp_char != c {
                            return false;
                        }
                    },
                    None => return false,
                }
            }   ,
            _ => unreachable!(),
        };
    }
    stack.is_empty()
}
// is valid parentheses with fold
pub fn is_valid_fold(s: String) -> bool {
    enum CheckResult {
        Invalid,
        Continue(Vec<char>),
    }
    fn check(so_far: CheckResult, c: char) -> CheckResult {
        match so_far {
            CheckResult::Continue(mut v) => {
                match c {
                    '(' | '[' | '{' => {
                        v.push(c);
                        CheckResult::Continue(v)
                    }
                    ')' | ']' | '}' => {
                        let last = match v.pop() {
                            Some(c) => c,
                            None => return CheckResult::Invalid,
                        };
                        match (last, c) {
                            ('(', ')') | ('[', ']') | ('{', '}') => CheckResult::Continue(v),
                            _ => CheckResult::Invalid,
                        }
                    },
                    _ => unreachable!(),
                }  
            },
            CheckResult::Invalid => CheckResult::Invalid 
        }
    }
    match s.chars().fold(CheckResult::Continue(Vec::new()), check){
        CheckResult::Continue(v) => v.is_empty(),
        CheckResult::Invalid => false
    }
}

// is valid parentheses with recursion
pub fn is_valid_recur(s: String) -> bool {
    if s.is_empty() {
        return true;
    }
    let parent = ["()", "[]", "{}"];
    for p in parent {
        if s.contains(p) {
            return is_valid_recur(s.replace(p, "", ));
        }
    }
    s.is_empty()
}
#[cfg(test)]
mod tests {
    use super::is_valid;

    #[test]
    fn test_empty_string() {
        assert!(is_valid("".to_string()));
    }

    #[test]
    fn test_single_char() {
        assert!(!is_valid("(".to_string()));
        assert!(!is_valid(")".to_string()));
        assert!(!is_valid("[".to_string()));
        assert!(!is_valid("]".to_string()));
        assert!(!is_valid("{".to_string()));
        assert!(!is_valid("}".to_string()));
    }

    #[test]
    fn test_valid_cases() {
        assert!(is_valid("()".to_string()));
        assert!(is_valid("[]".to_string()));
        assert!(is_valid("{}".to_string()));
        assert!(is_valid("(())".to_string()));
        assert!(is_valid("[[]]".to_string()));
        assert!(is_valid("{{}}".to_string()));
        assert!(is_valid("({[]})".to_string()));
        assert!(is_valid("({[]}){()}()[]".to_string()));
    }

    #[test]
    fn test_invalid_cases() {
        assert!(!is_valid("(]".to_string()));
        assert!(!is_valid("([)".to_string()));
        assert!(!is_valid("{)".to_string()));
        assert!(!is_valid("(()".to_string()));
        assert!(!is_valid("[[".to_string()));
        assert!(!is_valid("{".to_string()));
        assert!(!is_valid("({[]}".to_string()));
    }
}