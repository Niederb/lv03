#![no_std]

pub struct Wgs84 {
    longitude: f64,
    latitude: f64,
    altitude: f64,
}

impl Wgs84 {
    pub fn to_lv03(&self) -> Lv03 {
        let phi = (3600.0 * self.latitude - 169_028.66) / 10_000.0;
        let phi_2 = phi * phi;
        let phi_3 = phi * phi_2;
        let lambda = (3600.0 * self.longitude - 26_782.5) / 10_000.0;
        let lambda_2 = lambda * lambda;
        let lambda_3 = lambda * lambda_2;

        let e = 2_600_072.37 + 211_455.93 * lambda
            - 10938.51 * lambda * phi
            - 0.36 * lambda * phi_2
            - 44.54 * lambda_3;
        let n = 1_200_147.07 + 308_807.95 * phi + 3745.25 * lambda_2 + 76.63 * phi_2
            - 194.56 * lambda_2 * phi
            + 119.79 * phi_3;
        let y = e - 2_000_000.00;
        let x = n - 1_000_000.00;
        let altitude = self.altitude - 49.55 + 2.73 * lambda + 6.94 * phi;
        Lv03 {
            north: x,
            east: y,
            altitude,
        }
    }
}

/// Coordinate point in the Lv03 system
pub struct Lv03 {
    /// Coordinate pointing north. (X coordinate)
    north: f64,
    /// Coordinate pointing east. (Y coordinate)
    east: f64,
    altitude: f64,
}

impl Lv03 {
    /// Can return none if the given coordinates do not lead to a valid representation in the swiss coordinate system
    pub fn new(north: f64, east: f64, altitude: f64) -> Option<Self> {
        if north < 0.0 || east < 0.0 {
            // North and east coordinates must be positive
            None
        } else if north > east {
            // East coordinate must always be bigger than north
            None
        } else {
            Some(Lv03 {
                north,
                east,
                altitude,
            })
        }
    }

    pub fn to_wgs84(&self) -> Wgs84 {
        let y = (self.east - 600_000.0) / 1_000_000.0;
        let y_2 = y * y;
        let y_3 = y * y_2;
        let x = (self.north - 200_000.0) / 1_000_000.0;
        let x_2 = x * x;
        let x_3 = x * x_2;
        let lambda = 2.6779094 + 4.728982 * y + 0.791484 * y * x + 0.1306 * y * x_2 - 0.0436 * y_3;
        let phi = 16.9023892 + 3.238272 * x
            - 0.270978 * y_2
            - 0.002528 * x_2
            - 0.0447 * y_2 * x
            - 0.0140 * x_3;
        let altitude = self.altitude + 49.55 - 12.6 * y - 22.64 * x;

        let lambda = lambda * 100.0 / 36.0;
        let phi = phi * 100.0 / 36.0;
        Wgs84 {
            longitude: lambda,
            latitude: phi,
            altitude,
        }
    }

    pub fn distance_squared(&self, p: &Lv03) -> f64 {
        let d_north = self.north - p.north;
        let d_east = self.east - p.east;
        let d_altitude = self.altitude - p.altitude;
        d_north * d_north + d_east * d_east + d_altitude * d_altitude
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_conversions(ch: &Lv03, wgs: &Wgs84) {
        let wgs_converted = ch.to_wgs84();
        assert!((wgs_converted.longitude - wgs.longitude).abs() < 0.001);
        assert!((wgs_converted.latitude - wgs.latitude).abs() < 0.001);

        let ch_converted = wgs.to_lv03();
        assert!((ch_converted.east - ch.east).abs() < 2.0);
        assert!((ch_converted.north - ch.north).abs() < 2.0);
    }

    #[test]
    fn to_lv03_bundeshaus() {
        let ch = Lv03 {
            east: 600_421.43,
            north: 199_498.43,
            altitude: 542.8,
        };
        let wgs = Wgs84 {
            longitude: 7.44417,
            latitude: 46.94658,
            altitude: 542.8,
        };
        test_conversions(&ch, &wgs);
    }

    #[test]
    fn to_wgs84_700_100() {
        let ch = Lv03 {
            east: 700_000.0,
            north: 100_000.0,
            altitude: 542.8,
        };
        let wgs = Wgs84 {
            longitude: 8.730497076,
            latitude: 46.044130339,
            altitude: 542.8,
        };
        test_conversions(&ch, &wgs);
    }

    #[test]
    fn test_negative() {
        assert!(Lv03::new(-1.0, 2.0, 5.0).is_none());
        assert!(Lv03::new(1.0, -2.0, 5.0).is_none());
        assert!(Lv03::new(1.0, 2.0, -5.0).is_some());
    }

    #[test]
    fn test_coordinates_swapped() {
        assert!(Lv03::new(600_000.0, 200_000.0, 500.0).is_none());
    }

    #[test]
    fn test_distance() {
        let p1 = Lv03::new(200_000.0, 600_000.0, 500.0).unwrap();
        let p2 = Lv03::new(200_002.0, 600_000.0, 500.0).unwrap();
        assert_eq!(4.0, p1.distance_squared(&p2));
    }
}
