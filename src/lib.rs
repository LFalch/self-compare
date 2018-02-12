#![warn(missing_docs)]
//! A crate for comparing all elements of a slice with themselves

mod ext;
pub use ext::*;

/// A structure for immutably comparing the elemnts of a slice with themselves
///
/// Implements `Iterator` since the references don't need to be unique
pub struct Comparer<'a, T: 'a> {
    list: &'a [T],
    i: usize,
    j: usize,
}

impl<'a, T: 'a> Comparer<'a , T> {
    /// Returns a `Comparer`
    pub fn new(list: &'a [T]) -> Self {
        Comparer {
            list,
            i: 0,
            j: 1,
        }
    }
    /// Returns the inner slice
    pub fn inner(&self) -> &[T] {
        self.list
    }
    /// Returns the indices of the next two elements to be compared
    pub fn indices(&self) -> (usize, usize) {
        (self.i, self.j)
    }
    /// Same as `next()` but also returns the indices of the elements
    pub fn next_enumerated(&mut self) -> Option<((usize, &T), (usize, &T))> {
        let (i, j) = (self.i, self.j);
        self.next().map(|(a, b)| ((i, a), (j, b)))
    }
}

impl<'a, T: 'a> Iterator for Comparer<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.list.is_empty() || self.i == self.list.len()-1 {
            return None;
        }
        let (x, y) = (self.i, self.j);

        self.j += 1;
        if self.j == self.list.len() {
            self.i += 1;
            self.j = self.i + 1;
        }

        Some((&self.list[x], &self.list[y]))
    }
}

/// A structure for mutably comparing the elements of a slice with themselves
pub struct ComparerMut<'a, T: 'a> {
    list: &'a mut [T],
    i: usize,
    j: usize,
}

impl<'a, T: 'a> ComparerMut<'a, T> {
    /// Returns a `ComparerMut`
    pub fn new(list: &'a mut [T]) -> Self {
        ComparerMut {
            list,
            i: 0,
            j: 1,
        }
    }
    /// Returns the inner mutable slice
    pub fn inner(&mut self) -> &mut [T] {
        self.list
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

#[cfg(test)]
mod mut_tests {
    use super::*;

    #[test]
    fn empty() {
        let mut v: [i32; 0] = [];
        let mut c = v.self_comparer_mut();

        assert!(c.next().is_none())
    }
    #[test]
    fn one_element() {
        let mut v = [1];
        let mut c = v.self_comparer_mut();

        assert!(c.next().is_none())
    }
    #[test]
    fn two_elements() {
        let mut v = [1, 2];
        let mut c = v.self_comparer_mut();

        assert_eq!(c.next(), Some((&mut 1, &mut 2)));
        assert!(c.next().is_none());
    }
    #[test]
    fn compare_and_enumerate_yield_the_same() {
        let mut v: Vec<_> = (0..100).collect();

        let mut regular_comparisons = Vec::new();
        let mut enumerated_comparisons = Vec::new();

        v.compare_self_mut(|a, b| regular_comparisons.push((*a, *b)));
        v.compare_self_enumerated_mut(|(_, a), (_, b)| enumerated_comparisons.push((*a, *b)));

        assert_eq!(regular_comparisons, enumerated_comparisons)
    }
    #[test]
    fn all_compare_0() {
        all_compare(0);
    }
    #[test]
    fn all_compare_1() {
        all_compare(1);
    }
    #[test]
    fn all_compare_1000() {
        all_compare(1000);
    }
    fn all_compare(size: usize) {
        let mut vec = vec![Vec::with_capacity(size.saturating_sub(1)); size];

        vec.compare_self_enumerated_mut(|(i, a), (j, b)| {
            let id = a.binary_search(&j).unwrap_err();
            a.insert(id, j);
            let id = b.binary_search(&i).unwrap_err();
            b.insert(id, i);
        });

        assert!(vec
            .into_iter()
            .enumerate()
            .map(|(i, v)| v.into_iter().zip((0..size).filter(move |x| x != &i)))
            .all(|mut v| v.all(|(a, b)| a == b))
        );
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let v: [i32; 0] = [];
        let mut c = v.self_comparer();

        assert!(c.next().is_none())
    }
    #[test]
    fn one_element() {
        let v = [1];
        let mut c = v.self_comparer();

        assert!(c.next().is_none())
    }
    #[test]
    fn two_elements() {
        let v = [1, 2];
        let mut c = v.self_comparer();

        assert_eq!(c.next(), Some((&1, &2)));
        assert!(c.next().is_none());
    }
    #[test]
    fn compare_and_enumerate_yield_the_same() {
        let v: Vec<_> = (0..100).collect();

        let mut regular_comparisons = Vec::new();
        let mut enumerated_comparisons = Vec::new();

        v.compare_self(|a, b| regular_comparisons.push((*a, *b)));
        v.compare_self_enumerated(|(_, a), (_, b)| enumerated_comparisons.push((*a, *b)));

        assert_eq!(regular_comparisons, enumerated_comparisons)
    }
}
