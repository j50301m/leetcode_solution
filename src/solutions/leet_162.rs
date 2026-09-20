struct Solution {}

impl Solution {
    pub fn find_peak_element_approach1(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }

        let (mut lo, mut hi) = (0, nums.len()); // [lo,hi)
        while hi - lo > 1 {
            let mid = lo + (hi - lo) / 2;
            if nums[mid] < nums[mid - 1] {
                //判斷左手邊
                hi = mid;
            } else {
                lo = mid;
            }
        }
        lo as i32
    }

    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }

        let (mut lo, mut hi) = (0, nums.len() - 1); // [lo,hi)
        while hi - lo > 0 {
            let mid = lo + (hi - lo) / 2;
            if nums[mid] < nums[mid + 1] {
                // 判斷右手邊
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const IMPLS: [(&str, fn(Vec<i32>) -> i32); 2] = [
        ("approach1", Solution::find_peak_element_approach1),
        ("find_peak_element", Solution::find_peak_element),
    ];

    /// peak 不唯一，題目接受任何一個 → 驗「是不是 peak」而不是驗固定索引
    fn assert_peak(nums: &[i32]) {
        for (name, f) in IMPLS {
            let i = f(nums.to_vec());
            assert!(
                (0..nums.len() as i32).contains(&i),
                "{name}: 回傳 {i} 超出範圍，nums = {nums:?}"
            );
            let i = i as usize;
            let left_ok = i == 0 || nums[i] > nums[i - 1];
            let right_ok = i + 1 == nums.len() || nums[i] > nums[i + 1];
            assert!(
                left_ok && right_ok,
                "{name}: 索引 {i} 不是 peak，nums = {nums:?}"
            );
        }
    }

    /// peak 唯一時才鎖定索引
    fn assert_index(nums: &[i32], want: i32) {
        assert_peak(nums);
        for (name, f) in IMPLS {
            assert_eq!(f(nums.to_vec()), want, "{name}: nums = {nums:?}");
        }
    }

    #[test]
    fn leetcode_examples() {
        assert_index(&[1, 2, 3, 1], 2);
        assert_peak(&[1, 2, 1, 3, 5, 6, 4]); // 1 和 5 都對
    }

    #[test]
    fn tiny_inputs() {
        assert_index(&[1], 0);
        assert_index(&[1, 2], 1);
        assert_index(&[2, 1], 0);
    }

    #[test]
    fn monotonic_peak_sits_on_the_boundary() {
        // 全程上坡 → 靠 nums[n] = -∞ 才成立，最後一格是 peak
        assert_index(&[1, 2, 3, 4, 5], 4);
        // 全程下坡 → 靠 nums[-1] = -∞，第一格是 peak
        assert_index(&[5, 4, 3, 2, 1], 0);
    }

    #[test]
    fn many_peaks() {
        assert_peak(&[1, 3, 2, 4, 3, 5, 4]);
        assert_peak(&[5, 4, 3, 4, 5, 4, 3]);
    }

    #[test]
    fn extreme_values() {
        // -2^31 也是合法輸入，別假設元素為正
        assert_index(&[i32::MIN], 0);
        assert_index(&[i32::MIN, i32::MAX], 1);
        assert_peak(&[i32::MAX, i32::MIN, i32::MAX]);
    }

    #[test]
    fn stress_1000_elements() {
        // 37 與 1001 互質 → 1000 個相異值，保證沒有相鄰相等
        let nums: Vec<i32> = (0..1000).map(|i| (i * 37 % 1001) as i32).collect();
        assert_peak(&nums);
    }
}
