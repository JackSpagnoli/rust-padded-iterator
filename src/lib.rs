//! This crate provides an iterator that pads the end of the iterator with a value up to a given length.
//!
//! # Examples:
//!
//! ```
//! use padded_iterator::PaddedIterBuilder;
//!
//! let mut iter = PaddedIterBuilder::new()
//!     .iter(vec![1, 2, 3].into_iter())
//!     .length(5)
//!     .pad(0)
//!     .build()
//!     .unwrap();
//!
//! assert_eq!(iter.next(), Some(1));
//! assert_eq!(iter.next(), Some(2));
//! assert_eq!(iter.next(), Some(3));
//! assert_eq!(iter.next(), Some(0));
//! assert_eq!(iter.next(), Some(0));
//! assert_eq!(iter.next(), None);
//! ```
//!
//! ```
//! use padded_iterator::PaddedIter;
//!
//! let mut iter = PaddedIter::from((vec![1, 2, 3].into_iter(), 5, 0));
//!
//! assert_eq!(iter.next(), Some(1));
//! assert_eq!(iter.next(), Some(2));
//! assert_eq!(iter.next(), Some(3));
//! assert_eq!(iter.next(), Some(0));
//! assert_eq!(iter.next(), Some(0));
//! assert_eq!(iter.next(), None);
//! ```

mod utils;
#[cfg(test)]
mod tests;

pub use self::utils::BuildError;

/// An iterator which has been padded to a given length.
#[derive(Debug)]
pub struct PaddedIter<T: Iterator<Item = U>, U> {
    iter: T,
    n: usize,
    length: usize,
    pad: U,
}

impl<T, U> PaddedIter<T, U>
where
    T: Iterator<Item = U>,
    U: Clone,
{
    pub fn next(&mut self) -> Option<U> {
        self.n += 1;

        if self.n > self.length {
            return None;
        }

        let next_item = self.iter.next();
        match next_item {
            Some(item) => Some(item),
            None => Some(self.pad.clone()),
        }
    }
}

impl<T, U> From<(T, usize, U)> for PaddedIter<T, U>
where
    T: Iterator<Item = U>,
    U: Clone,
{
    /// Create a new PaddedIter from an iterator, length, and pad value.
    ///
    /// # Examples:
    ///
    /// ```
    /// use padded_iterator::PaddedIter;
    ///
    /// let mut iter = PaddedIter::from((vec![1, 2, 3].into_iter(), 5, 0));
    ///
    /// assert_eq!(iter.next(), Some(1));
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), Some(3));
    /// assert_eq!(iter.next(), Some(0));
    /// assert_eq!(iter.next(), Some(0));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn from((iter, length, pad): (T, usize, U)) -> Self {
        Self {
            iter,
            n: 0,
            length,
            pad,
        }
    }
}

/// A builder for PaddedIter.
pub struct PaddedIterBuilder<T: Iterator<Item = U>, U> {
    iter: Option<T>,
    length: Option<usize>,
    pad: Option<U>,
}

impl<T, U> PaddedIterBuilder<T, U>
where
    T: Iterator<Item = U>,
    U: Clone,
{
    pub fn new() -> Self {
        PaddedIterBuilder {
            iter: None,
            length: None,
            pad: None,
        }
    }

    /// Sets the base iterator for the PaddedIter.
    pub fn iter(mut self, iter: T) -> Self {
        self.iter = Some(iter);
        self
    }

    /// Sets the length to pad the iterator to.
    pub fn length(mut self, length: usize) -> Self {
        self.length = Some(length);
        self
    }

    /// Sets the value to pad the iterator with.
    pub fn pad(mut self, pad: U) -> Self {
        self.pad = Some(pad);
        self
    }

    /// Consumes the builder and returns a PaddedIter.
    pub fn build(self) -> Result<PaddedIter<T, U>, BuildError> {
        let iter = self.iter.ok_or(BuildError::IterNotSet)?;
        let length = self.length.ok_or(BuildError::LengthNotSet)?;
        let pad = self.pad.ok_or(BuildError::PadNotSet)?;

        Ok(PaddedIter {
            iter,
            n: 0,
            length,
            pad,
        })
    }
}
