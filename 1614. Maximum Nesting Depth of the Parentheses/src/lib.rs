#![allow(dead_code)]
// naive solution
pub fn max_depth_naive(s: String) -> i32 {
    let mut max_depth = 0;
    let mut depth = 0;
    for c in s.chars() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => (),
        }
        if depth > max_depth {
            max_depth = depth;
        }
    }
    max_depth
}
// functional solution
fn max_depth_scan(s: String) -> i32 {
    s.chars()
        .scan(0, |acc, x| {
            *acc = match x {
                '(' => *acc + 1,
                ')' => *acc - 1,
                _ => *acc,
            };
            Some(*acc)
        })
        .max()
        .unwrap_or(0)
}

// recursive solution
fn max_depth_recur(s: String) -> i32 {
    fn helper(s: &str, depth: i32, max_depth: i32) -> (i32, i32, i32) {
        match s.chars().next() {
            None => (0, depth, max_depth),
            Some('(') => helper(&s[1..], depth + 1, max_depth.max(depth + 1)),
            Some(')') => helper(&s[1..], depth - 1, max_depth),
            Some(_) => helper(&s[1..], depth, max_depth),
        }
    }
    helper(&s, 0, 0).2
}

fn max_depth(s: String) -> i32 {
    fn helper(s: &str, depth: i32) -> i32 {
        match s.chars().next() {
            None => 0,
            Some('(') => helper(&s[1..],depth+1).max(depth+1),
            Some(')') => helper(&s[1..],depth-1),
            Some(_) => helper(&s[1..],depth),
        }
    }
    helper(&s, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_depth() {
        assert_eq!(max_depth("".to_string()), 0);
        assert_eq!(max_depth("()(()())".to_string()), 2);
        assert_eq!(max_depth("()()".to_string()), 1);
        assert_eq!(max_depth("abc".to_string()), 0);
        assert_eq!(max_depth("(a(b)c)d".to_string()), 2);
        assert_eq!(max_depth("(((a)))".to_string()), 3);
        assert_eq!(max_depth("(a)(b)(c)(d)".to_string()), 1);
        assert_eq!(max_depth("a((bc)d)e".to_string()), 2);
    }
}
