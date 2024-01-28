use std::path::Path;

use gdal::raster::RasterBand;
use gdal::Dataset;

use crate::image::Image;

pub fn read_image_raster(file_path: &Path, index: Option<isize>) -> Image {
    // verify the file exists
    if !file_path.exists() {
        panic!("File does not exist: {}", file_path.to_str().unwrap());
    }

    // open dataset
    let dataset = Dataset::open(file_path).unwrap();

    // read the band
    let rasterband: RasterBand = dataset.rasterband(index.unwrap_or(1)).unwrap();

    // TODO: Make this work with for other raster data types too
    // TODO: Make this work with multiple bands
    // TODO: Make this work with window

    match rasterband.read_as::<u8>((0, 0), rasterband.size(), rasterband.size(), None) {
        Ok(data) => Image::from_shape_vec(
            [
                rasterband.x_size() as usize,
                rasterband.y_size() as usize,
                1 as usize,
            ],
            data.data,
        ),
        Err(e) => panic!("Error reading band: {}", e),
    }
}
