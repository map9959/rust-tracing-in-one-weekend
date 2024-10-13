pub const INFINITY: f64 = f64::MAX;
pub const PI: f64 = std::f64::consts::PI;
pub fn degrees_to_radians(degrees: f64) -> f64{
    degrees/180.0*PI
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
    pub fn clamp(&self, x: f64) -> f64{
        if x < self.min {return self.min};
        if x > self.max {return self.max};
        x
    }
}
pub const EMPTY: Interval = Interval{min: f64::MAX, max: f64::MIN};
pub const UNIVERSE: Interval = Interval{min: f64::MIN, max: f64::MAX};
pub const NON_NEG: Interval = Interval{min: 0.0, max: f64::MAX};