#[allow(non_snake_case)]
pub mod vector3{
    extern crate derivative;
    
    use std::fmt;
    use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign,Div, Neg};
    #[derive(Copy, Clone, Default)]
    pub struct Vec3{
        pub x :f64,
        pub y :f64,
        pub z :f64,
    }
    pub fn Vec3(_x:f64, _y:f64, _z:f64)->Vec3{
        Vec3{x:_x, y:_y, z:_z}
    }
    pub fn Vec3ZERO()->Vec3{
        Vec3{x:0.0, y:0.0, z:0.0}
    }
    impl Vec3{
        pub fn length(&self)->f64{
           (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
        }

        pub fn sq_len(&self)->f64{
            self.length()*self.length()           
        }

        pub fn dot(&self, b: Vec3)->f64{
            self.x*b.x + self.y * b.y + self.z * b.z
        }
        pub fn normalize(&self)->Vec3{
            let _len :f64 = self.length();
            Vec3{x:self.x/_len, y: self.y/_len, z:self.z/_len}
        }
      
    }
    impl Neg for Vec3{
        type Output = Self;
        fn neg(self)->Self{
            self * (-1.0) 
        }
    }
    impl Add for Vec3{
        type Output = Self;
        fn add(self, b: Self)->Self{
            Self{
                x:self.x + b.x,
                y:self.y + b.y,
                z:self.z + b.z,
            } 
        }
    }
    impl AddAssign for Vec3{
        fn add_assign(&mut self, b:Vec3){
            self.x += b.x;
            self.y += b.y;
            self.z += b.z;
        }
    }
    impl Sub for Vec3{
        type Output = Self;
        fn sub(self, b: Self)->Self{
            Self{
                x:self.x - b.x,
                y:self.y - b.y,
                z:self.z - b.z,
            } 
        }
    }
    impl SubAssign for Vec3{
        fn sub_assign(&mut self, b:Vec3){
            self.x -= b.x;
            self.y -= b.y;
            self.z -= b.z;
        }
    }

    impl Mul for Vec3{
        type Output = Self;
        fn mul(self, b: Self)->Self{
            Self{
                x:self.x * b.x,
                y:self.y * b.y,
                z:self.z * b.z,
            } 
        }
    }
    impl MulAssign for Vec3{
        fn mul_assign(&mut self, b:Vec3){
            self.x *= b.x;
            self.y *= b.y;
            self.z *= b.z;
        }
    }

    impl Mul<f64> for Vec3{
        type Output = Self;
        fn mul(self, k:f64)->Self{
            Self{
                x:k * self.x,
                y:k * self.y,
                z:k * self.z,
            } 
        }
    }
    impl Div<f64> for Vec3{
        type Output = Self;
        fn div(self, k:f64)->Self{
            self * (1.0/k)
        }
    }

    

    //--------DEBUG -------//
    impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}
}

