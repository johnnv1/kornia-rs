use kornia_rs::io::geo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read a tiff image
    let image_path = std::path::Path::new("tests/data/T31UDQ_20240105T105339_TCI_10m.tif");
    let image = geo::read_image_raster(image_path, Some(1isize));

    println!("Image size: {:?}", image.image_size());

    // read a jp2 image
    let image_path = std::path::Path::new("tests/data/T31UDQ_20240105T105339_TCI_10m.jp2");
    let image = geo::read_image_raster(image_path, Some(1isize));

    println!("Image size: {:?}", image.image_size());
    Ok(())
}
