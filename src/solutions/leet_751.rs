struct Solution {}

impl Solution {
    pub fn ip_to_cidr(ip: String, n: i32) -> Vec<String> {
        let mut n = n as u64;
        let chunks = ip.split('.');
        let mut x = 0;
        for num in chunks {
            x = (x << 8) | num.parse::<u64>().unwrap();
        }

        let mut result = Vec::new();
        while n > 0 {
            let mut size = x & x.wrapping_neg(); // lowbit
            if size == 0 {
                size = 1 << 32;
            }

            while size > n {
                size /= 2;
            }
            let prefix = 32 - size.ilog2();
            result.push(Self::to_ip(x, prefix));
            x += size;
            n -= size;
        }

        result
    }

    fn to_ip(x: u64, prefix: u32) -> String {
        let part1 = x >> 24 & 255;
        let part2 = x >> 16 & 255;
        let part3 = x >> 8 & 255;
        let part4 = x & 255;
        format!("{}.{}.{}.{}/{}", part1, part2, part3, part4, prefix)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn run(ip: &str, n: i32) -> Vec<String> {
        Solution::ip_to_cidr(ip.to_string(), n)
    }

    #[test]
    fn case1() {
        assert_eq!(
            run("255.0.0.7", 10),
            vec!["255.0.0.7/32", "255.0.0.8/29", "255.0.0.16/32"]
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            run("117.145.102.62", 8),
            vec![
                "117.145.102.62/31",
                "117.145.102.64/30",
                "117.145.102.68/31"
            ]
        );
    }

    // 中間要切成好幾塊，不是「頭 + 中 + 尾」三段
    #[test]
    fn middle_multiple_blocks() {
        assert_eq!(
            run("0.0.0.1", 14),
            vec![
                "0.0.0.1/32",
                "0.0.0.2/31",
                "0.0.0.4/30",
                "0.0.0.8/30",
                "0.0.0.12/31",
                "0.0.0.14/32"
            ]
        );
    }

    #[test]
    fn single_ip() {
        assert_eq!(run("1.2.3.4", 1), vec!["1.2.3.4/32"]);
    }

    // 最後一段 255 + 1 要進位到第三段
    #[test]
    fn carry_to_next_byte() {
        assert_eq!(run("0.0.0.255", 3), vec!["0.0.0.255/32", "0.0.1.0/31"]);
    }

    // 起點剛好對齊，一整塊蓋完
    #[test]
    fn aligned_full_block() {
        assert_eq!(run("10.0.0.0", 256), vec!["10.0.0.0/24"]);
    }

    // x = 0 時 lowbit 是 0，要當成 2^32
    #[test]
    fn zero_ip() {
        assert_eq!(run("0.0.0.0", 1), vec!["0.0.0.0/32"]);
    }

    #[test]
    fn zero_ip_block() {
        assert_eq!(run("0.0.0.0", 256), vec!["0.0.0.0/24"]);
    }
}
