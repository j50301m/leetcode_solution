struct Solution;

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();

        for num in asteroids {
            if num > 0 {
                stack.push(num);
            } else {
                let mut need_push = true;
                while let Some(&last) = stack.last() {
                    if last < 0 {
                        break;
                    } else if last == num.abs() {
                        need_push = false;
                        stack.pop();
                        break;
                    } else if last > num.abs() {
                        need_push = false;
                        break;
                    } else {
                        stack.pop();
                    }
                }

                if need_push {
                    stack.push(num);
                }
            }
        }

        stack
    }

    fn asteroid_collision_1(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<i32> = Vec::new();

        'next: for a in asteroids {
            while let Some(&top) = stack.last() {
                if top < 0 || a > 0 {
                    break; // 反向飛開，不會撞
                }
                if top >= -a {
                    if top == -a {
                        stack.pop(); // 同大小，同歸於盡
                    }
                    continue 'next; // a 被撞掉，不入棧
                }
                stack.pop(); // top 較小被撞掉，繼續往下比
            }
            stack.push(a);
        }

        stack
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::asteroid_collision(vec![5, 10, -5]), vec![5, 10]);
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::asteroid_collision(vec![3, 5, -6, 2, -1, 4]),
            vec![-6, 2, 4]
        );
    }
}
