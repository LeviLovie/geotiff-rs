//! This is a crate to easily parse GeoTiff files written in pure Rust.
//! Documentation fore the [GeoTiff](https://en.wikipedia.org/wiki/GeoTIFF) format can be found
//! [here](https://www.earthdata.nasa.gov/s3fs-public/imported/19-008r4.pdf).
//!
//! # Example
//! ```
//! fn main() {
//!     let geotiff = geotiff_rs::GeoTiff::from_file("example.tif").unwrap();
//!     println!("Size: {:?}", geotiff.get_size());
//!     println!("Data: {:?}", geotiff.get_data());
//!     println!("Pixel at (0, 0): {:?}", geotiff.get_pixel(0, 0));
//! }
//! ```

#[cfg(test)]
mod tests;

mod geotiff;
mod parser;

pub use geotiff::GeoTiff;
pub use parser::{TiffFile, TiffParserError};
