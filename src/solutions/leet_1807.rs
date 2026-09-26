use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        let bytes = s.as_bytes();
        let mut result = String::new();

        let mut map = HashMap::new();
        for k in knowledge.iter() {
            map.insert(&k[0], &k[1]);
        }

        let mut i = 0;
        while i < s.len() {
            let byte = bytes[i];
            match byte {
                b'(' => {
                    let val = Self::parse_key(bytes, &mut i, &map);
                    result.push_str(val);
                }
                _ => {
                    i += 1;
                    result.push(byte as char);
                }
            }
        }

        result
    }

    fn parse_key<'a>(bytes: &[u8], i: &mut usize, map: &HashMap<&String, &'a String>) -> &'a str {
        let mut key = String::new();
        while *i < bytes.len() {
            let byte = bytes[*i];
            *i += 1;
            if byte == b'(' {
                continue;
            } else if byte == b')' {
                break;
            }

            if Self::is_letter(byte) {
                key.push(byte as char);
            }
        }

        if let Some(val) = map.get(&key) {
            return val.as_str();
        } else {
            let q: &'a str = "?";
            return q;
        }
    }

    fn is_letter(b: u8) -> bool {
        b >= b'a' && b <= b'z'
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::evaluate(
                "(name)is(age)yearsold".to_string(),
                vec![
                    vec!["name".to_string(), "bob".to_string()],
                    vec!["age".to_string(), "two".to_string()]
                ]
            ),
            "bobistwoyearsold".to_string()
        );
    }
}
