/*
 * @lc app=leetcode id=12 lang=rust
 *
 * [12] Integer to Roman
 */

// @lc code=start
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut result: Vec<&str> = Vec::new();
        let (num, mut thousand) = convert_thousand(num);
        result.append(&mut thousand);

        let (num, mut hundred) = convert_hundred(num);
        result.append(&mut hundred);

        let (num, mut ten) = convert_ten(num);
        result.append(&mut ten);

        let (_, mut signle) = convert_single(num);
        result.append(&mut signle);

        result.concat()
    }
}

fn convert_thousand(mut num: i32) -> (i32, Vec<&'static str>) {
    let mut result: Vec<&str> = Vec::new();
    while num >= 1000 {
        num -= 1000;
        result.push("M");
    }
    return (num, result);
}

fn convert_hundred(mut num: i32) -> (i32, Vec<&'static str>) {
    let mut result: Vec<&str> = Vec::new();
    while num >= 100 {
        if num >= 900 {
            num -= 900;
            result.push("CM");
        } else if num >= 500 {
            num -= 500;
            result.push("D");
        } else if num >= 400 {
            num -= 400;
            result.push("CD");
        } else {
            num -= 100;
            result.push("C");
        }
    }
    (num, result)
}

fn convert_ten(mut num: i32) -> (i32, Vec<&'static str>) {
    let mut result: Vec<&str> = Vec::new();
    while num >= 10 {
        if num >= 90 {
            num -= 90;
            result.push("XC");
        } else if num >= 50 {
            num -= 50;
            result.push("L");
        } else if num >= 40 {
            num -= 40;
            result.push("XL");
        } else {
            num -= 10;
            result.push("X");
        }
    }
    (num, result)
}

fn convert_single(mut num: i32) -> (i32, Vec<&'static str>) {
    let mut result: Vec<&str> = Vec::new();
    while num > 0 {
        if num >= 9 {
            num -= 9;
            result.push("IX");
        } else if num >= 5 {
            num -= 5;
            result.push("V");
        } else if num >= 4 {
            num -= 4;
            result.push("IV");
        } else {
            num -= 1;
            result.push("I");
        }
    }
    (num, result)
}

// @lc code=end
