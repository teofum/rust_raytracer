pub struct Interval(pub f64, pub f64);

impl Interval {
    pub const EMPTY: Self = Interval(f64::INFINITY, f64::NEG_INFINITY);
    pub const UNIVERSE: Self = Interval(f64::NEG_INFINITY, f64::INFINITY);

    pub fn min(&self) -> f64 {
        self.0
    }

    pub fn max(&self) -> f64 {
        self.1
    }

    pub fn contains(&self, x: f64) -> bool {
        self.0 <= x && x <= self.1
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.0 < x && x < self.1
    }
}
