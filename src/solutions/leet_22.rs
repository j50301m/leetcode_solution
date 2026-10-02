struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut memo = String::new();
        let mut result = Vec::new();
        Self::dfs(&mut memo, n, n, &mut result);
        result
    }

    fn dfs(memo: &mut String, left: i32, right: i32, result: &mut Vec<String>) {
        if left == 0 && right == 0 {
            result.push(memo.clone());
            return;
        }

        if left > 0 {
            memo.push('(');
            Self::dfs(memo, left - 1, right, result);
            memo.pop();
        }

        if right > left {
            memo.push(')');
            Self::dfs(memo, left, right - 1, result);
            memo.pop();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::generate_parenthesis(2),
            vec!["(())".to_string(), "()()".to_string()]
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec![
                "((()))".to_string(),
                "(()())".to_string(),
                "(())()".to_string(),
                "()(())".to_string(),
                "()()()".to_string()
            ]
        );
    }
}
