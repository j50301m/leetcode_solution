/**
 * Your TreeAncestor object will be instantiated and called as such:
 * let obj = TreeAncestor::new(n, parent);
 * let ret_1: i32 = obj.get_kth_ancestor(node, k);
 */
struct TreeAncestor {
    table: Vec<Vec<i32>>,
}
const LOG: usize = 16;
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TreeAncestor {
    fn new(n: i32, parent: Vec<i32>) -> Self {
        let n = n as usize;
        let mut table = vec![vec![-1; n]; LOG];
        table[0] = parent;
        for jump in 1..LOG {
            for node in 0..n {
                let half_way = table[jump - 1][node];
                if half_way == -1 {
                    continue;
                }
                table[jump][node] = table[jump - 1][half_way as usize];
            }
        }

        Self { table }
    }

    fn get_kth_ancestor(&self, node: i32, k: i32) -> i32 {
        let mut node = node;
        let k = k as usize;
        let bit_count = (usize::BITS - k.leading_zeros()) as usize;
        for jump in 0..bit_count {
            if (k >> jump & 1) == 1 {
                if node == -1 {
                    return -1;
                }
                node = self.table[jump][node as usize];
            }
        }
        node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_case() {
        let tree = TreeAncestor::new(7, vec![-1, 0, 0, 1, 1, 2, 2]);
        assert_eq!(tree.get_kth_ancestor(3, 1), 1);
        assert_eq!(tree.get_kth_ancestor(5, 2), 0);
        assert_eq!(tree.get_kth_ancestor(6, 3), -1);
    }

    #[test]
    fn chain_long_jump() {
        // 0 <- 1 <- 2 <- ... <- 9
        let parent: Vec<i32> = (-1..9).collect();
        let tree = TreeAncestor::new(10, parent);
        assert_eq!(tree.get_kth_ancestor(9, 5), 4);
        assert_eq!(tree.get_kth_ancestor(9, 9), 0);
        assert_eq!(tree.get_kth_ancestor(9, 10), -1);
        assert_eq!(tree.get_kth_ancestor(9, 50000), -1);
    }

    #[test]
    fn single_node() {
        let tree = TreeAncestor::new(1, vec![-1]);
        assert_eq!(tree.get_kth_ancestor(0, 1), -1);
    }
}
