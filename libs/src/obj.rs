pub mod objects{
    extern crate derivative;
    use self::derivative::Derivative;
    use vec3::vector3::Vec3;
    #[derive(Derivative)]
    #[derive(Copy, Clone, Default)]
    pub struct Material{
        #[derivative(Default(value = "Vec3(0.0, 0.0, 0.0)"))]
        pub color:Vec3,
        #[derivative(Default(value = "Vec3(0.0, 0.0, 0.0)"))]
        pub emission_color:Vec3,
        #[derivative(Default(value = "0.0"))]
        pub emission_strength:f64,
        #[derivative(Default(value = "0.0"))]
        pub roughness: f64,
        #[derivative(Default(value = "false"))]
        pub is_glossy: bool,
        #[derivative(Default(value = "0.0"))]
        pub refractive_index: f64,
        #[derivative(Default(value = "0.0"))]
        pub metallic: f64

      
    }
    #[derive(Copy, Clone)]
    pub struct Sphere{
        pub centre:Vec3,
        pub radius:f64,
        pub material: Material,
    }
    #[derive(Clone)]
    pub struct ObjManager{
        pub single_sphere: Vec<Sphere>
    }

}
