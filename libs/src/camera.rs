#[allow(non_snake_case)]
pub mod cam {
    use vec3::vector3::*;
    use utils::util::deg2rad;
    use ray::ray::Ray;
    use utils::util::Rd_Direction;
    #[derive(Copy, Clone, Default)]
    pub struct Camera {
        pub a_ratio: f64,
        pub _focal_len: f64, 
        pub fov: i32,
        pub _u_left_corner: Vec3,
        pub origin: Vec3,
        pub h: Vec3,
        pub v: Vec3,
        pub _blur_rate: f64
    }
    //ugly asf
    pub fn default_val()->Camera{
        Camera{
            a_ratio: 1.7777777777777777, 
            _focal_len: 5.0,
            fov: 55,
            origin: Vec3ZERO(),
            _blur_rate: 0.02,
            ..Default::default()
        }
    }
    fn calc(focal_len:f64, fov:i32, origin:Vec3, ratio:f64)->(Vec3, Vec3, Vec3){
        let viewport_H = focal_len * (2.0  * deg2rad((fov >> 1).into())).tan();
        let viewport_W = viewport_H*ratio;

        let horizontal = Vec3(viewport_W, 0.0, 0.0);
        let vertical   = Vec3(0.0, viewport_H, 0.0);

        let u_left_corner = origin - horizontal*0.5 - vertical*0.5 - Vec3(0.0, 0.0, focal_len);

        (u_left_corner, horizontal, vertical)
    }
    impl Camera{
        pub fn get_ray(&self, tx: f64, ty:f64)->Ray{
            let mut _d = true;
            let rd = Rd_Direction()*self._blur_rate;
            let k = self._u_left_corner + self.h*tx + self.v*ty - self.origin;
            if self._blur_rate != 0.0 {
                //blurring objects through randomizing their bounce rays
                //with a small amount of blur rate, this can act as an antialiasing factor    
                _d = true;
            }
            else {
                _d = false;
            }
            Ray{
                    origin: self.origin, direction: (if _d {(rd+k).normalize()} else {k.normalize()})
            }
        }
        pub fn init(&self,s: &str)->Self{
            let get_default_val = default_val();
            let mut calc_;
            let mut FOV = get_default_val.fov;
            let mut ratio:f64 = get_default_val.a_ratio; 
            let mut focal_len = get_default_val._focal_len;
            let mut origin = get_default_val.origin;
            let mut blur_rate = get_default_val._blur_rate;
           
            calc_ = calc(focal_len, FOV, origin, ratio);
            
            if s != "Default"  {
                FOV = self.fov;
                focal_len = self._focal_len;
                origin = self.origin;
                ratio = self.a_ratio;
                blur_rate =self._blur_rate;
                calc_ = calc(focal_len, FOV, origin, ratio);
            }

            Self{
                a_ratio: ratio, 
                _focal_len: focal_len,
                fov: FOV,
                origin: origin,
                h: calc_.1, v: calc_.2,
                _u_left_corner:calc_.0, 
                _blur_rate: blur_rate
            }
        }
         
    }
}
