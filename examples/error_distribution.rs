use lv03::Lv03;

use bmp::{Image, Pixel};

/** Example a creates an image showing how the error is
* distributed accross Switzerland when converting to WGS84 and back.
*/
fn main() {
    let mut img = Image::new(850 - 480, 300 - 70);

    for (x, y) in img.coordinates() {
        let east = (1000.0 * x as f64) + 480_000.0;
        let north = (1000.0 * y as f64) + 70_000.0;
        let lv03 = Lv03::new(north, east, 1000.0);
        if let Some(lv03) = lv03 {
            let wgs84 = lv03.to_wgs84();
            if let Some(lv03_converted) = wgs84.to_lv03() {
                //let error = lv03.distance_squared(&lv03_converted);
                let error = ((lv03.north - lv03_converted.north).powi(2) + (lv03.east - lv03_converted.east).powi(2)).sqrt();
                img.set_pixel(x, y, Pixel::new((100.0 * error) as u8, 0, 0));
            }
        }
    }

    img.save("Output.bmp").unwrap();
}
