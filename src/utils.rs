pub const infinity: f64 = f64::MAX;
pub const pi: f64 = std::f64::consts::PI;
pub fn degrees_to_radians(degrees: f64) -> f64{
    degrees/180.0*pi
}
pub struct Interval{
    pub min: f64,
    pub max: f64
}
impl Interval{
    pub fn new(min: f64, max: f64) -> Interval{
        Interval{min, max}
    }
    pub fn contains(&self, x: f64) -> bool{
        self.min <= x && x <= self.max
    }
    pub fn surrounds(&self, x: f64) -> bool{
        self.min < x && x < self.max
    }
    pub fn size(&self) -> f64{
        self.max - self.min
    }
}
pub const empty: Interval = Interval{min: f64::MAX, max: f64::MIN};
pub const universe: Interval = Interval{min: f64::MIN, max: f64::MAX};
pub const non_neg: Interval = Interval{min: 0.0, max: f64::MAX};