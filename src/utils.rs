//! Utility functions for the crate
//! 
//! Contains an enum of errors that can occur when building a padded iterator.

use std::fmt::{Display, Formatter, Result};

/// An error that can occur when building a padded iterator.
#[derive(Debug, PartialEq)]
pub enum BuildError {

    /// The iterator was not set.
    IterNotSet,
    /// The length was not set.
    LengthNotSet,
    /// The pad was not set.
    PadNotSet,
}

impl Display for BuildError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            BuildError::IterNotSet => write!(f, "Iterator not set"),
            BuildError::LengthNotSet => write!(f, "Length not set"),
            BuildError::PadNotSet => write!(f, "Pad not set"),
        }
    }
}

impl std::error::Error for BuildError {}