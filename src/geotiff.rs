use super::{parser::Ifd, TiffFile, TiffParserError};
use std::path::Path;

/// Provides an abstraction to read GeoTiff files.
#[derive(Debug)]
pub struct GeoTiff {
    tiff: TiffFile,
}

impl GeoTiff {
    /// Creates a new GeoTiff from a file.
    pub fn from_file<P: AsRef<Path>>(name: P) -> Result<Self, TiffParserError> {
        let tiff = TiffFile::from_file(name)?;
        Ok(Self { tiff })
    }

    /// Returns the data of the GeoTiff by its index.
    pub fn get_ifd(&self, index: usize) -> Option<&Ifd> {
        self.tiff.ifds.get(index)
    }

    /// Returns the size of the GeoTiff, using tags `ImageWidth` (`256`) and `ImageLength` (`157`)
    /// for the first IFD.
    pub fn get_size(&self) -> (usize, usize) {
        let ifd = &self.tiff.ifds[0];
        (
            ifd.image_width().unwrap() as usize,
            ifd.image_length().unwrap() as usize,
        )
    }

    /// Returns the data of the GeoTiff.
    pub fn get_data(&self) -> Vec<i32> {
        let ifd = &self.tiff.ifds[0];
        ifd.data.clone()
    }

    /// Returns the data of the GeoTiff at the location specified in the arguments.
    pub fn get_pixel(&self, lon: usize, lat: usize) -> i32 {
        let ifd = &self.tiff.ifds[0];
        let width = ifd.image_width().unwrap() as usize;
        let length = ifd.image_length().unwrap() as usize;
        ifd.data[(length - 1 - lat) * width + lon]
    }
}
