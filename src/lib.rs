pub mod generals;

#[cfg(feature = "optimize")]
pub mod optimize;

#[cfg(feature = "solve")]
pub mod solve;

#[cfg(feature = "stats")]
pub mod stats;

//#[cfg(feature = "geometry")]
pub mod geometry;

const IMAGINARY_NUMBER_LETTER: &str = "i";
