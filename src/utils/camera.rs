use std::f32::consts::PI;
use utils::vec3::{Vec3, unit_vector, cross, dot};
use utils::ray::Ray;
use utils::random::drand48;

#[derive(Default)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f32,
    pub time0: f32,
    pub time1: f32,
}

#[allow(dead_code)]
impl Camera {
    pub fn new(lookfrom: &Vec3,
               lookat: &Vec3,
               vup: &Vec3,
               vfov: f32,
               aspect: f32,
               aperture: f32,
               focus_dist: f32,
               t0: f32,
               t1: f32)
               -> Self {
        let lens_radius = aperture / 2.;
        let w: Vec3 = unit_vector(lookfrom.clone() - lookat.clone());
        let u: Vec3 = unit_vector(cross(vup, &w));
        let v: Vec3 = cross(&w, &u);
        let theta = vfov * PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect * half_height;
        let origin = lookfrom.clone();
        Self {
            origin: origin.clone(),
            lower_left_corner: origin - u.clone() * focus_dist * half_width - v.clone() * focus_dist * half_height -
                               w.clone() * focus_dist,
            horizontal: u.clone() * 2. * focus_dist * half_width,
            vertical: v.clone() * 2. * focus_dist * half_height,
            w: w,
            u: u,
            v: v,
            lens_radius: lens_radius,
            time0: t0,
            time1: t1,
            ..Self::default()
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset: Vec3 = self.u.clone() * rd.x() + self.v.clone() * rd.y();
        let ray_vec = self.lower_left_corner.clone() + self.horizontal.clone() * u + self.vertical.clone() * v -
                      self.origin.clone() - offset.clone();
        let origin = self.origin.clone() + offset.clone();
        let time = self.time0 + drand48() * (self.time1 - self.time0);
        Ray::new(&origin, &ray_vec, time)
    }
}

#[allow(dead_code)]
pub fn random_in_unit_disk() -> Vec3 {
    let mut p: Vec3;
    loop {
        p = Vec3::new(drand48(), drand48(), 0.) * 2.0 - Vec3::new(1., 1., 0.);
        if !(dot(&p, &p) >= 1.) {
            break;
        }
    }
    p
}
