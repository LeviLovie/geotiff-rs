fn main() {
    let geotiff = geotiff_rs::GeoTiff::from_file("example.tif").unwrap();
    let (width, height) = geotiff.get_size();
    for x in 0..width {
        for y in 0..height {
            let value = geotiff.get_pixel(x, y);
            println!("Value at {} {} is {}", x, y, value);
        }
    }
}
