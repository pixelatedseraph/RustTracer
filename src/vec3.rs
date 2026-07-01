
use std::fmt::write;
use std::ops::*;
use std::fmt;

#[derive(Default,Debug,Clone,Copy)]
pub struct Vec3{
   pub x : f64,
   pub y : f64,
   pub z : f64,
}

impl Vec3{
    pub fn new(x: f64, y : f64 ,z : f64) -> Self{
        Self { x, y, z }
    }
}

impl Add for Vec3{
    type Output = Self;

    fn add(self,rhs : Self) -> Self::Output{
        Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}


impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3{
    type Output = Self;

    fn sub(self,rhs : Self) -> Self::Output{
        Self{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for Vec3{
    type Output = Self;

    fn neg(self) -> Self::Output {
       Self{ 
            x : -self.x,
            y : -self.y,
            z : -self.z,
       }
    }
}

impl Mul<Vec3> for Vec3{
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self{
            x : self.x * rhs.x,
            y : self.y * rhs.y,
            z : self.z * rhs.z,
        }
    }
}


impl Mul<f64> for Vec3{
    type Output = Self;

    fn mul(self,rhs : f64) -> Self::Output{
        Self{
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f64> for Vec3{
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<Vec3> for Vec3{
    type Output = Self;

    fn div(self, rhs: Vec3) -> Self::Output {
        Self{
            x : self.x / rhs.x,
            y : self.y / rhs.y,
            z : self.z / rhs.z,
        }
    }
}

impl Div<f64> for Vec3{
    type Output = Self;

    fn div(self,rhs : f64) -> Self::Output{
        Self{
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vec3{
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl fmt::Display for Vec3{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}",self.x,self.y,self.z)
    }
}


impl Vec3{
    pub fn length(self) -> f64{
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64{
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(self,rhs : Self) -> f64{
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self,rhs : Self) -> Vec3{
        Vec3::new(self.y * rhs.z  - self.z * rhs.y,
            self.z * rhs.x  - self.x * rhs.z,
            self.x * rhs.y  - self.y * rhs.x 
        )
    }

    pub fn unit_vector(self) -> Self{
        self / self.length()
    }
}

