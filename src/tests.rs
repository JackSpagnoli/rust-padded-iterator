use std::vec::IntoIter;

use crate::{utils::BuildError, PaddedIterBuilder};

#[test]
fn test_padded_iter() {
    let mut iter = PaddedIterBuilder::new()
        .iter(vec![1, 2, 3].into_iter())
        .length(5)
        .pad(0)
        .build()
        .unwrap();

    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_padded_iter_length_0() {
    let mut iter = PaddedIterBuilder::new()
        .iter(vec![1, 2, 3].into_iter())
        .length(0)
        .pad(0)
        .build()
        .unwrap();

    assert_eq!(iter.next(), None);
}

#[test]
fn test_padded_iter_length_1() {
    let mut iter = PaddedIterBuilder::new()
        .iter(vec![1, 2, 3].into_iter())
        .length(1)
        .pad(0)
        .build()
        .unwrap();

    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_padded_iter_mod_iter_after_creation() {
    let mut builder = PaddedIterBuilder::new()
        .iter(vec![1, 2, 3].into_iter())
        .length(5)
        .pad(0);

    builder = builder.iter(vec![4, 5, 6].into_iter());

    let mut iter = builder.build().unwrap();

    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), Some(5));
    assert_eq!(iter.next(), Some(6));
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_padded_iter_mod_length_after_creation() {
    let mut builder = PaddedIterBuilder::new()
        .iter(vec![1, 2, 3].into_iter())
        .length(5)
        .pad(0);
    builder = builder.length(2);

    let mut iter = builder.build().unwrap();

    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_padded_iter_mod_pad_after_creation() {
    let mut builder = PaddedIterBuilder::new()
        .iter(vec![1, 2, 3].into_iter())
        .length(5)
        .pad(0);
    builder = builder.pad(1);

    let mut iter = builder.build().unwrap();

    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_padded_iter_no_length() {
    let builder = PaddedIterBuilder::new()
        .iter(vec![1, 2, 3].into_iter())
        .pad(0);

    assert!(matches!(builder.build(), Err(BuildError::LengthNotSet)));
}

#[test]
fn test_padded_iter_no_pad() {
    let builder = PaddedIterBuilder::new()
        .iter(vec![1, 2, 3].into_iter())
        .length(5);

    assert!(matches!(builder.build(), Err(BuildError::PadNotSet)));
}

#[test]
fn test_padded_iter_no_iter() {
    let builder: PaddedIterBuilder<IntoIter<i32>, i32> = PaddedIterBuilder::new().length(5).pad(0);

    assert!(matches!(builder.build(), Err(BuildError::IterNotSet)));
}
