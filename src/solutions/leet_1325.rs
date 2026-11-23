struct ProductOfNumbers {
    prefix: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {
    fn new() -> Self {
        Self { prefix: Vec::new() }
    }

    fn add(&mut self, num: i32) {
        if num == 0 {
            self.prefix.clear();
            return;
        }

        match self.prefix.last() {
            Some(&last) => {
                self.prefix.push(last * num);
            }
            None => {
                self.prefix.push(num);
            }
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        let k = k as usize;
        let n = self.prefix.len();
        if k > n {
            return 0;
        }

        self.prefix[n - 1] / self.prefix[n - k - 1]
    }
}
