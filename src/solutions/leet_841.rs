struct Solution {}

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let n = rooms.len();
        let mut visited = vec![false; n];
        let mut queue = Vec::new();
        queue.push((&rooms[0], 0));

        while let Some((room, room_id)) = queue.pop() {
            for &key in room {
                if !visited[key as usize] {
                    queue.push((&rooms[key as usize], key));
                }
            }
            visited[room_id as usize] = true;
        }

        visited.iter().all(|&x| x == true)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::can_visit_all_rooms(vec![vec![1], vec![2], vec![3], Vec::new()]),
            true
        )
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::can_visit_all_rooms(vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]]),
            false
        )
    }
}
