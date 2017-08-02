use utils::hitable::{Hitable, HitRecord};
use utils::ray::Ray;
use utils::vec3::{dot, Vec3};
use utils::random::drand48;
use utils::material::Material;

#[allow(dead_code)]
pub struct Sphere {
    pub center: Vec3,
    radius: f32,
    mat: Box<Material>,
}

#[allow(dead_code)]
impl Sphere {
    pub fn new(cen: Vec3, r: f32, m: Box<Material>) -> Self {
        Self {
            center: cen,
            radius: r,
            mat: m,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.origin().clone() - self.center.clone();
        let a: f32 = dot(r.direction(), r.direction());
        let b: f32 = dot(&oc, r.direction());
        let c: f32 = dot(&oc, &oc) - self.radius * self.radius;
        let discriminaun = b * b - a * c;
        if discriminaun > 0. {
            let mut temp: f32 = (-b - discriminaun.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p.clone() - self.center.clone()) / self.radius;
                rec.mat = self.mat.clone();
                return true;
            }
            temp = (-b + discriminaun.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p.clone() - self.center.clone()) / self.radius;
                rec.mat = self.mat.clone();
                return true;
            }
        }
        return false;
    }
}


#[allow(dead_code)]
pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    time0: f32,
    time1: f32,
    radius: f32,
    mat: Box<Material>,
}

#[allow(dead_code)]
impl MovingSphere {
    pub fn new(cen0: Vec3, cen1: Vec3, t0: f32, t1: f32, r: f32, m: Box<Material>) -> Self {
        Self {
            center0: cen0,
            center1: cen1,
            time0: t0,
            time1: t1,
            radius: r,
            mat: m,
        }
    }

    fn center(&self, time: f32) -> Vec3 {
        self.center0.clone() +
        (self.center1.clone() - self.center0.clone()) * ((time - self.time0) / (self.time1 - self.time0))
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.origin().clone() - self.center(r.clone().time());
        let a: f32 = dot(r.direction(), r.direction());
        let b: f32 = dot(&oc, r.direction());
        let c: f32 = dot(&oc, &oc) - self.radius * self.radius;
        let discriminaun = b * b - a * c;
        if discriminaun > 0. {
            let mut temp: f32 = (-b - discriminaun.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p.clone() - self.center(r.clone().time())) / self.radius;
                rec.mat = self.mat.clone();
                return true;
            }
            temp = (-b + discriminaun.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p.clone() - self.center(r.clone().time())) / self.radius;
                rec.mat = self.mat.clone();
                return true;
            }
        }
        return false;
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::new(drand48(), drand48(), drand48()) * 2.0 - Vec3::new(1., 1., 1.);
        if p.squared_len() >= 1.0 {
            return p;
        }
    }
}
