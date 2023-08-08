pub mod util{
    extern crate rand;
    extern crate rand_distr;
    extern crate min_max;
    use self::min_max::*;
    use self::rand::Rng;  
    use std::f64::consts::PI;
    use vec3::vector3::Vec3;
    use self::rand_distr::{Normal, Distribution, StandardNormal};
    pub fn deg2rad(d:f64)->f64{
        (d*PI) / 180.0
    }
    pub fn op_power(mut a:f64, mut b:i32)->f64{
        let mut final_ = 1.0;
        while b > 0 {
            if b&1 == 1 {final_ *= a};
            a*=a; b>>=1;
        }
        return final_;
    }
    pub fn Rd_Value()->f64{
        let mut RNG = rand::thread_rng();
        RNG.gen_range(0.0..=1.0)

    }

    pub fn Rd_Value_NDIST()->f64{
        let mut rand = rand::thread_rng();
        let normal =Normal::new(0.0, 1.0).unwrap();
        normal.sample(&mut rand)
    }

    pub fn Rd_Direction()->Vec3{
        Vec3(Rd_Value_NDIST(),
        Rd_Value_NDIST(),
        Rd_Value_NDIST()).normalize()
    }
    pub fn Reflective_Direction(dir_vec: Vec3, normal_vec: Vec3)->Vec3{
        dir_vec - normal_vec * 2.0  * dir_vec.dot(normal_vec)
    }
    pub fn Refractive_Direction(dir_vec: Vec3, normal_vec: Vec3, rf_idx:f64)->Vec3{
        let mut cosTheta = (-dir_vec.dot(normal_vec)).min(1.0);
        let R_perpendicular = (dir_vec + normal_vec*cosTheta)*rf_idx;
        let R_parallel = normal_vec*(-(1.0 - R_perpendicular.sq_len()).abs().sqrt());
        (R_perpendicular + R_parallel)
    }
    pub fn Reflectance(cosine: f64, ref_idx:f64)->f64{
        //polynomial approximation by Schlick
   
        let mut R0 = op_power((1.0 - ref_idx) / (1.0 +  ref_idx),2);
        //R0 =R0*R0;
        R0 + (1.0 - R0)*op_power((1.0 - cosine),5)
    }
    pub fn Clamp(color:f64, min:f64, max:f64)->f64{
        if color < min { return min}
        if color > max { return max}
        color
    }
    pub fn Lerp(a: Vec3, b: Vec3, k: f64)->Vec3{
        // linear interpolation
        a * (1.0 - k) + b * k
    }
}