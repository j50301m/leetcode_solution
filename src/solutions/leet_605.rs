struct Solution;

impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
        let mut planted = 0;

        // 種花的condition = [i-1], [i+1] both =0 and i==0
        for idx in 0..flowerbed.len() {
            if flowerbed[idx] == 1 {
                continue;
            }

            let left = *flowerbed.get(idx - 1).unwrap_or(&0);
            let right = *flowerbed.get(idx + 1).unwrap_or(&0);
            if left == 0 && right == 0 {
                flowerbed[idx] = 1;
                planted += 1;
            }

            if planted >= n {
                return true;
            }
        }

        planted >= n
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 1;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, true)
    }

    #[test]
    fn case_2() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 2;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, false)
    }
}
