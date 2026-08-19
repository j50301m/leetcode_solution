use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        // 走道把一排切成 1 | 2345 | 6789 | 10，跨中間走道要 2+2，
        // 所以合法的四連位只有這三組。座位 1 和 10 永遠無關。
        const LEFT: u16 = 0b11_1100; // 2,3,4,5
        const MID: u16 = 0b1111_0000; // 4,5,6,7
        const RIGHT: u16 = 0b11_1100_0000; // 6,7,8,9

        let mut rows: HashMap<i32, u16> = HashMap::new();
        for seat in reserved_seats {
            *rows.entry(seat[0]).or_default() |= 1 << seat[1];
        }

        // 沒人訂的排一律坐得下兩組
        let mut total = 2 * (n - rows.len() as i32);
        for taken in rows.values() {
            total += match (taken & LEFT == 0, taken & RIGHT == 0) {
                (true, true) => 2,
                (false, false) => (taken & MID == 0) as i32,
                _ => 1,
            };
        }

        total
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn one_row(reserved: &[i32]) -> i32 {
        Solution::max_number_of_families(1, reserved.iter().map(|&c| vec![1, c]).collect())
    }

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::max_number_of_families(
                3,
                vec![
                    vec![1, 2],
                    vec![1, 3],
                    vec![1, 8],
                    vec![2, 6],
                    vec![3, 1],
                    vec![3, 10]
                ]
            ),
            4
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::max_number_of_families(2, vec![vec![2, 1], vec![1, 8], vec![2, 6]]),
            2
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::max_number_of_families(
                4,
                vec![vec![4, 3], vec![1, 4], vec![4, 6], vec![1, 7]]
            ),
            4
        );
    }

    #[test]
    fn empty_row_seats_two_families() {
        assert_eq!(one_row(&[]), 2);
    }

    #[test]
    fn aisle_seats_1_and_10_dont_matter() {
        assert_eq!(one_row(&[1, 10]), 2);
    }

    #[test]
    fn seat_9_blocks_the_right_group() {
        // 只剩 [2,3,4,5]
        assert_eq!(one_row(&[9]), 1);
    }

    #[test]
    fn seats_4_and_9_block_everything() {
        // [2-5] 卡 4、[4-7] 卡 4、[6-9] 卡 9
        assert_eq!(one_row(&[4, 9]), 0);
    }

    #[test]
    fn only_middle_group_left() {
        // [2-5] 卡 2、[6-9] 卡 9，只剩 [4,5,6,7]
        assert_eq!(one_row(&[2, 9]), 1);
    }

    #[test]
    fn four_free_seats_across_wrong_aisle_dont_count() {
        // 3,4,5,6 連在一起但跨走道是 3+1，不合法；只剩 [6,7,8,9]
        assert_eq!(one_row(&[2]), 1);
    }
}
