use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        let mask = 0;
        let mut memo = HashMap::<i32, bool>::new();

        if desired_total <= 0 {
            return true;
        }
        if desired_total > (max_choosable_integer + 1) * max_choosable_integer / 2 {
            return false;
        }

        fn backtrack(
            mask: i32,
            remaining: i32,
            max_num: i32,
            memo: &mut HashMap<i32, bool>,
        ) -> bool {
            if let Some(&result) = memo.get(&mask) {
                return result;
            }

            for i in 0..max_num {
                if mask & (1 << i) != 0 {
                    continue;
                }

                if (i + 1) >= remaining {
                    memo.insert(mask, true);
                    return true;
                }

                if !backtrack(mask | (1 << i), remaining - i - 1, max_num, memo) {
                    memo.insert(mask, true);
                    return true;
                }
            }

            memo.insert(mask, false);
            false
        }

        backtrack(mask, desired_total, max_choosable_integer, &mut memo)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        assert!(!Solution::can_i_win(10, 11));
    }

    #[test]
    fn case_2() {
        assert!(Solution::can_i_win(10, 0));
    }

    #[test]
    fn case_3() {
        assert!(Solution::can_i_win(10, 1));
    }

    // 總和 10 < 11，誰都到不了終點
    #[test]
    fn unreachable_even_depth() {
        assert!(!Solution::can_i_win(4, 11));
    }

    // 總和 15 < 50，層數是奇數 —— 少了剪枝時假訊號會翻成 true
    #[test]
    fn unreachable_odd_depth() {
        assert!(!Solution::can_i_win(5, 50));
    }

    #[test]
    fn single_number() {
        assert!(Solution::can_i_win(1, 1));
        assert!(!Solution::can_i_win(1, 2));
    }

    // 先手直接拿 10 就結束
    #[test]
    fn first_move_wins() {
        assert!(Solution::can_i_win(10, 10));
        assert!(Solution::can_i_win(10, 20));
    }

    // 總和夠但先手必敗，驗遞迴本體
    #[test]
    fn reachable_but_lose() {
        assert!(!Solution::can_i_win(10, 40));
    }

    // 總和 210 剛好等於目標 → 20 個數字必須全部取完，最後一手是後手
    #[test]
    fn upper_bound() {
        assert!(!Solution::can_i_win(20, 210));
    }
}
