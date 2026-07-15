pub struct Solution;

impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        // fn gcd(a: i32, b: i32) -> i32 {
        //     if b == 0 {
        //         return a;
        //     }
        //     gcd(b, a % b)
        // }

        fn gcd(mut a: i32, mut b: i32) -> i32 {
            while b != 0 {
                let t = a % b;
                a = b;
                b = t;
            }
            a
        }

        let mut max = 0;
        let mut min = 1001;

        // Solve the min and max val
        for i in nums {
            if i > max {
                max = i;
            }
            if i < min {
                min = i;
            }
        }

        gcd(max, min)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let nums = vec![2, 5, 6, 9, 10];
        let result = Solution::find_gcd(nums);
        println!("{:?}", result);
    }
}
