use std::path::Path;

use gdal::raster::RasterBand;
use gdal::Dataset;

use crate::image::{Image, ImageSize};

pub fn read_raster(file_path: &Path, index: Option<isize>) -> Image {
    // verify the file exists
    if !file_path.exists() {
        panic!("File does not exist: {}", file_path.to_str().unwrap());
    }

    // open dataset
    let dataset = Dataset::open(file_path).unwrap();

    // read the band
    let rasterband: RasterBand = dataset.rasterband(index.unwrap_or(1)).unwrap();

    // TODO: Make this work with for other raster data types too
    let rv = rasterband.read_as::<u8>(
        (0, 0),
        (rasterband.x_size(), rasterband.y_size()),
        (rasterband.x_size(), rasterband.y_size()),
        None,
    );

    Image::new(
        ImageSize {
            width: rasterband.x_size() as usize,
            height: rasterband.y_size() as usize,
        },
        rv.unwrap().data.to_vec(),
    )
}
