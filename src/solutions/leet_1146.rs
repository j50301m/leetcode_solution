/**
 * Your SnapshotArray object will be instantiated and called as such:
 * let obj = SnapshotArray::new(length);
 * obj.set(index, val);
 * let ret_2: i32 = obj.snap();
 * let ret_3: i32 = obj.get(index, snap_id);
 */
struct SnapshotArray {
    history: Vec<Vec<(i32, i32)>>, // (ver ,val)
    ver: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SnapshotArray {
    fn new(length: i32) -> Self {
        Self {
            history: vec![vec![(0, 0)]; length as usize],
            ver: 0,
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        match self.history[index as usize].binary_search_by_key(&self.ver, |(ver, _)| *ver) {
            Ok(pos) => {
                self.history[index as usize][pos] = (self.ver, val);
            }
            Err(_) => {
                self.history[index as usize].push((self.ver, val));
            }
        }
    }

    fn snap(&mut self) -> i32 {
        self.ver += 1;
        self.ver - 1
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let h = &self.history[index as usize];
        h[h.partition_point(|&(ver, _)| ver <= snap_id) - 1].1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let mut sa = SnapshotArray::new(3);
        sa.set(0, 5);
        assert_eq!(sa.snap(), 0);
        sa.set(0, 6);
        assert_eq!(sa.get(0, 0), 5);
    }

    #[test]
    fn never_set_index() {
        let mut sa = SnapshotArray::new(2);
        sa.snap();
        assert_eq!(sa.get(1, 0), 0);
    }

    #[test]
    fn overwrite_in_same_snap() {
        let mut sa = SnapshotArray::new(1);
        sa.set(0, 5);
        sa.set(0, 7);
        sa.snap();
        assert_eq!(sa.get(0, 0), 7);
    }

    #[test]
    fn unchanged_snaps_use_previous_value() {
        let mut sa = SnapshotArray::new(1);
        sa.set(0, 7);
        sa.snap(); // 0
        sa.snap(); // 1
        sa.set(0, 9);
        sa.snap(); // 2
        assert_eq!(sa.get(0, 0), 7);
        assert_eq!(sa.get(0, 1), 7);
        assert_eq!(sa.get(0, 2), 9);
    }

    #[test]
    fn set_before_first_change() {
        let mut sa = SnapshotArray::new(1);
        sa.snap(); // 0
        sa.set(0, 4);
        sa.snap(); // 1
        assert_eq!(sa.get(0, 0), 0);
        assert_eq!(sa.get(0, 1), 4);
    }
}
