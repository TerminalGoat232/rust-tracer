

#[allow(non_camel_case_types)]
extern crate sdl2_sys;
#[allow(unused_imports)]
use sdl2_sys::{SDL_GetTicks};

extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;


use sdl2::rect::{Point};
use sdl2::render::CanvasBuilder;

extern crate stuffs;
use stuffs::ray::ray::*;
use stuffs::vec3::vector3::*;
use stuffs::obj::objects::*;
use stuffs::utils::util::*;
use stuffs::camera::cam::*;

extern crate min_max;

static mut SPHERES_MANAGER: ObjManager = ObjManager{
    single_sphere: Vec::<Sphere>::new()
};
static mut MAX_BOUNCE_COUNT: u32 = 1550;
static mut RAY_PER_PIXEL: u32 = 5;

pub fn main() {
    // initialize camera's values
    let cam = Camera {
        fov:65,
        ..default_val()
    }
     // leave the arg blank if you prefer custom values
     //"Default" if you favor const default value that i recommended in camera.rs
    .init("");

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();  
    let ratio:f64 = cam.a_ratio;
    let width:usize = 1080;
    let height:usize = (width as f64 / ratio) as usize;  
    let window = video_subsystem
        .window(
           "rust tracer", 
            width.try_into().unwrap(), 
            height.try_into().unwrap()
        )
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = CanvasBuilder::new(window)
        .accelerated()
        .build()
        .unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    let mut frame_buffer = vec![vec![Vec3{x:0.0, y:0.0, z:0.0}; height]; width];
    //let frame_buffer = &mut FRAMES_CONTAINER;
    let mut _x = 0;
    let mut _y = 0;

    let mut frame = 0;
    //let mut draw_color = Vec3ZERO();
    
    fn ray_col(ray: Ray,  _x:i32,  _y:i32)-> Vec3{
       //if ray_collision(ray).did_hit{
        return ray_path_trace(ray);

    }

    fn ray_collision(mut ray: Ray)-> HitIn4{
        unsafe{
            let spheres_vec = &SPHERES_MANAGER;
            let mut closest = HitIn4 {
                did_hit: false, 
                distance: (1<<6) as f64, 
                hit_point: Vec3(0.0, 0.0, 0.0),
                normal: Vec3(0.0, 0.0, 0.0),
                material: Material{color: Vec3(0.0, 0.0, 0.0),..Default::default() } };
            for i in 0..spheres_vec.single_sphere.len(){
                let get_sphere = spheres_vec.single_sphere[i];
                let hitinfo: HitIn4 = ray.Ray_cast_to(get_sphere);
                if hitinfo.did_hit && hitinfo.distance < closest.distance{
                    closest = hitinfo;
                    closest.material = get_sphere.material;
                    
                }
            }
            closest
        }
    }

    fn ray_path_trace(mut ray: Ray)->Vec3{
        let mut incoming_light = Vec3(0.0, 0.0, 0.0);
        let mut ray_color = Vec3(1.0, 1.0, 1.0);
        unsafe {
            
            for _c in 1..=MAX_BOUNCE_COUNT{
                let hitinfo = ray_collision(ray);

                if hitinfo.did_hit{
                    ray.origin = hitinfo.hit_point;

                    let mut _dir = ray.direction;
                    let is_glossy = hitinfo.material.is_glossy;
                    let diffuse_direction = (hitinfo.normal + Rd_Direction()).normalize();
                    let specular_direction = Reflective_Direction(_dir, hitinfo.normal);
                   
                    let is_specular_bounce = if hitinfo.material.metallic > Rd_Value() {1.0} else {0.0};
                    //check if an object is not "transparency" as refractive ratio is zero so we can use reflective ray
                    if hitinfo.material.refractive_index == 0.0 { 
                        //blending between specular/diffuse material
                        if is_glossy == true {
                            ray.direction = Lerp(specular_direction, diffuse_direction, hitinfo.material.roughness * is_specular_bounce);    
                        }
                        ray.direction = Lerp(specular_direction, diffuse_direction, hitinfo.material.roughness);
                        
                    }
                    //if the object hates black peoples, use refractive ray instead
                    else {
                        let _refraction_direction = Vec3(0.0, 0.0, 0.0);
                       
                        let cos_theta = (-ray.direction).dot(hitinfo.normal).min(1.0);
                        let sin_theta =(1.0 - cos_theta*cos_theta).sqrt(); // sin^2 + cos^2 = 1
                       
                        let isnt_refractable = (hitinfo.material.refractive_index * sin_theta) > 1.0;

                        let r1_r2 = if isnt_refractable || (Reflectance(cos_theta, hitinfo.material.refractive_index) > Rd_Value()) {1.0} else {0.0};

                        ray.direction =Lerp(Refractive_Direction(_dir, hitinfo.normal, hitinfo.material.refractive_index), specular_direction, r1_r2 );
                    }
                    let emitted = hitinfo.material.emission_color * hitinfo.material.emission_strength;
                    incoming_light += emitted * ray_color * 2.0;
                    ray_color *=  Lerp(hitinfo.material.color,ray_color, is_specular_bounce);
                    
                }
                else {    let down_sky_color: Vec3 =  Vec3(0.65, 0.82, 1.0);
                          let up_sky_color: Vec3 = Vec3(1.0, 1.0, 1.0);

                          let unit_dir = ray.direction.normalize();
                          let k = unit_dir.y+0.8;

                          incoming_light += ray_color * Lerp(down_sky_color, up_sky_color, k);
                          break;
                     }
            }
        }
        //return
        incoming_light
      
    }
    #[allow(dead_code)]
    fn coc(a: u32)->f64{
        (core::f64::consts::PI*(a as f64)/180.0).cos()
    }
    #[allow(dead_code)]
    fn sic(a: u32)->f64{
        (core::f64::consts::PI*(a as f64)/180.0).sin()
    }
    
    
    //initialize objects 
    let  list_of_spheres = vec![
        //8.5, 6.0, 6.0
        //sun ray col: 0.89, 0.758, 0.441
        Sphere{centre: -Vec3(-1.0, 16.0, 12.0), radius: 8.5, material: Material{color:Vec3(0.0, 0.0, 0.0),
           emission_color:Vec3(0.89, 0.758, 0.441), emission_strength:6.7,roughness:1.0,..Default::default()}},
          // Sphere{centre: -Vec3(-0.0, 0.3, 1.6), radius: 0.5, material: Material{color: Vec3(1.0, 1.0, 1.0),refractive_index:0.5, ..Default::default()}},
        //Sphere{centre: -Vec3(-1.0, 0.3, 2.0), radius: 0.6, material: Material{color: Vec3(1.0, 1.0, 1.0),refractive_index:0.4,..Default::default()}},
        //Sphere{centre: -Vec3(-1.0, -0.5, 2.7), radius: 1.0, material: Material{color: Vec3(0.7, 0.7, 0.7),roughness:0.0,..Default::default()}},
        Sphere{centre: -Vec3(1.4, -0.8, 3.6), radius: 0.5, material: Material{color:Vec3(0.0, 0.0, 0.0),
            emission_color: Vec3(1.0, 1.0, 1.0),roughness:1.0, emission_strength:2.0,..Default::default()}},
        Sphere{centre: -Vec3(0.7, 1.5, 3.6), radius: 0.5, material: Material{color: Vec3(1.0, 1.0, 1.0),refractive_index:0.41374,is_glossy:false, ..Default::default()}},
        Sphere{centre: -Vec3(0.0, -0.4, 3.8), radius: 0.7, material: Material{color: Vec3(1.0, 1.0, 1.0),refractive_index:0.5,is_glossy:false, ..Default::default()}},
        Sphere{centre: -Vec3(0.2, -0.8, 2.8), radius: 0.2, material: Material{color: Vec3(1.0, 1.0, 1.0),refractive_index:0.41374,is_glossy:false, ..Default::default()}},
        Sphere{centre: -Vec3(-2.0, -0.2, 4.4), radius: 1.0, material: Material{color: Vec3(1.0, 1.0, 1.0),refractive_index:0.3,is_glossy:false, ..Default::default()}},
        Sphere{centre: -Vec3(1.0, 0.5, 5.9), radius: 1.0, material: Material{color: Vec3(0.0, 0.6, 0.2),metallic:0.15,is_glossy:true,roughness:0.2, ..Default::default()}},
        Sphere{centre: -Vec3(-0.6, -1.3, 2.9), radius: 0.4, material: Material{color: Vec3(0.0, 0.2, 0.6),metallic:0.05,is_glossy:true,roughness:0.4, ..Default::default()}},
        Sphere{centre: -Vec3(0.0, -8.5,7.0), radius: 8.0, material: Material{color: Vec3(0.3, 0.2, 0.2),roughness:1.0,..Default::default()}},
        Sphere{centre: -Vec3(3.5, 0.2, 5.7), radius: 1.3, material: Material{color: Vec3(1.0, 0.7, 0.6),roughness:0.2,..Default::default()}},
        Sphere{centre: -Vec3(-1.3, 0.4, 5.9), radius: 1.0, material: Material{color: Vec3(1.0, 0.0, 0.0),metallic:0.3, is_glossy:true,roughness:0.0,..Default::default()}},
        //Sphere{centre: -Vec3(-0.2, -0.5, 4.5), radius: 0.7, material: Material{color: Vec3(1.0, 1.0, 1.0), refractive_index:0.4124124, ..Default::default()}},
        Sphere{centre: -Vec3(3.0, -1.5, 3.7), radius: 0.5, material: Material{color: Vec3(0.3, 0.2, 0.2),roughness:0.1,..Default::default()}},
    ];
    // objects appending field:
    unsafe {for sp in &list_of_spheres {SPHERES_MANAGER.single_sphere.push(*sp);}}
    // the main loop
    'running: loop {

      unsafe {
        
        //SPHERES_MANAGER.single_sphere[9].centre.y=(coc(frame));
        // SPHERES_MANAGER.single_sphere[7].centre.y=0.4*(coc(frame*30));
        // SPHERES_MANAGER.single_sphere[5].centre.y=-1.3*(coc(frame*30));
        // SPHERES_MANAGER.single_sphere[0].centre.x=-7.0*(sic(frame*30));
        // SPHERES_MANAGER.single_sphere[0].centre.z=16.0*(coc(frame*30));
        // let jitterMatrix = vec![
        //     -0.25,  0.75,
        //      0.75,  0.33333,
        //     -0.75, -0.25,
        //      0.25, -0.75,
        // ];
        let start: u32 = SDL_GetTicks();
        let mut _time: u32 = 0;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                => break 'running,
                _ => {}
            }
        }
        let mut point: Point;

        for x in 0..width as i32  {
            for y in 0..height as i32 {
                let uz_x = x as usize; let uz_y = y as usize;
                let tx = (x as f64) / (width as f64 - 1.0);
                let ty = (y as f64) / (height as f64 - 1.0);
                let ray = cam.get_ray(tx, ty);
                
                let mut total_incoming_light = Vec3(0.0, 0.0, 0.0);
               
                for _ridx in 0..RAY_PER_PIXEL{total_incoming_light +=  ray_col(ray,x, y)}
        
                let mut draw_color =  total_incoming_light / RAY_PER_PIXEL as f64;
                //progressively rendering 
                let weight = 1.0 / (frame + 1) as f64;
                //blending the next frame with the previous frame
                draw_color = frame_buffer[(x) as usize][(y) as usize] * (1.0 - weight) + draw_color * weight;
                frame_buffer[uz_x][uz_y] = draw_color;
                //FRAMES_CONTAINER[uz_x][uz_y] = frame_buffer[uz_x][uz_y] ;
                let draw_rgb = frame_buffer[uz_x][uz_y];
                let _ = &canvas.set_draw_color(
                    Color::RGB(
                        (Clamp(draw_rgb.x,0.0,0.999) *255.0) as u8, 
                        (Clamp(draw_rgb.y,0.0,0.999) *255.0) as u8, 
                        (Clamp(draw_rgb.z,0.0,0.999) *255.0) as u8)
                    ); 
                point = Point::new(x, y);
                let _ = &canvas.draw_point(point).unwrap();
            }
       }
      

       canvas.present();
       frame += 1;
       let end: u32 = SDL_GetTicks();
       _time += end - start;
       println!("rendered frame No: {} took {} ms ",frame, _time);

       }
    }
}

