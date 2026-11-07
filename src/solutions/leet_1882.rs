use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution {}

impl Solution {
    pub fn assign_tasks(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
        let (n, m) = (servers.len(), tasks.len());
        let mut occupied: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new(); // time,id
        let mut released: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new(); // weight,id

        for i in 0..n {
            let weight = servers[i];
            released.push(Reverse((weight, i)));
        }

        let mut ans = vec![0; m];
        let mut curr_time = 0;
        for i in 0..m {
            let spend_time = tasks[i] as usize;

            curr_time = curr_time.max(i);
            while let Some(&Reverse((time, id))) = occupied.peek() {
                if time > curr_time {
                    break;
                }
                occupied.pop();
                released.push(Reverse((servers[id], id)));
            }

            let Some(Reverse((_weight, id))) = released.pop().or_else(|| {
                let &Reverse((closed_time, _id)) = occupied.peek().unwrap();
                curr_time = closed_time;
                while let Some(&Reverse((time, id))) = occupied.peek() {
                    if time > curr_time {
                        break;
                    }
                    occupied.pop();
                    released.push(Reverse((servers[id], id)));
                }

                released.pop()
            }) else {
                unimplemented!()
            };

            occupied.push(Reverse((curr_time + spend_time, id)));
            ans[i] = id as i32;
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
            Solution::assign_tasks(vec![3, 3, 2], vec![1, 2, 3, 2, 1, 2]),
            vec![2, 2, 0, 2, 1, 2]
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::assign_tasks(vec![5, 1, 4, 3, 2], vec![2, 1, 2, 4, 5, 2, 1]),
            vec![1, 4, 1, 4, 1, 3, 2]
        );
    }

    // 同權重 → 小 index 優先
    #[test]
    fn same_weight_tie_break_by_index() {
        assert_eq!(Solution::assign_tasks(vec![1, 1], vec![5, 5]), vec![0, 1]);
    }

    // 等待中排了兩個任務，兩台同時空出來 → 同一秒兩個都要派出去
    #[test]
    fn backlog_assigns_multiple_tasks_same_second() {
        assert_eq!(
            Solution::assign_tasks(vec![1, 2], vec![3, 2, 1, 1, 1]),
            vec![0, 1, 0, 1, 0]
        );
    }

    // 全忙時，同一秒空出兩台 → 要比 weight，不是比 id
    #[test]
    fn wait_then_pick_lightest_among_same_free_time() {
        assert_eq!(
            Solution::assign_tasks(vec![2, 1], vec![3, 2, 1]),
            vec![1, 0, 1]
        );
    }

    // 唯一的 server 忙碌時，第二個任務要等它空出來
    #[test]
    fn wait_for_busy_server() {
        assert_eq!(
            Solution::assign_tasks(vec![7], vec![3, 1, 1]),
            vec![0, 0, 0]
        );
    }
}
