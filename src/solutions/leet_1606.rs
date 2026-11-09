use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap},
};

struct Solution {}

impl Solution {
    pub fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
        let mut released: BTreeSet<i32> = BTreeSet::new();
        let mut occupied: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new(); // time, id
        let mut processed = vec![0; k as usize];
        for i in 0..k {
            released.insert(i);
        }

        for i in 0..arrival.len() {
            let (arr_time, spend_time) = (arrival[i], load[i]);

            // Popup expired task,ans release the server
            while let Some(&Reverse((closed_time, id))) = occupied.peek() {
                if closed_time > arr_time {
                    break;
                }
                occupied.pop();
                released.insert(id);
            }

            let start = (i % k as usize) as i32;
            let Some(id) = released
                .range(start..)
                .next()
                .or_else(|| released.iter().next())
                .copied()
            else {
                continue;
            };

            released.remove(&id);
            occupied.push(Reverse((arr_time + spend_time, id)));
            processed[id as usize] += 1;
        }

        let mut processed = processed
            .into_iter()
            .enumerate()
            .map(|(id, count)| (id as i32, count))
            .collect::<Vec<(i32, i32)>>();
        processed.sort_by_key(|(_id, count)| Reverse(*count));
        let max = processed[0].1;
        let pos = processed.partition_point(|(_id, count)| *count == max);
        let mut results = vec![0i32; pos];
        for i in 0..pos {
            results[i] = processed[i].0;
        }

        return results;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sorted(mut ids: Vec<i32>) -> Vec<i32> {
        ids.sort();
        ids
    }

    #[test]
    fn case1() {
        assert_eq!(
            sorted(Solution::busiest_servers(
                3,
                vec![1, 2, 3, 4, 5],
                vec![5, 2, 3, 3, 3]
            )),
            vec![1]
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            sorted(Solution::busiest_servers(
                3,
                vec![1, 2, 3, 4],
                vec![1, 2, 1, 2]
            )),
            vec![0]
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            sorted(Solution::busiest_servers(
                3,
                vec![1, 2, 3],
                vec![10, 12, 11]
            )),
            vec![0, 1, 2]
        );
    }

    // 第 1 個請求 (i=1) 應該從 server 1 開始找，不是挑編號最小的 server 0
    #[test]
    fn case4_start_from_i_mod_k() {
        assert_eq!(
            sorted(Solution::busiest_servers(2, vec![1, 2], vec![1, 1])),
            vec![0, 1]
        );
    }
}
