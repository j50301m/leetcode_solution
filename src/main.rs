use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap, HashMap, HashSet},
    vec,
};

mod solutions;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        // Rust heap is max heap
        let mut heap = BinaryHeap::new();

        // Push all element in heap
        for list in lists {
            let mut cur = list;
            while let Some(node) = cur {
                heap.push(node.val);
                cur = node.next;
            }
        }

        // Pop element
        let mut head = None;
        while let Some(val) = heap.pop() {
            head = Some(Box::new(ListNode { val, next: head }));
        }

        return head;
    }
}
fn build(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &v in vals.iter().rev() {
        head = Some(Box::new(ListNode { val: v, next: head }));
    }
    head
}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_area = 0;
        while left < right {
            let h = if height[left] < height[right] {
                height[left]
            } else {
                height[right]
            };

            let area = h * (right - left) as i32;
            if area > max_area {
                max_area = area;
            }

            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_area
    }

    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map: HashMap<i32, usize> = HashMap::new(); // key: nums[i], val= index;

        // map.insert(nums[0], 0);
        for i in 0..nums.len() {
            if map.contains_key(&nums[i]) {
                let index = map[&nums[i]];
                if ((i - index) as i32).abs() <= k {
                    return true;
                }
            }
            map.insert(nums[i], i);
        }
        false
    }

    pub fn contains_nearby_almost_duplicate(
        nums: Vec<i32>,
        index_diff: i32,
        value_diff: i32,
    ) -> bool {
        let mut window: BTreeSet<i32> = BTreeSet::new();

        for i in 0..nums.len() {
            let num = nums[i];
            if window
                .range((num - value_diff)..=(num + value_diff))
                .next()
                .is_some()
            {
                return true;
            }

            // Insert current num
            window.insert(num);

            if (i >= index_diff as usize) {
                window.remove(&nums[i - index_diff as usize]);
            }
        }

        return false;
    }

    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut k = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[k] = nums[i];
                k += 1;
            }
        }

        k as i32
    }

    pub fn move_zeros(nums: &mut Vec<i32>) {
        let mut k = 0;

        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[k] = nums[i];
                k += 1;
            }
        }

        for i in k..nums.len() {
            nums[i] = 0;
        }
    }

    pub fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;

        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                nums[i] = nums[i] * 2;
                nums[i + 1] = 0;
            }
        }

        let mut k = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[k] = nums[i];
                k += 1;
            }
        }

        for i in k..nums.len() {
            nums[i] = 0;
        }

        nums
    }

    // 2527. Find Xor-Beauty of Array
    pub fn xor_beauty(nums: Vec<i32>) -> i32 {
        let acc = nums.iter().fold(0, |acc, x| acc ^ x);
        acc
    }

    pub fn decode(encoded: Vec<i32>) -> Vec<i32> {
        let total = (1..=encoded.len() as i32 + 1).fold(0, |acc, n| acc ^ n);
        let mut odd = 0;

        for i in (1..encoded.len()).step_by(2) {
            odd = odd ^ encoded[i];
        }

        let mut decoded = Vec::with_capacity(encoded.len() + 1);
        // push first prem
        decoded.push(total ^ odd);

        // For loop to resolve all prem
        for i in 0..encoded.len() {
            let encoded_num = encoded[i];
            let pre_decoded_num = decoded[i];
            decoded.push(encoded_num ^ pre_decoded_num);
        }

        decoded
    }

    // 1291. Sequential Digits
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut _low = low;
        let mut _high = high;
        // Create a vector with all digits
        let digits = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        // Find the min_window, and max_window
        let mut min_window = 1;
        let mut max_window = 1;
        while _low > 0 || _high > 0 {
            _low = _low / 10;
            if _low > 0 {
                min_window += 1;
            }

            _high = _high / 10;
            if _high > 0 {
                max_window += 1;
            }
        }

        // For loop: for each window try to get a num in [low, high], with sequential digit
        let mut result = Vec::new();
        for window in min_window..=max_window {
            for slice in digits.windows(window) {
                let num = slice.iter().fold(0, |acc, digit| acc * 10 + digit);
                if num >= low && num <= high {
                    result.push(num);
                }
            }
        }

        result
    }

    // 291. Word Pattern II
    pub fn word_pattern_match(pattern: String, s: String) -> bool {
        let mut map: HashMap<u8, &str> = HashMap::new();
        let mut used: HashSet<&str> = HashSet::new();
        Self::backtrack(pattern.as_bytes(), 0, s.as_str(), 0, &mut map, &mut used)
    }

    fn backtrack<'a>(
        pattern: &[u8],
        p_idx: usize,
        s: &'a str,
        s_idx: usize,
        map: &mut HashMap<u8, &'a str>,
        used: &mut HashSet<&'a str>,
    ) -> bool {
        if p_idx == pattern.len() && s_idx == s.len() {
            return true;
        }

        if p_idx == pattern.len() || s_idx == s.len() {
            return false;
        }

        let c = pattern[p_idx];

        if let Some(&w) = map.get(&c) {
            return s[s_idx..].starts_with(w)
                && Self::backtrack(pattern, p_idx + 1, s, s_idx + w.len(), map, used);
        }

        for end in (s_idx + 1)..=s.len() {
            let candidate = &s[s_idx..end];

            if used.contains(candidate) {
                continue;
            }

            map.insert(c, candidate);
            used.insert(candidate);

            if Self::backtrack(pattern, p_idx + 1, s, end, map, used) {
                return true;
            }

            map.remove(&c);
            used.remove(candidate);
        }
        false
    }
}

fn main() {
    let lists = vec![build(&[1, 4, 5]), build(&[1, 3, 4]), build(&[2, 6])];
    println!("{:?}", Solution::merge_k_lists(lists));

    let list = vec![1, 4, 2, 3, 1, 2];
    let is_duplicate = Solution::contains_nearby_duplicate(list, 3);
    println!("{}", is_duplicate);

    let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let k = Solution::remove_element(&mut nums, 2);
    println!("k={},{:?} ", k, nums);

    let mut nums = vec![0, 1, 0, 3, 12];
    Solution::move_zeros(&mut nums);
    println!("{:?}", nums);

    let nums = vec![1, 2, 2, 1, 1, 0];
    let nums = Solution::apply_operations(nums);
    println!("{:?}", nums);

    let encoded = vec![6, 5, 4, 6];
    let decoded = Solution::decode(encoded);
    println!("{:?}", decoded);

    let result = Solution::sequential_digits(100, 13000);
    println!("{:?}", result);

    let inputs = ["deer", "door", "cake", "card"].map(String::from).to_vec();
    let valid_word_abbr = solutions::unique_word_abbreviation::ValidWordAbbr::new(inputs);
    println!("{:?}", valid_word_abbr.is_unique("deer".to_string()));

    let result = Solution::word_pattern_match("abab".to_string(), "redblueredblue".to_string());
    println!("{:?}", result);
}
