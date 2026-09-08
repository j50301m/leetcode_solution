use std::collections::BTreeMap;

struct Solution {}

impl Solution {
    pub fn high_five(items: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut map = BTreeMap::new();
        for item in items.into_iter() {
            let id = item[0];
            let score = item[1];

            map.entry(id).or_insert(Vec::new()).push(score);
        }

        let mut result = Vec::new();
        for (&id, scores) in map.iter_mut() {
            scores.sort_by(|a, b| b.cmp(a));
            let n = 5.min(scores.len());
            let mut sum = 0;
            for i in 0..n {
                sum += scores[i];
            }
            result.push(vec![id, sum / n as i32]);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // 幫某位學生生成 n 筆成績，湊「每人至少五筆」的測資用
    fn rows(id: i32, scores: &[i32]) -> Vec<Vec<i32>> {
        scores.iter().map(|&s| vec![id, s]).collect()
    }

    // LeetCode 範例一。
    // id 1 有六筆（60 那筆要被擠掉），id 2 剛好五筆而且平均是 88.6 —— 一次踩到兩個重點。
    #[test]
    fn case1_example() {
        let items = vec![
            vec![1, 91],
            vec![1, 92],
            vec![2, 93],
            vec![2, 97],
            vec![1, 60],
            vec![2, 77],
            vec![1, 65],
            vec![1, 87],
            vec![1, 100],
            vec![2, 100],
            vec![2, 76],
        ];
        // id1: 100+92+91+87+65 = 435 / 5 = 87
        // id2: 100+97+93+77+76 = 443 / 5 = 88.6 -> 88
        assert_eq!(Solution::high_five(items), vec![vec![1, 87], vec![2, 88]]);
    }

    // 輸出必須依 id 由小到大。
    // 換成 HashMap 就會爆這題；用字串排序也會（"10" < "2"）。
    #[test]
    fn case2_sorted_by_id() {
        let mut items = rows(10, &[100; 5]);
        items.extend(rows(2, &[50; 5]));
        items.extend(rows(7, &[80; 5]));
        assert_eq!(
            Solution::high_five(items),
            vec![vec![2, 50], vec![7, 80], vec![10, 100]]
        );
    }

    // 只取前五高。
    // 排序方向寫反的話這裡會變成 (1+2+3+4+5)/5 = 3，差距大到不可能看漏。
    #[test]
    fn case3_only_top_five() {
        let items = rows(1, &[1, 2, 3, 4, 5, 100, 100, 100, 100, 100]);
        assert_eq!(Solution::high_five(items), vec![vec![1, 100]]);
    }

    // 整數除法是無條件捨去，不是四捨五入。
    // 449 / 5 = 89.8 -> 89；寫成 round 就會回 90。
    #[test]
    fn case4_truncating_division() {
        let items = rows(1, &[90, 90, 90, 90, 89]);
        assert_eq!(Solution::high_five(items), vec![vec![1, 89]]);
    }

    // 重複分數不能被當成同一筆吃掉。
    // 六個 80 一個 0：正確答案 80；若不小心去重，前五高會湊進那個 0 變成 64。
    #[test]
    fn case5_duplicate_scores() {
        let items = rows(1, &[80, 80, 80, 80, 80, 80, 0]);
        assert_eq!(Solution::high_five(items), vec![vec![1, 80]]);
    }

    // 同一個 id 的紀錄散在輸入各處，不保證相鄰。
    // 只看「前一筆的 id」來分組的寫法會在這裡碎成一堆。
    #[test]
    fn case6_interleaved_ids() {
        let items = vec![
            vec![1, 100],
            vec![2, 0],
            vec![1, 100],
            vec![2, 0],
            vec![1, 100],
            vec![2, 0],
            vec![1, 100],
            vec![2, 0],
            vec![1, 100],
            vec![2, 0],
        ];
        assert_eq!(Solution::high_five(items), vec![vec![1, 100], vec![2, 0]]);
    }

    // 5.min(len) 那條分支：題目保證每人至少五筆，所以這是超出約束的防禦性測試。
    // 留著是因為程式碼寫了這個分支 —— 分母要跟著變成 3，不是永遠除以 5。
    #[test]
    fn case7_fewer_than_five_scores() {
        let items = rows(1, &[10, 20, 30]);
        assert_eq!(Solution::high_five(items), vec![vec![1, 20]]);
    }
}
