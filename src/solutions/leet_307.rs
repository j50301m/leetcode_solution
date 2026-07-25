struct NumArray {
    segment_tree: Vec<i32>,
    nums_len: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len().next_power_of_two() * 2;
        let mut segment_tree: Vec<i32> = vec![0; n];
        Self::build(1, 0, nums.len() - 1, &nums, &mut segment_tree);
        Self {
            segment_tree,
            nums_len: nums.len(),
        }
    }

    fn build(node: usize, left: usize, right: usize, nums: &[i32], tree: &mut [i32]) {
        if left == right {
            tree[node] = nums[left];
            return;
        }

        let mid = left + (right - left) / 2;
        Self::build(2 * node, left, mid, nums, tree);
        Self::build(2 * node + 1, mid + 1, right, nums, tree);

        tree[node] = tree[2 * node] + tree[2 * node + 1];
    }

    fn update(&mut self, index: i32, val: i32) {
        Self::modify(
            1,
            0,
            self.nums_len - 1,
            index as usize,
            val,
            &mut self.segment_tree,
        );
    }

    fn modify(node: usize, left: usize, right: usize, index: usize, val: i32, tree: &mut [i32]) {
        if left == right && left == index {
            tree[node] = val;
            return;
        }
        let mid = left + (right - left) / 2;

        if index <= mid {
            Self::modify(2 * node, left, mid, index, val, tree);
        } else {
            Self::modify(2 * node + 1, mid + 1, right, index, val, tree);
        }

        tree[node] = tree[2 * node] + tree[2 * node + 1];
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        Self::query(
            1,
            0,
            self.nums_len - 1,
            left as usize,
            right as usize,
            &self.segment_tree,
        )
    }

    fn query(
        node: usize,
        left: usize,
        right: usize,
        target_left: usize,
        target_right: usize,
        tree: &Vec<i32>,
    ) -> i32 {
        if target_left <= left && right <= target_right {
            return tree[node];
        }

        let mid = left + (right - left) / 2;
        let mut sum = 0;
        if target_left <= mid {
            sum += Self::query(2 * node, left, mid, target_left, target_right, tree);
        }
        if target_right > mid {
            sum += Self::query(
                2 * node + 1,
                mid + 1,
                right,
                target_left,
                target_right,
                tree,
            );
        }
        sum
    }
}

// /**
//  * Your NumArray object will be instantiated and called as such:
//  * let obj = NumArray::new(nums);
//  * obj.update(index, val);
//  * let ret_2: i32 = obj.sum_range(left, right);
//  */
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let mut array = NumArray::new(vec![1, 3, 5]);
        let ans_1 = array.sum_range(0, 2);
        array.update(1, 2);
        let ans_2 = array.sum_range(0, 2);
        assert_eq!(ans_1, 9);
        assert_eq!(ans_2, 8);
    }
}
