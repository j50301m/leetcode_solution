struct Solution {}

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut least_spells = Vec::new();
        for potion in potions {
            let potion = potion as i64;
            if success % potion != 0 {
                least_spells.push((success / potion) + 1);
            } else {
                least_spells.push(success / potion);
            }
        }
        least_spells.sort_unstable();

        let mut result = Vec::new();
        for spell in spells {
            let spell = spell as i64;
            let count = least_spells.iter().filter(|&x| *x > spell).count();
            result.push(count as i32);
        }

        result
    }
}
