#![warn(missing_docs)]
//! A crate for comparing all elements of a slice with themselves

/// A structure for comparing the elements of a slice with themselves
pub struct Comparer<'a, T: 'a> {
    list: &'a mut [T],
    i: usize,
    j: usize,
}

impl<'a, T: 'a> Comparer<'a, T> {
    /// Returns a `Comparer`
    pub fn new(list: &'a mut [T]) -> Self {
        Comparer {
            list,
            i: 0,
            j: 1,
        }
    }
    /// Returns the indices of the next two elements to be compared
    pub fn indices(&self) -> (usize, usize) {
        (self.i, self.j)
    }
    /// Optionally returns mutable reference to two elements until all elements have been compared
    pub fn next(&mut self) -> Option<(&mut T, &mut T)> {
        if self.list.is_empty() || self.i == self.list.len()-1 {
            return None;
        }
        let (x, y) = (self.i, self.j);

        self.j += 1;
        if self.j == self.list.len() {
            self.i += 1;
            self.j = self.i + 1;
        }

        Some(unsafe {
            (self.list.as_mut_ptr().offset(x as isize).as_mut().unwrap(),
            self.list.as_mut_ptr().offset(y as isize).as_mut().unwrap())
        })
    }
    /// Same as `next()` but also returns the indices of the elements
    pub fn next_enumerated(&mut self) -> Option<((usize, &mut T), (usize, &mut T))> {
        let (i, j) = (self.i, self.j);
        self.next().map(|(a, b)| ((i, a), (j, b)))
    }
}

/// Convenience function for calling `next` until `None` and applying a function to each pair
pub fn compare<'a, T: 'a, F: FnMut(&mut T, &mut T)>(l: &'a mut [T], mut f: F) {
    let mut c = Comparer::new(l);
    while let Some((a, b)) = c.next() {
        f(a, b)
    }
}

/// Same as `compare()` but also parses the indices of the elements to the function
pub fn compare_enumerated<'a, T: 'a, F: FnMut((usize, &mut T), (usize, &mut T))>(l: &'a mut [T], mut f: F) {
    let mut c = Comparer::new(l);
    while let Some((a, b)) = c.next_enumerated() {
        f(a, b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut v: [i32; 0] = [];
        let mut c = Comparer::new(&mut v);
        assert!(c.next().is_none())
    }
    #[test]
    fn one_element() {
        let mut v = [1];
        let mut c = Comparer::new(&mut v);
        assert!(c.next().is_none())
    }
    #[test]
    fn two_elements() {
        let mut v = [1, 2];
        let mut c = Comparer::new(&mut v);
        assert_eq!(c.next(), Some((&mut 1, &mut 2)));
        assert!(c.next().is_none());
    }
    // TODO Test that all elements of a bigger array are being compared
}
