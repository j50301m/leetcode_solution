pub struct Solution;

impl Solution {
    pub fn gcd_of_odd_even_sums(n: i32) -> i32 {
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                return a;
            }
            gcd(b, a % b)
        }

        let mut odd = 0;
        let mut even = 0;

        // Sum of odd and even
        for i in 1..=2 * n {
            let remainder = i % 2;
            match remainder {
                0 => {
                    even += i;
                }
                1 => {
                    odd += i;
                }
                _ => {}
            }
        }

        gcd(odd, even)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let result = Solution::gcd_of_odd_even_sums(5);
        println!("{:?}", result);
    }
}
