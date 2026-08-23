struct Solution;

impl Solution {
    pub fn sum_game(num: String) -> bool {
        let bytes = num.as_bytes();
        let mut sum: i32 = 0;
        let mut question_count = 0;
        for i in 0..bytes.len() / 2 {
            let c = bytes[i];
            if c as char == '?' {
                question_count += 1;
                continue;
            }
            sum += (c - b'0') as i32;
        }
        for i in bytes.len() / 2..bytes.len() {
            let c = bytes[i];
            if c as char == '?' {
                question_count -= 1;
                continue;
            }
            sum -= (c - b'0') as i32;
        }

        if (sum * 2) == -9 * question_count {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // 官方三個例子
    #[test]
    fn no_question_mark_tie() {
        // 5+0 == 2+3，沒有 ? 可動，Bob 直接贏
        assert!(!Solution::sum_game("5023".to_string()));
    }

    #[test]
    fn two_question_marks_one_side() {
        // 左 7、右兩個 ?：Bob 補九只能湊到 9，7 != 9
        assert!(Solution::sum_game("25??".to_string()));
    }

    #[test]
    fn exactly_compensated() {
        // 左 14/1個?、右 5/3個?：淨差 9，淨多 2 個 ? 在右邊 → 剛好補平
        assert!(!Solution::sum_game("?3295???".to_string()));
    }

    // 邊界：?  與差值同側，Bob 沒救
    #[test]
    fn question_marks_on_the_bigger_side() {
        // 左 9/2個?、右 0/0個?：左只會更大
        assert!(Solution::sum_game("9??00000".to_string()));
    }

    // 邊界：差值不是 9 的倍數（整數除法會在這裡出錯）
    #[test]
    fn diff_not_multiple_of_nine() {
        // 左 10、右兩個 ?：Bob 最多湊到 9，10 != 9
        assert!(Solution::sum_game("55??".to_string()));
    }

    // 邊界：左右 ? 數量相同 → 互相對消，只看已知數字和（並確認不會除以零）
    #[test]
    fn balanced_question_marks_cancel() {
        assert!(!Solution::sum_game("?5?5".to_string())); // 5 == 5，Bob 鏡像跟牌
        assert!(Solution::sum_game("?5?6".to_string())); // 5 != 6，鏡像也救不回來
    }

    // 邊界：? 總數為奇數 → Alice 有最後一手，必勝
    #[test]
    fn odd_number_of_question_marks() {
        assert!(Solution::sum_game("0?".to_string()));
    }

    // 邊界：最小長度、左右各一個 ? → Bob 鏡像跟牌
    #[test]
    fn all_question_marks() {
        assert!(!Solution::sum_game("??".to_string()));
    }
}
