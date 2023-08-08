pub mod cam {
    use vec3::vector3::*;
    use utils::util::deg2rad;
    use ray::ray::Ray;
    use utils::util::Rd_Direction;
    #[derive(Copy, Clone)]
    pub struct Camera {
        pub a_ratio: f64,
        pub _focal_len: f64, 
        pub fov: f64,
        pub _viewport_H: f64, 
        pub _viewport_W: f64,
        pub h: Vec3, 
        pub v: Vec3, 
        pub _u_left_corner: Vec3,
        pub origin: Vec3,
        pub _blur_rate: f64
    }
    //ugly asf
    pub fn get_camera_val()->Camera{
        let FOV = 55.0;
        let ratio:f64 = 1.7777777777777777; 
        let focal_len = 5.0;
        let origin = Vec3ZERO();
        let viewport_H = focal_len * (2.0  * deg2rad(((FOV as i32) >> 1) as f64)).tan();
        let viewport_W = viewport_H*ratio;

        let horizontal = Vec3(viewport_W, 0.0, 0.0);
        let vertical   = Vec3(0.0, viewport_H, 0.0);

        let blur_rate = 0.02;
        let u_left_corner = origin - horizontal*0.5 - vertical*0.5 - Vec3(0.0, 0.0, focal_len);

        return Camera{
            a_ratio: ratio, _focal_len: focal_len,
            fov: FOV,_viewport_H: viewport_H, _viewport_W: viewport_W,
            h: horizontal, v: vertical, origin: origin, _u_left_corner: u_left_corner, 
            _blur_rate: blur_rate
        }
    }
    impl Camera{
        pub fn get_ray(&self, tx: f64, ty:f64)->Ray{
            let mut d = Vec3ZERO();
            let k = (self._u_left_corner + self.h*tx + self.v*ty - self.origin);
            if self._blur_rate != 0.0 {
                //bluring objects by randomizing their bounce rays
                //with small amount of blur rate, this can be act as an antialiasing factor
                let mut rd = Rd_Direction()*self._blur_rate;
                d = (rd+k).normalize();
            }
            else {
                d = k.normalize();
            }
            Ray{
                    origin: self.origin, direction: (d)
                }
            }
         
        }
    
}