struct Solution;

impl Solution {
    pub fn stone_game_ix(stones: Vec<i32>) -> bool {
        let mut cnt = [0; 3];

        for x in stones {
            cnt[(x % 3) as usize] += 1;
        }

        if cnt[0] % 2 == 0 {
            return cnt[1] > 0 && cnt[2] > 0;
        }

        (cnt[1] as i32 - cnt[2] as i32).abs() > 2
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(Solution::stone_game_ix(vec![2, 1]), true);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::stone_game_ix(vec![2]), false);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::stone_game_ix(vec![5, 1, 2, 4, 3]), false);
    }

    #[test]
    fn example4() {
        assert_eq!(Solution::stone_game_ix(vec![1, 1, 2]), true);
    }
}
