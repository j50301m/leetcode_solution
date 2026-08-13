struct Solution;

#[derive(Clone, Copy, Default)]
struct Node {
    pre: u32,
    suf: u32,
    best: u32,
}

struct Seg {
    t: Vec<Node>,
    s: Vec<u8>,
}

impl Seg {
    fn new(s: Vec<u8>) -> Self {
        let n = s.len();
        let mut seg = Seg {
            t: vec![Node::default(); 4 * n],
            s,
        };
        seg.build(1, 0, n - 1);
        seg
    }

    fn pull(&mut self, node: usize, l: usize, mid: usize, r: usize) {
        let (a, b) = (self.t[node * 2], self.t[node * 2 + 1]);
        let (a_len, b_len) = ((mid - l + 1) as u32, (r - mid) as u32);

        let mut best = a.best.max(b.best);
        let mut pre = a.pre;
        let mut suf = b.suf;

        if self.s[mid] == self.s[mid + 1] {
            best = best.max(a.suf + b.pre);
            // 整段同字元才能穿過分界線延伸
            if a.pre == a_len {
                pre = a_len + b.pre;
            }
            if b.suf == b_len {
                suf = b_len + a.suf;
            }
        }

        self.t[node] = Node { pre, suf, best };
    }

    fn build(&mut self, node: usize, l: usize, r: usize) {
        if l == r {
            self.t[node] = Node {
                pre: 1,
                suf: 1,
                best: 1,
            };
            return;
        }
        let mid = (l + r) / 2;
        self.build(node * 2, l, mid);
        self.build(node * 2 + 1, mid + 1, r);
        self.pull(node, l, mid, r);
    }

    fn update(&mut self, node: usize, l: usize, r: usize, i: usize) {
        // 葉節點永遠是 (1, 1, 1)，換字元也不變，只要重算祖先
        if l == r {
            return;
        }
        let mid = (l + r) / 2;
        if i <= mid {
            self.update(node * 2, l, mid, i);
        } else {
            self.update(node * 2 + 1, mid + 1, r, i);
        }
        self.pull(node, l, mid, r);
    }
}

impl Solution {
    pub fn longest_repeating(
        s: String,
        query_characters: String,
        query_indices: Vec<i32>,
    ) -> Vec<i32> {
        let n = s.len();
        let mut seg = Seg::new(s.into_bytes());

        query_characters
            .bytes()
            .zip(query_indices)
            .map(|(c, i)| {
                let i = i as usize;
                seg.s[i] = c; // 先改字串，pull 才讀得到新字元
                seg.update(1, 0, n - 1, i);
                seg.t[1].best as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::longest_repeating("babacc".into(), "bcb".into(), vec![1, 3, 3]),
            vec![3, 3, 4]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::longest_repeating("abyzz".into(), "aa".into(), vec![2, 1]),
            vec![2, 3]
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::longest_repeating("a".into(), "b".into(), vec![0]),
            vec![1]
        );
    }

    #[test]
    fn case_4() {
        // 換成一樣的字元，答案不變
        assert_eq!(
            Solution::longest_repeating("aaa".into(), "a".into(), vec![1]),
            vec![3]
        );
    }

    #[test]
    fn case_5() {
        // 一刀把最長段切開，再接回來
        assert_eq!(
            Solution::longest_repeating("aaaaa".into(), "ba".into(), vec![2, 2]),
            vec![2, 5]
        );
    }

    fn brute(s: &[u8]) -> i32 {
        let mut best = 1;
        let mut run = 1;
        for i in 1..s.len() {
            run = if s[i] == s[i - 1] { run + 1 } else { 1 };
            best = best.max(run);
        }
        best
    }

    fn xorshift(seed: &mut u64) -> u64 {
        *seed ^= *seed << 13;
        *seed ^= *seed >> 7;
        *seed ^= *seed << 17;
        *seed
    }

    #[test]
    fn random_vs_brute() {
        let mut seed: u64 = 88172645463325252;
        for _ in 0..300 {
            let n = 1 + (xorshift(&mut seed) % 12) as usize;
            let k = 1 + (xorshift(&mut seed) % 10) as usize;
            let s: String = (0..n)
                .map(|_| (b'a' + (xorshift(&mut seed) % 3) as u8) as char)
                .collect();
            let qc: String = (0..k)
                .map(|_| (b'a' + (xorshift(&mut seed) % 3) as u8) as char)
                .collect();
            let qi: Vec<i32> = (0..k)
                .map(|_| (xorshift(&mut seed) % n as u64) as i32)
                .collect();

            let got = Solution::longest_repeating(s.clone(), qc.clone(), qi.clone());

            let mut cur = s.clone().into_bytes();
            let want: Vec<i32> = qc
                .bytes()
                .zip(&qi)
                .map(|(c, &i)| {
                    cur[i as usize] = c;
                    brute(&cur)
                })
                .collect();

            assert_eq!(got, want, "s={s} qc={qc} qi={qi:?}");
        }
    }
}
