struct Solution {}

impl Solution {
    pub fn digit_sum(s: String, k: i32) -> String {
        let mut vec = Self::to_vec(s);
        let k = k as usize;
        while vec.len() > k {
            vec = Self::condense(vec, k);
        }

        vec.iter().map(|&x| (x as u8 + b'0') as char).collect()
    }

    fn condense(vec: Vec<i32>, k: usize) -> Vec<i32> {
        let vec = vec
            .chunks(k)
            .map(|chunk| chunk.iter().sum::<i32>())
            .collect::<Vec<i32>>();

        let mut result = Vec::new();
        for num in vec {
            for byte in num.to_string().bytes() {
                result.push((byte - b'0') as i32);
            }
        }

        result
    }

    fn to_vec(s: String) -> Vec<i32> {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut vec = vec![0; n];
        for i in 0..n {
            let num = (bytes[i] - b'0') as i32;
            vec[i] = num;
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_sum() {
        assert_eq!(Solution::digit_sum("11111222223".to_string(), 3), "135");
        assert_eq!(Solution::digit_sum("00000000".to_string(), 3), "000");
        assert_eq!(Solution::digit_sum("1234".to_string(), 2), "37");
    }
}
