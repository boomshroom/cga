//! Contains definitions for standard Euclidean geometric objects, namely vectors, bivectors, etc.

mod bivector;
mod trivector;
mod vector;

pub use bivector::Bivector;
pub use trivector::Trivector;
pub use vector::Vector;
