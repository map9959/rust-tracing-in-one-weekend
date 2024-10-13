use std::{ops, fmt::Display, f64::consts::PI};
use rand::Rng;
use rand_distr::{Normal, Distribution};

use crate::utils::Interval;

#[derive(Copy, Clone)]
pub struct Vec3{
    pub x: f64,
    pub y: f64,
    pub z: f64
}
impl Vec3{
    pub fn new(x: f64, y: f64, z: f64) -> Vec3{
        return Vec3{x, y, z};
    }
    pub fn length(&self) -> f64{
        return f64::sqrt(self.length_squared());
    }
    pub fn length_squared(&self) -> f64{
        return self.x*self.x + self.y*self.y + self.z*self.z;
    }
    ///Returns true if the vector is near zero.
    pub fn near_zero(&self) -> bool{
        let s = 1e-8;
        return f64::abs(self.x) < s && f64::abs(self.y) < s && f64::abs(self.z) < s;
    }
}

impl ops::Add<Vec3> for Vec3{
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output{
        return Vec3::new(self.x+rhs.x, self.y+rhs.y, self.z+rhs.z);
    }
}
impl ops::Sub<Vec3> for Vec3{
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output{
        return Vec3::new(self.x-rhs.x, self.y-rhs.y, self.z-rhs.z);
    }
}
impl ops::Mul<f64> for Vec3{
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        return Vec3::new(self.x*rhs, self.y*rhs, self.z*rhs);
    }
}
impl ops::Mul<Vec3> for Vec3{
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        return Vec3::new(self.x*rhs.x, self.y*rhs.y, self.z*rhs.z);
    }
}
///Extra multiplication for referenced vectors.
impl ops::Mul<f64> for &Vec3{
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        return Vec3::new(self.x*rhs, self.y*rhs, self.z*rhs);
    }
}
impl ops::Div<f64> for Vec3{
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        return Vec3::new(self.x/rhs, self.y/rhs, self.z/rhs);
    }
}
impl ops::Neg for Vec3{
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        return Vec3::new(-self.x, -self.y, -self.z)
    }
}
impl Display for Vec3{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{} {} {}", self.x, self.y, self.z);
    }
}

///Takes the dot product of two vectors.
pub fn dot(lhs: &Vec3, rhs: &Vec3) -> f64{
    lhs.x*rhs.x + lhs.y*rhs.y + lhs.z*rhs.z
}
///Takes the cross product of two vectors.
pub fn cross(lhs: &Vec3, rhs: &Vec3) -> Vec3{
    Vec3::new(
        lhs.y*rhs.z-lhs.z*rhs.y,
        lhs.z*rhs.x-lhs.x*rhs.z,
        lhs.x*rhs.y-lhs.y*rhs.x)
}

///Returns a random vector with values between 0 and 1.
pub fn random_vec() -> Vec3{
    let mut rng = rand::thread_rng();
    Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
}
///Returns a random vector with values between min and max.
pub fn random_vec_range(min: f64, max: f64) -> Vec3{
    let mut rng = rand::thread_rng();
    Vec3::new(rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max))
}
///Returns a normalized vector from vec.
pub fn unit_vector(vec: &Vec3) -> Vec3{
    Vec3::new(vec.x, vec.y, vec.z)/(vec.length())
}
///Returns a random unit vector by generating and normalizing a 3d vector from a Gaussian distribution.
pub fn random_unit_vector() -> Vec3{
    let mut rng = rand::thread_rng();
    let normal = Normal::new(0.0, 1.0).unwrap();
    let v = Vec3::new(
        normal.sample(&mut rng),
        normal.sample(&mut rng),
        normal.sample(&mut rng)
    );
    return unit_vector(&v);
}
///Returns a random unit vector on the hemisphere defined by the normal n.
pub fn random_on_hemisphere(normal: &Vec3) -> Vec3{
    let v = random_unit_vector();
    if dot(normal, &v) > 0.0{
        return v;
    }else{
        return -v;
    }
}

///Reflects a vector v off the surface defined by the normal n.
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3{
    return *v-n*dot(v, n)*2.0;
}