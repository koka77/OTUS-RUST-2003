#![allow(dead_code)]
/*
    Дана строка, состоящая только из символов '{', '}', '(', ')', '[', ']'.
    Такая строка является корректной, если:
    - каждой открывающей скобке соответствует закрывающая того же типа
    - соблюдается порядок закрытия скобок
    - для каждой закрывающей скобки есть соответствующая открывающая пара

    Написать функцию, которая проверит корректность данной строки.
*/

fn validate_paren(s: &str) -> bool {
    let mut v = Vec::new();
    for c in s.chars() {
        if (c == ']' || c == '}' || c == ')') && v.is_empty() {
            return false;
        }
        if c == '[' || c == '{' || c == '(' {
            v.push(c)
        } else {
            let prev = v[v.len() - 1];

            if c == ']' && prev != '[' || c == '}' && prev != '{' || c == ')' && prev != '(' {
                return false;
            }
            v.pop();
        }
    }
    v.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(validate_paren("()"), true);
        assert_eq!(validate_paren("()[]{}"), true);
        assert_eq!(validate_paren("({[]()})"), true);
        assert_eq!(validate_paren("(}"), false);
        assert_eq!(validate_paren("()]"), false);
        assert_eq!(validate_paren("(){"), false);
    }
}

fn main() {}