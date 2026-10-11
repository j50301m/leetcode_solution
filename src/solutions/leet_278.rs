struct Solution {}

// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    fn is_bad_version(&self, version: i32) -> bool {
        version == 4
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;

        while left < right {
            let mid = left + (right - left) / 2;

            if self.is_bad_version(mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
    }
}
