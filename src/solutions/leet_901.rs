/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */
struct StockSpanner {
    stack: Vec<(i32, i32)>, // (price,span)
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }

    fn next(&mut self, price: i32) -> i32 {
        let mut span = 1;
        while let Some(&(last_price, last_span)) = self.stack.last() {
            if price < last_price {
                break;
            }
            self.stack.pop();
            span += last_span;
        }

        self.stack.push((price, span));

        span
    }
}

#[cfg(test)]
mod test {

    use super::*;

    fn run(prices: Vec<i32>) -> Vec<i32> {
        let mut s = StockSpanner::new();
        prices.into_iter().map(|p| s.next(p)).collect()
    }

    #[test]
    fn case1() {
        assert_eq!(
            run(vec![100, 80, 60, 70, 60, 75, 85]),
            vec![1, 1, 1, 2, 1, 4, 6]
        );
    }

    #[test]
    fn case2() {
        // 相等的價格也要被吞掉
        assert_eq!(run(vec![31, 41, 48, 59, 79]), vec![1, 2, 3, 4, 5]);
        assert_eq!(run(vec![50, 50, 50]), vec![1, 2, 3]);
    }

    #[test]
    fn case3() {
        // 一路遞減，什麼都不 pop
        assert_eq!(run(vec![90, 80, 70]), vec![1, 1, 1]);
    }
}
