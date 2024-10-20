#[derive(Debug)]
struct Stack<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn len(&self) -> usize {
        self.size
    }

    fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    fn push(&mut self, val: T) {
        self.data.push(val);
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.size > 0 {
            self.size -= 1;
            self.data.pop()
        } else {
            None
        }
    }

    fn peek(&self) -> Option<&T> {
        if self.size == 0 {
            None
        } else {
            self.data.get(self.size - 1)
        }
    }
}

fn bracket_match(bracket: &str) -> bool {
    let mut stack = Stack::new();

    // 定义一个映射，用于快速检查对应的闭合括号
    let matching_bracket = |c| match c {
        '(' => Some(')'),
        '{' => Some('}'),
        '[' => Some(']'),
        ')' | '}' | ']' => None, // 只处理开放括号
        _ => None, // 其他字符返回 None
    };

    // 遍历输入字符串的每个字符
    for ch in bracket.chars() {
        // 如果是开放括号，推入栈
        if let Some(closing) = matching_bracket(ch) {
            stack.push(closing);
        } else if ch == ')' || ch == '}' || ch == ']' {
            // 如果是闭合括号，检查栈是否为空
            if let Some(expected_closing) = stack.pop() {
                // 这里需要比较的是 expected_closing 和 ch
                if expected_closing != ch {
                    return false; // 不匹配则返回 false
                }
            } else {
                return false; // 栈为空，意味着有多余的闭合括号
            }
        }
    }

    // 最后检查栈是否为空，若为空，则所有括号均匹配
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bracket_matching_1() {
        let s = "(2+3){func}[abc]";
        assert_eq!(bracket_match(s), true);
    }

    #[test]
    fn bracket_matching_2() {
        let s = "(2+3)*(3-1";
        assert_eq!(bracket_match(s), false);
    }

    #[test]
    fn bracket_matching_3() {
        let s = "{{([])}}";
        assert_eq!(bracket_match(s), true);
    }

    #[test]
    fn bracket_matching_4() {
        let s = "{{(}[)]}";
        assert_eq!(bracket_match(s), false);
    }

    #[test]
    fn bracket_matching_5() {
        let s = "[[[]]]]]]]]]";
        assert_eq!(bracket_match(s), false);
    }

    #[test]
    fn bracket_matching_6() {
        let s = "";
        assert_eq!(bracket_match(s), true);
    }
}
