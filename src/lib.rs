mod binary_heap;
pub use binary_heap::*;
extern crate core;

/// An intermediate trait for specialization of `Extend`.
// #[doc(hidden)]
// trait SpecExtend<I: IntoIterator> {
//     /// Extends `self` with the contents of the given iterator.
//     fn spec_extend(&mut self, iter: I);
// }


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
