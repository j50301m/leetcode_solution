struct Solution;

impl Solution {
    pub fn similar_rgb(color: String) -> String {
        let mut ans = "#".to_string();
        for i in [1, 3, 5] {
            let v = u8::from_str_radix(&color[i..i + 2], 16).unwrap() as u32;
            let k = (v + 8) / 17;
            ans.push_str(&format!("{:x}{:x}", k, k));
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::similar_rgb("#09f166".to_string()),
            "#11ee66".to_string()
        );
    }

    #[test]
    fn case2() {
        // 0x90 = 144：離 0x88 (136) 比離 0x99 (153) 近；0xff 是上界，別讓 k 變 16
        assert_eq!(
            Solution::similar_rgb("#9000ff".to_string()),
            "#8800ff".to_string()
        );
    }
}
