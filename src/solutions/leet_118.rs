pub struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let len = num_rows as usize;
        let mut result: Vec<Vec<i32>> = Vec::with_capacity(len);

        result.push(vec![1]);
        if num_rows == 1 {
            return result;
        }

        result.push(vec![1, 1]);
        if num_rows == 2 {
            return result;
        }

        for i in 2..len {
            let last = &result[i - 1];
            let mut tmp = vec![1; i + 1];
            for j in 1..i {
                tmp[j] = last[j] + last[j - 1];
            }
            result.push(tmp);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let result = Solution::generate(5);
        println!("{:?}", result);
    }
}
