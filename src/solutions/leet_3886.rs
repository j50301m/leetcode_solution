struct Solution {}

impl Solution {
    pub fn sortable_integers(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let divisors = Self::divisors(n as i32);
        let mut sum = 0;
        for divisor in divisors {
            let d = divisor as usize;
            let mut all_match = true;
            if d == 1 {
                for i in 0..n - 1 {
                    if nums[i] > nums[i + 1] {
                        all_match = false;
                        break;
                    }
                }
            } else {
                let mut pre_max = 0;
                for i in (0..n).step_by(d) {
                    if let Some((min, max)) = Self::can_ascending(&nums[i..i + d]) {
                        if min < pre_max {
                            all_match = false;
                            break;
                        }
                        pre_max = max;
                    } else {
                        all_match = false;
                        break;
                    }
                }
            }

            if all_match {
                sum += d;
            }
        }

        sum as i32
    }

    fn divisors(num: i32) -> Vec<i32> {
        let mut divsors = Vec::new();
        let mut divsor = 1;
        while divsor * divsor <= num {
            if num % divsor == 0 {
                divsors.push(divsor);

                let paired = num / divsor;
                if paired != divsor {
                    divsors.push(paired);
                }
            }
            divsor += 1;
        }

        divsors
    }

    fn can_ascending(nums: &[i32]) -> Option<(i32, i32)> {
        let n = nums.len();
        let mut tolerate = 1;
        let mut max = 0;
        let mut min = i32::MAX;
        if n < 2 {
            return None;
        }
        for i in 0..n {
            if nums[i] > nums[(i + 1) % n] {
                tolerate -= 1;
            }
            if tolerate < 0 {
                return None;
            }
            max = nums[i].max(max);
            min = nums[i].min(min);
        }
        Some((min, max))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::sortable_integers(vec![3, 1, 2]), 3);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::sortable_integers(vec![3, 4, 1, 2]), 4);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::sortable_integers(vec![5, 8]), 3);
    }

    #[test]
    fn case4() {
        assert_eq!(Solution::sortable_integers(vec![4, 1, 2, 3]), 4);
    }

    #[test]
    fn case5() {
        assert_eq!(Solution::sortable_integers(vec![2, 3, 4, 1]), 4);
    }
}
