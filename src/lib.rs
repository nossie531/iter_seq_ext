//! Iterator extension for sequential items.
//!
//! _The author of this crate is not good at English._  
//! _Forgive me if the document is hard to read._

#![warn(missing_docs)]
#![cfg_attr(not(test), no_std)]

pub mod prelude;
pub use iterator_seq_ext::*;
pub use replace::*;

mod iterator_seq_ext;
mod msg;
mod replace;
