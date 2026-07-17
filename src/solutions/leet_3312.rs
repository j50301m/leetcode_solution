struct Solution;

impl Solution {
    pub fn gcd_values(nums: Vec<i32>, queries: Vec<i64>) -> Vec<i32> {
        // 找到對大值 因為我們找倒gcd 因定比max小
        let max = *nums.iter().max().expect("must have a value") as usize;

        // 把所有可能的數當成index,裡面出現的次數
        let mut count = vec![0; max + 1];
        for &n in nums.iter() {
            count[n as usize] += 1;
        }

        // 找出所有gcd=n的數 => 所有n的倍數個數- 所有m乘的n倍數的個數總和
        let mut exact = vec![0i64; max + 1];
        for g in (1..=max).rev() {
            let mut c: i64 = 0; // g的倍數有多少個
            let mut over: i64 = 0; // n的倍數有幾個
            for m in (g..=max).step_by(g) {
                c += count[m];
                over += exact[m];
            }
            exact[g] = c * (c - 1) / 2 - over; // C(c,2), 扣n的倍數總和
        }

        // 查找
        let mut result = Vec::with_capacity(queries.len());
        for i in queries {
            let mut remain = i;
            for j in 0..exact.len() {
                let _count = exact[j];
                if remain < _count {
                    result.push(j as i32);
                    break;
                }
                remain -= _count;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use std::assert_eq;

    use super::*;

    #[test]
    fn case_1() {
        let nums = vec![4, 4, 2, 1];
        let queries = vec![5, 3, 1, 0];
        let result = Solution::gcd_values(nums, queries);
        assert_eq!(result, vec![4, 2, 1, 1]);
    }

    #[test]
    fn case_2() {
        let nums = vec![2, 3, 4];
        let queries = vec![0, 2, 2];
        let result = Solution::gcd_values(nums, queries);
        assert_eq!(result, vec![1, 2, 2]);
    }
}
