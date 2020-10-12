use lv03::Lv03;

use bmp::{Image, Pixel};

/** Example a creates an image showing how the error is
* distributed accross Switzerland when converting to WGS84 and back.
*/
fn main() {
    let mut img = Image::new(840 - 490, 290 - 80);

    for (x, y) in img.coordinates() {
        let east = (1000.0 * x as f64) + 490_000.0;
        let north = (1000.0 * y as f64) + 80_000.0;
        let lv03 = Lv03::new(north, east, 1000.0).unwrap();
        let wgs84 = lv03.to_wgs84();
        let lv03_converted = wgs84.to_lv03().unwrap();
        let error = lv03.distance_squared(&lv03_converted);
        img.set_pixel(x, y, Pixel::new((100.0 * error) as u8, 0, 0));
    }

    img.save("Output.bmp").unwrap();
}
