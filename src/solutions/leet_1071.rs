struct Solution;

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if format!("{}{}", &str1, &str2) != format!("{}{}", &str2, &str1) {
            return "".to_string();
        }

        let gcd = Self::gcd(str1.len(), str2.len());

        str1[..gcd].to_string()
    }

    fn gcd(mut a: usize, mut b: usize) -> usize {
        while b != 0 {
            let tmp = a % b;
            a = b;
            b = tmp;
        }
        a
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let str1 = String::from("ABCABCABC");
        let str2 = String::from("ABC");
        let result = Solution::gcd_of_strings(str1, str2);
        assert_eq!(result, "ABC".to_string())
    }
}
