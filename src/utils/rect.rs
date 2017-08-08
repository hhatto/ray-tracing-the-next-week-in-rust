use utils::hitable::{Hitable, HitRecord};
use utils::vec3::Vec3;
use utils::ray::Ray;
use utils::material::Material;
use utils::aabb::AABB;

#[derive(Clone)]
pub struct XYRect {
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
    mp: Box<Material>,
}

#[allow(dead_code)]
impl XYRect {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, mt: Box<Material>) -> Self {
        Self {
            x0: x0,
            x1: x1,
            y0: y0,
            y1: y1,
            k: k,
            mp: mt,
        }
    }
}

impl Hitable for XYRect {
    fn hit(&self, r: &Ray, t0: f32, t1: f32, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin().z()) / r.direction().z();
        if t < t0 || t > t1 {
            return false;
        }
        let x = r.origin().x() + t * r.direction().x();
        let y = r.origin().y() + t * r.direction().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        rec.mat = self.mp.clone();
        rec.p = r.point_at_parameter(t);
        rec.normal = Vec3::new(0., 0., 1.);
        true
    }
    fn bounding_box(&self, _t0: f32, _t1: f32, vox: &mut AABB) -> bool {
        *vox = AABB::new(Vec3::new(self.x0, self.y0, self.k - 0.0001), Vec3::new(self.x1, self.y1, self.k + 0.0001));
        true
    }
    fn box_clone(&self) -> Box<Hitable> {
        Box::new((*self).clone())
    }
}
