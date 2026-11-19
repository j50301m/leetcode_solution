struct Solution {}

impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        envelopes.sort_by_key(|x| (x[0], -x[1]));
        let mut tails = Vec::new();
        for doll in envelopes.iter() {
            let l = doll[1];
            let pos = tails.partition_point(|&x| x < l);
            if pos == tails.len() {
                tails.push(l);
            } else {
                tails[pos] = l;
            }
        }
        tails.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::max_envelopes(vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]]),
            3
        );
    }
}
