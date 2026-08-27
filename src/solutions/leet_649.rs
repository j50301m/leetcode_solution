use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut radiants = VecDeque::new();
        let mut dires = VecDeque::new();
        let n = senate.len();
        for (i, &s) in senate.as_bytes().iter().enumerate() {
            if s == b'R' {
                radiants.push_back(i);
            } else {
                dires.push_back(i);
            }
        }

        while radiants.len() > 0 && dires.len() > 0 {
            let r = radiants.pop_front().unwrap();
            let d = dires.pop_front().unwrap();
            if r < d {
                radiants.push_back(r + n);
            } else {
                dires.push_back(d + n);
            }
        }

        if radiants.len() > 0 {
            return "Radiant".to_string();
        }

        "Dire".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::predict_party_victory("RD".to_string()), "Radiant");
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::predict_party_victory("RDD".to_string()), "Dire");
    }

    // "RRDDD" 的鏡像 -> 答案也鏡像
    #[test]
    fn case3() {
        assert_eq!(Solution::predict_party_victory("DDRRR".to_string()), "Dire");
    }

    // 前面的人先開口，人少的一方也能贏 -> 不能用人數判斷
    #[test]
    fn case4() {
        assert_eq!(
            Solution::predict_party_victory("RRDDD".to_string()),
            "Radiant"
        );
    }

    #[test]
    fn case5() {
        assert_eq!(Solution::predict_party_victory("DD".to_string()), "Dire");
    }
}
