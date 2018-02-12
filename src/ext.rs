use ::{Comparer, ComparerMut};
/// Extra methods for slices to compare with self
pub trait SliceCompareExt<T> {
    /// Returns a `Comparer` from a reference to a slice
    fn self_comparer(&self) -> Comparer<T>;
    /// Returns a `Comparer` from a reference to a mutable slice
    fn self_comparer_mut(&mut self) -> ComparerMut<T>;
    /// Convenience function for itereating through the `Comparer` and applying a function to each pair
    fn compare_self<F: FnMut(&T, &T)>(&self, mut f: F) {
        for (a, b) in self.self_comparer() {
            f(a, b)
        }
    }
    /// Same as `compare_self()` but also parses the indices of the elements to the function
    fn compare_self_enumerated<F: FnMut((usize, &T), (usize, &T))>(&self, mut f: F) {
        let mut c = self.self_comparer();
        while let Some((a, b)) = c.next_enumerated() {
            f(a, b);
        }
    }
    /// Convenience function for itereating through the `ComparerMut` and applying a function to each pair
    fn compare_self_mut<F: FnMut(&mut T, &mut T)>(&mut self, mut f: F) {
        let mut c = self.self_comparer_mut();
        while let Some((a, b)) = c.next() {
           f(a, b)
        }
    }
    /// Same as `compare_self_mut()` but also parses the indices of the elements to the function
    fn compare_self_enumerated_mut<F: FnMut((usize, &mut T), (usize, &mut T))>(&mut self, mut f: F) {
        let mut c = self.self_comparer_mut();
        while let Some((a, b)) = c.next_enumerated() {
            f(a, b);
        }
    }
}

impl<T> SliceCompareExt<T> for [T] {
    fn self_comparer(&self) -> Comparer<T> {
        Comparer::new(self)
    }
    fn self_comparer_mut(&mut self) -> ComparerMut<T> {
        ComparerMut::new(self)
    }
}
