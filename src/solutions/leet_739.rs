struct Solution {}

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<(i32, usize)> = Vec::with_capacity(temperatures.len());
        let mut result = vec![0; temperatures.len()];

        for (i, &temperature) in temperatures.iter().enumerate() {
            while let Some(&(top_temp, idx)) = stack.last() {
                if temperature <= top_temp {
                    break;
                }
                let _ = stack.pop();
                result[idx] = (i - idx) as i32;
            }
            stack.push((temperature, i));
        }

        result
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::daily_temperatures(vec![89, 62, 70, 58, 47, 47, 46, 76, 100, 70]),
            vec![8, 1, 5, 4, 3, 2, 1, 1, 0, 0]
        );
    }
}
