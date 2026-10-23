struct Solution {}

impl Solution {
    pub fn beautiful_indices(s: String, a: String, b: String, k: i32) -> Vec<i32> {
        let s = s.as_bytes();
        let a = a.as_bytes();
        let b = b.as_bytes();
        let k = k as usize;
        let a_indies = Self::find_indices(s, a);
        let b_indies = Self::find_indices(s, b);

        let mut b_ptr = 0;
        let mut result = Vec::new();
        for &a_idx in a_indies.iter() {
            while b_ptr < b_indies.len() && b_indies[b_ptr] + k < a_idx {
                b_ptr += 1;
            }

            if b_ptr < b_indies.len() && b_indies[b_ptr] <= a_idx + k {
                result.push(a_idx as i32);
            }
        }

        result
    }

    fn build_longest_borders(s: &[u8]) -> Vec<usize> {
        let mut longest_borders = vec![0; s.len()];
        let mut border = 0;
        for i in 1..s.len() {
            while border > 0 && s[i] != s[border] {
                border = longest_borders[border - 1];
            }

            if s[i] == s[border] {
                border += 1;
            }

            longest_borders[i] = border;
        }

        longest_borders
    }

    fn find_indices(source: &[u8], target: &[u8]) -> Vec<usize> {
        let mut found = Vec::new();
        let longest_borders = Self::build_longest_borders(target);
        let mut border = 0;
        for i in 0..source.len() {
            while border > 0 && source[i] != target[border] {
                border = longest_borders[border - 1];
            }

            if source[i] == target[border] {
                border += 1;
            }

            if border == target.len() {
                found.push(i - border + 1);
                border = longest_borders[border - 1];
            }
        }
        found
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::beautiful_indices(
                "isawsquirrelnearmysquirrelhouseohmy".to_string(),
                "my".to_string(),
                "squirrel".to_string(),
                15
            ),
            vec![16, 33]
        );
    }
}
