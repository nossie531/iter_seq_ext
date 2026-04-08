//! Provider of [`IteratorSeqExt`].

use crate::Replace;

/// Iterator extension for sequential items.
pub trait IteratorSeqExt: Iterator {
    /// Returns `true` if this iterator has given item sequences.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use iter_seq_ext::prelude::*;
    /// 
    /// let iter = [1, 2, 3, 4].into_iter();
    /// let result = IteratorSeqExt::contains(iter, &[2, 3]);
    /// assert!(result);
    /// ```
    #[must_use]
    fn contains(self, pat: &[Self::Item]) -> bool
    where
        Self: Sized,
        Self::Item: Eq,
    {
        if pat.is_empty() {
            return true;
        }

        let mut match_count = 0;
        for item in self {
            if item != pat[match_count] {
                match_count = 0;
                continue;
            }

            match_count += 1;
            if match_count == pat.len() {
                return true;
            }
        }

        false
    }

    /// Creates an iterator that replace some pattern to other pattern.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use iter_seq_ext::prelude::*;
    /// 
    /// let iter = [1, 2, 3, 4].into_iter();
    /// let result = IteratorSeqExt::replace(iter, &[2, 3], &[-2, -3]);
    /// assert_eq!(result.collect::<Vec<_>>(), vec![1, -2, -3, 4]);
    /// ```
    #[inline]
    fn replace<'a>(
        self,
        pat: &'a [Self::Item],
        alt: &'a [Self::Item],
    ) -> Replace<'a, Self, Self::Item>
    where
        Self: Sized,
        Self::Item: Clone + Eq,
    {
        Replace::new(self, pat, alt)
    }
}

impl<T> IteratorSeqExt for T
where
    T: Iterator,
{
    // nop.
}
