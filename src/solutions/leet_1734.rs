pub struct Solution;

impl Solution {
    pub fn decode(encoded: Vec<i32>) -> Vec<i32> {
        let total = (1..=encoded.len() as i32 + 1).fold(0, |acc, n| acc ^ n);
        let mut odd = 0;

        for i in (1..encoded.len()).step_by(2) {
            odd = odd ^ encoded[i];
        }

        let mut decoded = Vec::with_capacity(encoded.len() + 1);
        // push first prem
        decoded.push(total ^ odd);

        // For loop to resolve all prem
        for i in 0..encoded.len() {
            let encoded_num = encoded[i];
            let pre_decoded_num = decoded[i];
            decoded.push(encoded_num ^ pre_decoded_num);
        }

        decoded
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let result = Solution::decode(vec![3, 1]);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn case_2() {
        let result = Solution::decode(vec![6, 5, 4, 6]);
        assert_eq!(result, vec![2, 4, 1, 5, 3]);
    }
}
