struct Solution {}

impl Solution {
    pub fn successful_pairs_old(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut least_spells = Vec::new();
        for potion in potions {
            let potion = potion as i64;
            if success % potion != 0 {
                least_spells.push((success / potion) + 1);
            } else {
                least_spells.push(success / potion);
            }
        }
        least_spells.sort_unstable_by(|a, b| b.cmp(a));

        let mut result = Vec::new();
        for spell in spells {
            let spell = spell as i64;
            let idx = least_spells.partition_point(|&x| x > spell);
            result.push((least_spells.len() - idx) as i32);
        }

        result
    }

    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        potions.sort_unstable_by(|a, b| b.cmp(a));
        spells
            .into_iter()
            .map(|s| {
                let s = s as i64;
                let need = (success + s - 1) / s; // Point : 不要把每次乘法都放進partition_point重算
                let idx = potions.partition_point(|&p| p as i64 >= need);
                (idx) as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(spells: &[i32], potions: &[i32], success: i64) -> Vec<i32> {
        Solution::successful_pairs(spells.to_vec(), potions.to_vec(), success)
    }

    #[test]
    fn leetcode_examples() {
        assert_eq!(run(&[5, 1, 3], &[1, 2, 3, 4, 5], 7), vec![4, 0, 3]);
        assert_eq!(run(&[3, 1, 2], &[8, 5, 8], 16), vec![2, 0, 2]);
    }

    #[test]
    fn potions_given_unsorted() {
        // 題目沒保證 potions 有序：2*5=10、2*1=2、2*9=18，只有兩組達標
        assert_eq!(run(&[2], &[5, 1, 9], 10), vec![2]);
    }

    #[test]
    fn threshold_is_inclusive() {
        // 乘積剛好等於 success 也算成功
        assert_eq!(run(&[2], &[5], 10), vec![1]);
        assert_eq!(run(&[2], &[5], 11), vec![0]);
    }

    #[test]
    fn all_or_none() {
        assert_eq!(run(&[1, 1], &[1, 1], 1), vec![2, 2]);
        assert_eq!(run(&[1, 1], &[1, 1], 2), vec![0, 0]);
    }

    #[test]
    fn duplicate_potions() {
        // 重複值不能被去掉，5,8,8 要算出 2 而不是 1
        assert_eq!(run(&[3, 1, 2], &[5, 8, 8], 16), vec![2, 0, 2]);
    }

    #[test]
    fn upper_bounds_no_overflow() {
        // 10^5 * 10^5 = 10^10 已超出 i32，確認全程用 i64
        assert_eq!(run(&[100_000], &[100_000], 10_000_000_000), vec![1]);
        assert_eq!(run(&[99_999], &[100_000], 10_000_000_000), vec![0]);
    }
}
