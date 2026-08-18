// /// 二維樹狀陣列（2D Binary Indexed Tree / Fenwick Tree）
// ///
// /// tree[i][j] 存的是一塊「矩形」的和：
// ///     列 [i - lowbit(i) + 1, i]  ×  行 [j - lowbit(j) + 1, j]
// /// 高 lowbit(i)、寬 lowbit(j)，兩個維度各自跑各自的 lowbit。
// ///
// /// tree 用 1-based 索引（第 0 列 / 第 0 行永遠是 0），
// /// 因為 lowbit(0) == 0，從 0 出發會無窮迴圈。
// struct NumMatrix {
//     row: usize,
//     col: usize,
//     /// 原始值。只為了 update 時算出增量 delta，BIT 本身沒辦法「設定」某一格。
//     matrix: Vec<Vec<i32>>,
//     /// (row + 1) x (col + 1)
//     tree: Vec<Vec<i32>>,
// }

// /**
//  * `&self` means the method takes an immutable reference.
//  * If you need a mutable reference, change it to `&mut self` instead.
//  */
// impl NumMatrix {
//     fn new(matrix: Vec<Vec<i32>>) -> Self {
//         let row = matrix.len();
//         let col = matrix.first().map_or(0, |r| r.len());

//         let mut obj = Self {
//             row,
//             col,
//             matrix,
//             tree: vec![vec![0; col + 1]; row + 1],
//         };

//         // tree 一開始全是 0，所以「增量」就等於原值本身。
//         // O(m·n·log m·log n)，這題資料量下綽綽有餘。
//         for r in 0..row {
//             for c in 0..col {
//                 let val = obj.matrix[r][c];
//                 obj.add(r + 1, c + 1, val);
//             }
//         }

//         obj
//     }

//     fn update(&mut self, row: i32, col: i32, val: i32) {
//         let (r, c) = (row as usize, col as usize);

//         // BIT 只會「加」不會「設」，所以要自己算差值
//         let delta = val - self.matrix[r][c];
//         self.matrix[r][c] = val; // 同步，否則下一次 delta 就錯了
//         self.add(r + 1, c + 1, delta); // 外部 0-based → 內部 1-based
//     }

//     fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
//         // 外部 0-based → 內部 1-based。
//         // r2/c2 要 +1；r1/c1 刻意「不」+1 ——
//         // 容斥需要的正是 (r1 - 1) 那一行，而它在 1-based 下剛好就是原本的 row1。
//         // 這樣寫就永遠不會出現 usize 減法，也就不會下溢 panic。
//         let (r1, c1) = (row1 as usize, col1 as usize);
//         let (r2, c2) = (row2 as usize + 1, col2 as usize + 1);

//         self.query(r2, c2) - self.query(r1, c2) - self.query(r2, c1) + self.query(r1, c1)
//     }

//     /// 單點加上增量：走 pathUp(r) × pathUp(c)。
//     /// 所有「矩形涵蓋 (r, c)」的節點都要更新，而那些節點剛好是兩條路徑的笛卡兒積。
//     /// r, c 皆為 1-based。
//     fn add(&mut self, r: usize, c: usize, delta: i32) {
//         let mut i = r;
//         while i <= self.row {
//             let mut j = c;
//             while j <= self.col {
//                 self.tree[i][j] += delta;
//                 j += Self::lowbit(j);
//             }
//             i += Self::lowbit(i);
//         }
//     }

//     /// 前綴矩形 (1,1)..=(r, c) 的和：走 pathDown(r) × pathDown(c)。
//     /// 這些節點的矩形不重疊、且剛好鋪滿整塊前綴矩形。
//     /// r 或 c 為 0 時迴圈不進入，自然回傳 0。
//     fn query(&self, r: usize, c: usize) -> i32 {
//         let mut sum = 0;
//         let mut i = r;
//         while i > 0 {
//             let mut j = c;
//             while j > 0 {
//                 sum += self.tree[i][j];
//                 j -= Self::lowbit(j);
//             }
//             i -= Self::lowbit(i);
//         }
//         sum
//     }

//     /// 取出 x 最低位的那個 1。
//     /// usize 沒有一元負號，所以用 wrapping_neg() 取二補數。
//     fn lowbit(x: usize) -> usize {
//         x & x.wrapping_neg()
//     }
// }

struct NumMatrix {
    n: usize,
    m: usize,
    matrix: Vec<Vec<i32>>,
    tree: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let n = matrix.len();
        let m = matrix[0].len();
        let tree = vec![vec![0; m + 1]; n + 1];

        let mut obj = Self { n, m, matrix, tree };

        for i in 0..n {
            for j in 0..m {
                obj.add(i + 1, j + 1, obj.matrix[i][j]);
            }
        }

        obj
    }

    fn add(&mut self, row: usize, col: usize, delta: i32) {
        let mut r = row;
        while r <= self.n {
            let mut c = col;
            while c <= self.m {
                self.tree[r][c] += delta;
                c += self.low_bit(c);
            }
            r += self.low_bit(r);
        }
    }

    fn low_bit(&self, x: usize) -> usize {
        x & x.wrapping_neg()
    }

    fn update(&mut self, row: i32, col: i32, val: i32) {
        let r = row as usize;
        let c = col as usize;
        let origin = self.matrix[r][c];

        let delta = val - origin;
        self.add(r + 1, c + 1, delta);
        self.matrix[r][c] = val;
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let (r1, c1) = (row1 as usize, col1 as usize);
        let (r2, c2) = (row2 as usize, col2 as usize);

        let sum = self.query(r2 + 1, c2 + 1) - self.query(r2 + 1, c1) - self.query(r1, c2 + 1)
            + self.query(r1, c1);
        sum
    }

    fn query(&self, row: usize, col: usize) -> i32 {
        let mut sum = 0;
        let mut r = row;
        while r > 0 {
            let mut c = col;
            while c > 0 {
                sum += self.tree[r][c];
                c -= self.low_bit(c);
            }
            r -= self.low_bit(r);
        }

        sum
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * obj.update(row, col, val);
 * let ret_2: i32 = obj.sum_region(row1, col1, row2, col2);
 */
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let mut obj = NumMatrix::new(vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ]);

        assert_eq!(obj.sum_region(2, 1, 4, 3), 8);
        obj.update(3, 2, 2);
        assert_eq!(obj.sum_region(2, 1, 4, 3), 10);
    }

    #[test]
    fn case_2() {
        let mut obj = NumMatrix::new(vec![vec![1, 2], vec![3, 4]]);

        assert_eq!(obj.sum_region(0, 0, 1, 1), 10); // 全部
        assert_eq!(obj.sum_region(1, 1, 1, 1), 4); // 單格，容斥四項都用到
        assert_eq!(obj.sum_region(0, 0, 0, 1), 3); // 第一列

        obj.update(0, 0, 5); // 1 → 5，delta = +4
        assert_eq!(obj.sum_region(0, 0, 1, 1), 14);
        assert_eq!(obj.sum_region(1, 1, 1, 1), 4); // 沒被波及
    }

    #[test]
    fn case_3() {
        // 反覆 update 同一格：驗證 delta 有正確用「舊值」算，而不是越滾越大
        let mut obj = NumMatrix::new(vec![vec![1]]);

        obj.update(0, 0, 10);
        assert_eq!(obj.sum_region(0, 0, 0, 0), 10);
        obj.update(0, 0, 3);
        assert_eq!(obj.sum_region(0, 0, 0, 0), 3);
        obj.update(0, 0, -7);
        assert_eq!(obj.sum_region(0, 0, 0, 0), -7);
    }

    #[test]
    fn case_4() {
        // 非正方形，且 row/col 長度不同 —— 抓 col 誤用 matrix.len() 的 bug
        let mut obj = NumMatrix::new(vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]]);

        assert_eq!(obj.sum_region(0, 0, 1, 4), 55);
        assert_eq!(obj.sum_region(0, 4, 1, 4), 15); // 最後一行
        obj.update(1, 4, 0);
        assert_eq!(obj.sum_region(0, 0, 1, 4), 45);
    }
}
