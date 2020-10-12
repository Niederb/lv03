# lv03
![Rust](https://github.com/Niederb/lv03/workflows/build/badge.svg)

This crate provides methods to convert Wgs84 coordinates into the Swiss coordinate formats LV03 (Landesvermessung 1903 or CH1903) and LV95 (Landesvermessung 1995 or CH1903+).

## Swiss coordinate system
See Wikipedia for more information:  
https://en.wikipedia.org/wiki/Swiss_coordinate_system


## Implementation
Based on the formulas described in the document "Näherungsformeln für die Transformation zwischen Schweizer Projektionskoordinaten und WGS84" by the "Bundesamt für Landestopografie swisstopo".
The formulas are only an estimation. Accuracy should be within 1 meter or 0.1'' respectively.

See this document for implementation details:  
https://www.swisstopo.admin.ch/de/swisstopo/dokumente.detail.document.html/swisstopo-internet/de/documents/geo-documents/ch1903wgs84_d.pdf.html

## Examples
### LV03 to WGS84
```
// Federal building
let lv03 = Lv03::new(199_498.43, 600_421.43, 542.8).unwrap();
let wgs84 = lv03.to_wgs84();
```

### WGS84 to LV03
```
// Matterhorn peak
let wgs = Wgs84 {
    longitude: 7.65861,
    latitude: 45.97642,
    altitude: 4532.9,
};
let lv03 = wgs.to_lv03().unwrap();
```

## Tips
For manual conversions Swisstopo provides an online tool:  
https://www.swisstopo.admin.ch/de/karten-daten-online/calculation-services/navref.html