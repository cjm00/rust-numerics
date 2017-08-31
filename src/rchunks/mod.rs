pub struct RChunksIter<'a, T: 'a> {
    v: &'a [T],
    size: usize,
    upper: usize,
    lower: usize,
}

impl<'a, T> Iterator for RChunksIter<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<&'a [T]> {
        if self.upper == 0 {
            None
        } else {
            let upper = self.upper;
            let lower = self.lower;
            self.upper = self.upper.saturating_sub(self.size);
            self.lower = self.lower.saturating_sub(self.size);
            Some(&self.v[lower..upper])
        }
    }
}

pub trait RChunks {
    type Item;
    fn rchunks<'a>(&'a self, size: usize) -> RChunksIter<'a, Self::Item>;
}

impl<'a, T> RChunks for &'a [T] {
    type Item = T;
    fn rchunks<'b>(&'b self, size: usize) -> RChunksIter<'b, T> {
        RChunksIter {
            v: self,
            size: size,
            upper: self.len(),
            lower: self.len().saturating_sub(size),
        }
    }
}

#[test]
fn rchunks_test_1() {
    let s = vec![0usize, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let q = s.as_slice();
    let mut s_iter = q.rchunks(3);

    assert_eq!(s_iter.next().unwrap(), &[7usize, 8, 9]);
    assert_eq!(s_iter.next().unwrap(), &[4usize, 5, 6]);
    assert_eq!(s_iter.next().unwrap(), &[1usize, 2, 3]);
    assert_eq!(s_iter.next().unwrap(), &[0usize]);
    assert!(s_iter.next().is_none());

}
