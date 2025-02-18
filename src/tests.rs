use super::geotiff::GeoTiff;

#[test]
fn read_size() {
    let geotiff = GeoTiff::from_file("example.tif").unwrap();
    assert_eq!(geotiff.get_size(), (120, 120));
}

#[test]
fn read_data() {
    let geotiff = GeoTiff::from_file("example.tif").unwrap();
    assert_eq!(geotiff.get_data().len(), 120 * 120);
}

#[test]
fn read_pixel() {
    let geotiff = GeoTiff::from_file("example.tif").unwrap();
    assert_eq!(geotiff.get_pixel(0, 0), 3335);
    assert_eq!(geotiff.get_pixel(119, 119), 3945);
}
