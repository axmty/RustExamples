use std::fmt;

struct City {
    name: &'static str,
    lat: f32,
    lon: f32
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        write!(f, "{}: {:.3}°{} {:.3}°{}",
            self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

fn main() {
    let foo = 63565873;
    println!("{}", foo);
    println!("0x{:X}", foo);
    println!("0o{:o}", foo);
}