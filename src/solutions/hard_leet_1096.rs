use std::collections::BTreeSet;

struct Solution {}

// 文法（優先順序：× 比 , 先算，{} 是括號）
//   union   := product (',' product)*
//   product := atom atom ...          // 讀到 ',' 或 '}' 或結尾就停
//   atom    := 字母 | '{' union '}'
impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        let s = expression.as_bytes();
        let mut i = 0;
        Self::parse_union(s, &mut i).into_iter().collect()
    }

    // 聯集：先讀一個乘積項，之後每遇到 ',' 就再讀一個併進來
    fn parse_union(s: &[u8], i: &mut usize) -> BTreeSet<String> {
        let mut acc = Self::parse_product(s, i);
        while *i < s.len() && s[*i] == b',' {
            *i += 1; // 吃掉 ','
            acc.extend(Self::parse_product(s, i));
        }
        acc
    }

    // 乘積：acc 從 {""} 開始（不是 ∅！）
    fn parse_product(s: &[u8], i: &mut usize) -> BTreeSet<String> {
        let mut acc = BTreeSet::from([String::new()]);
        while *i < s.len() && s[*i] != b',' && s[*i] != b'}' {
            let atom = Self::parse_atom(s, i);
            acc = Self::cross(&acc, &atom);
        }
        acc
    }

    // 原子：字母 → {"x"}；'{' → 遞迴 parse_union，然後吃掉 '}'
    fn parse_atom(s: &[u8], i: &mut usize) -> BTreeSet<String> {
        let c = s[*i];
        *i += 1; // 吃掉字母或 '{'
        if c == b'{' {
            let inner = Self::parse_union(s, i);
            *i += 1; // 吃掉 '}'
            inner
        } else {
            BTreeSet::from([(c as char).to_string()])
        }
    }

    // 笛卡兒積：{x + y | x ∈ a, y ∈ b}
    fn cross(a: &BTreeSet<String>, b: &BTreeSet<String>) -> BTreeSet<String> {
        let mut out = BTreeSet::new();
        for x in a {
            for y in b {
                out.insert(format!("{x}{y}"));
            }
        }
        out
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn run(s: &str) -> Vec<String> {
        Solution::brace_expansion_ii(s.to_string())
    }

    #[test]
    fn case_1() {
        assert_eq!(run("{a,b}{c,{d,e}}"), ["ac", "ad", "ae", "bc", "bd", "be"]);
    }

    #[test]
    fn case_2() {
        assert_eq!(run("{{a,z},a{b,c},{ab,z}}"), ["a", "ab", "ac", "z"]);
    }

    #[test]
    fn letter_next_to_brace() {
        assert_eq!(run("a{b,c}d"), ["abd", "acd"]);
    }
}
