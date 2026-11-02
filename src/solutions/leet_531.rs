struct Solution {}

impl Solution {
    pub fn find_lonely_pixel(picture: Vec<Vec<char>>) -> i32 {
        let m = picture.len();
        let n = picture[0].len();
        let mut row_b = vec![0; m];
        let mut col_b = vec![0; n];
        for i in 0..m {
            for j in 0..n {
                if picture[i][j] == 'B' {
                    row_b[i] += 1;
                    col_b[j] += 1;
                }
            }
        }

        let mut count = 0;
        for i in 0..m {
            for j in 0..n {
                if picture[i][j] == 'B' && row_b[i] == 1 && col_b[j] == 1 {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn grid(rows: &[&str]) -> Vec<Vec<char>> {
        rows.iter().map(|row| row.chars().collect()).collect()
    }

    #[test]
    fn case1() {
        assert_eq!(Solution::find_lonely_pixel(grid(&["WWB", "WBW", "BWW"])), 3);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::find_lonely_pixel(grid(&["BBB", "BBW", "BBB"])), 0);
    }

    #[test]
    fn same_row_not_adjacent() {
        assert_eq!(Solution::find_lonely_pixel(grid(&["BWB"])), 0);
    }

    #[test]
    fn same_column_not_adjacent() {
        assert_eq!(Solution::find_lonely_pixel(grid(&["B", "W", "B"])), 0);
    }

    #[test]
    fn non_square() {
        assert_eq!(Solution::find_lonely_pixel(grid(&["WWWB", "BWWW"])), 2);
    }
}
