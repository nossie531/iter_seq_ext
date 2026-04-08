//! Providef of [Replace].

use crate::*;

/// An iterator that replacing sequences.
#[derive(Clone, Debug)]
#[must_use = msg::iter_must_use!()]
pub struct Replace<'a, I, T> {
    /// Base iterator.
    base: I,
    /// Target sequence.
    ptn: &'a [T],
    /// Alternative sequence.
    alt: &'a [T],
    /// Read point branch.
    branch: &'a [T],
    /// Last readed item.
    last: Option<T>,
    /// `true` if branch tail is poped.
    tail_poped: bool,
}

impl<'a, I, T> Replace<'a, I, T> {
    /// Creates a new instance.
    pub(super) fn new(base: I, ptn: &'a [T], alt: &'a [T]) -> Self {
        Self {
            base,
            ptn,
            alt,
            branch: &[],
            last: None,
            tail_poped: false,
        }
    }

    /// Returns front item that remains in iterator.
    fn pop_remain(&mut self) -> Option<I::Item>
    where
        I: Iterator<Item = T>,
        T: Clone,
    {
        if self.branch.len() > 0 {
            let item = self.branch[0].clone();
            self.branch = &self.branch[1..];
            self.tail_poped = self.branch.is_empty() && self.last.is_none();
            return Some(item);
        }

        if self.last.is_some() {
            return self.last.take();
        }

        None
    }
}

impl<I, T> Iterator for Replace<'_, I, T>
where
    I: Iterator<Item = T>,
    T: Eq + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.pop_remain() {
            return Some(item);
        }

        if self.tail_poped {
            return None;
        }

        loop {
            let item = self.base.next();
            let empty_ptn = self.ptn.is_empty();
            let sub_hit = empty_ptn || item.as_ref() == self.ptn.get(self.branch.len());
            let all_hit = sub_hit && self.branch.len() + 1 >= self.ptn.len();

            if !sub_hit {
                self.last = item;
                break;
            }
            
            if all_hit {
                self.branch = &self.alt;
                self.last = if !empty_ptn { self.base.next() } else { item };
                break;
            }

            self.branch = &self.ptn[0..self.branch.len() + 1];
        }

        self.pop_remain()
    }
}
