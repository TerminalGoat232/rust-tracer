pub mod ray{
    use vec3::vector3::*;
    use obj::objects::*;
    use utils::util::Rd_Direction;
    #[derive(Copy, Clone)]
    pub struct Ray{
        pub origin:Vec3,
        pub direction:Vec3,
    }
    pub struct HitIn4{
        pub did_hit: bool,
        pub distance: f64,
        pub hit_point:Vec3,
        pub normal:Vec3,
        pub material: Material,
    }
    impl Ray {
        pub fn Ray_cast_to(&mut self, sphere: Sphere) -> HitIn4{
              let mut hit_info  = HitIn4 {
                  did_hit: true, 
                  distance: (1<<6) as f64, 
                  hit_point: Vec3(0.0,0.0,0.0),
                  normal: Vec3(0.0,0.0,0.0),
                  material: Material{color: Vec3(0.0, 0.0, 0.0), ..Default::default()}
                };
              //let rd = Rd_Direction() * 0.0;
              let offset_ray_orig: Vec3 = (self.origin - sphere.centre);
              // Sphere formula 
              // ()|ray_org + ray_dir + distance|)^2 = radius^2;
              let a:f64 = self.direction.dot(self.direction);
              let b:f64 = offset_ray_orig.dot(self.direction);
              let c:f64 = offset_ray_orig.dot(offset_ray_orig) - sphere.radius * sphere.radius;
              let delta: f64 = b*b - a*c;
              if delta >= 0.0 {
                  let  distance: f64  = (-b - delta.sqrt()) / a;
                  if distance >= 0.0 {
                     hit_info.did_hit = true;
                     hit_info.distance = distance;
                     hit_info.hit_point = self.origin + self.direction * distance;
                     hit_info.normal = (hit_info.hit_point - sphere.centre ).normalize();
                  }    
                }
                else { hit_info.did_hit = false}
            hit_info
        }

      
    }

}
