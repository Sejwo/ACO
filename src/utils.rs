use std::f64::consts::PI;

pub fn usize_float_multiplication(value: usize, multiplier: f64) -> usize {
    ((value as f64) * multiplier).round() as usize
}

pub fn degrees_to_radians(deg: f64) -> f64 {
    deg * (PI / 180.0)
}
pub fn calculate_distances(lat1: f64, lat2: f64, long1: f64, long2: f64) -> f64 {
    // solution source https://stackoverflow.com/questions/27928/calculate-distance-between-two-latitude-longitude-points-haversine-formula
    let d_lat = degrees_to_radians(lat2 - lat1);
    let d_long = degrees_to_radians(long2 - long1);
    let r = 6371.0;
    let temp =
        (d_lat / 2.0).sin() * (d_lat / 2.0).sin() +
        degrees_to_radians(lat1).cos() *
            degrees_to_radians(lat2).cos() *
            (d_long / 2.0).sin() *
            (d_long / 2.0).sin();
    let c = 2.0 * temp.sqrt().atan2((1.0 - temp).sqrt());
    println!("{:?}", r * c);
    r * c
}
