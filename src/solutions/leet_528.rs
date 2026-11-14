/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(w);
 * let ret_1: i32 = obj.pick_index();
 */
struct Solution {
    sum: i32,
    prefix_sum: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(w: Vec<i32>) -> Self {
        let n = w.len();
        let mut prefix_sum = vec![0; n + 1];
        for i in 0..n {
            prefix_sum[i + 1] = w[i] + prefix_sum[i];
        }
        Self {
            sum: prefix_sum[n],
            prefix_sum,
        }
    }

    fn pick_index(&self) -> i32 {
        let target = rand::random_range(1..=self.sum);
        let pos = self.prefix_sum.partition_point(|&x| x < target) - 1;
        pos as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // 只有一個元素，不管怎麼抽都一定是 0
    #[test]
    fn case1() {
        let solution = Solution::new(vec![1]);
        for _ in 0..100 {
            assert_eq!(solution.pick_index(), 0);
        }
    }

    // w = [1, 3]：index 0 機率 1/4，index 1 機率 3/4
    // 抽 10000 次，數量允許有誤差（期望 2500 / 7500）
    #[test]
    fn case2() {
        let solution = Solution::new(vec![1, 3]);
        let mut count = [0; 2];
        for _ in 0..10000 {
            count[solution.pick_index() as usize] += 1;
        }
        assert!((2200..=2800).contains(&count[0]), "count = {:?}", count);
        assert!((7200..=7800).contains(&count[1]), "count = {:?}", count);
    }

    // 回傳值一定落在 0..w.len()，而且每個 index 都至少抽到一次
    #[test]
    fn case3() {
        let weights = vec![3, 14, 1, 7];
        let solution = Solution::new(weights.clone());
        let mut count = vec![0; weights.len()];
        for _ in 0..10000 {
            let index = solution.pick_index();
            assert!((0..weights.len() as i32).contains(&index));
            count[index as usize] += 1;
        }
        assert!(count.iter().all(|&times| times > 0), "count = {:?}", count);
    }
}
