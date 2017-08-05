use std::vec::Vec;
use utils::vec3::Vec3;
use utils::ray::Ray;
use utils::material::Material;
use utils::aabb::{AABB, surrounding_box};

#[allow(dead_code)]
#[derive(Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: Box<Material>,
}

#[allow(dead_code)]
impl HitRecord {
    pub fn new(m: Box<Material>) -> Self {
        Self {
            t: 0.,
            p: Vec3::new(0., 0., 0.),
            normal: Vec3::new(0., 0., 0.),
            mat: m,
        }
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, t0: f32, t1: f32, vox: &mut AABB) -> bool;
    fn box_clone(&self) -> Box<Hitable>;
}

impl Clone for Box<Hitable> {
    fn clone(&self) -> Box<Hitable> {
        self.box_clone()
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct HitableList {
    pub list: Vec<Box<Hitable>>,
}

unsafe impl Sync for HitableList {}
unsafe impl Send for HitableList {}

#[allow(dead_code)]
impl HitableList {
    pub fn new(hitable: Vec<Box<Hitable>>) -> Self {
        Self { list: hitable }
    }
}

impl Hitable for HitableList {
    fn box_clone(&self) -> Box<Hitable> {
        Box::new((*self).clone())
    }
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new(rec.mat.clone());
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for h in self.list.iter() {
            if h.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }

    fn bounding_box(&self, t0: f32, t1: f32, vox: &mut AABB) -> bool {
        if self.list.len() < 1 {
            return false;
        }

        let mut temp_box = AABB::new(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 0.));
        let first_true = self.list[0].bounding_box(t0, t1, &mut temp_box);
        if !first_true {
            return false;
        } else {
            *vox = temp_box.clone();
        }

        for _ in 1..self.list.len() {
            if self.list[0].bounding_box(t0, t1, &mut temp_box) {
                *vox = surrounding_box(vox.clone(), temp_box.clone());
            } else {
                return false;
            }
        }

        true
    }
}
