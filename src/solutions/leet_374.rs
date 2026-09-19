/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */
use std::sync::atomic::{AtomicI32, Ordering};

static PICK: AtomicI32 = AtomicI32::new(0);
static CALLS: AtomicI32 = AtomicI32::new(0);

struct Solution {}

impl Solution {
    unsafe fn guess_number(n: i32) -> i32 {
        let (mut lo, mut hi) = (1, n);
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            match Self::guess(mid) {
                -1 => hi = mid - 1,
                1 => lo = mid + 1,
                0 => {
                    return mid;
                }
                _ => unreachable!("unreachable"),
            }
        }
        0
    }

    // ponytail: 測試用替身。PICK/CALLS 是全域的，測試只能單執行緒跑（cargo test 預設會平行，
    // 所以所有 case 寫在同一個 #[test] 裡）。真要平行再換 thread_local。
    fn guess(num: i32) -> i32 {
        assert!(
            CALLS.fetch_add(1, Ordering::Relaxed) < 64,
            "guess() 被呼叫超過 64 次 —— 二分搜尋不該超過 ~31 次，代表迴圈收斂不了"
        );
        let n = PICK.load(Ordering::Relaxed);
        if num > n {
            -1
        } else if num < n {
            1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn run(n: i32, pick: i32) -> i32 {
        PICK.store(pick, Ordering::Relaxed);
        CALLS.store(0, Ordering::Relaxed);
        unsafe { Solution::guess_number(n) }
    }

    #[test]
    fn guess_number_works() {
        assert_eq!(run(10, 6), 6);
        assert_eq!(run(1, 1), 1);
        assert_eq!(run(2, 1), 1, "答案在最左邊");
        assert_eq!(run(2, 2), 2, "答案在最右邊");
        assert_eq!(run(100, 72), 72);
        assert_eq!(run(i32::MAX, 1), 1, "上界極大，不能有 lo+hi 溢位");
        assert_eq!(run(i32::MAX, i32::MAX), i32::MAX);
    }
}
