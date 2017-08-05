use std::mem;
use std::cmp;
use utils::vec3::Vec3;
use utils::ray::Ray;
use utils::hitable::{Hitable, HitRecord};

#[derive(Clone, Default, Debug)]
pub struct AABB {
    min: Vec3,
    max: Vec3,
}

#[allow(dead_code)]
impl AABB {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self { min: a, max: b }
    }

    pub fn min(self) -> Vec3 {
        self.min
    }

    pub fn max(self) -> Vec3 {
        self.max
    }
}

impl Hitable for AABB {
    fn box_clone(&self) -> Box<Hitable> {
        Box::new((*self).clone())
    }
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32, rec: &mut HitRecord) -> bool {
        for a in 0..3 {
            // let t0 = ffmin((self.min.e[a] - r.origin().e[a]) / r.direction().e[a],
            //               (self.max.e[a] - r.origin().e[a]) / r.direction().e[a]);
            // let t1 = ffmax((self.min.e[a] - r.origin().e[a]) / r.direction().e[a],
            //               (self.max.e[a] - r.origin().e[a]) / r.direction().e[a]);
            // let tmin = ffmax(t0, tmin);
            // let tmax = ffmax(t1, tmax);

            let inv_d = 1. / r.direction().e[a];
            let mut t0 = (self.clone().min().e[a] - r.origin().e[a]) * inv_d;
            let mut t1 = (self.clone().max().e[a] - r.origin().e[a]) * inv_d;
            if inv_d < 0. {
                mem::swap(&mut t0, &mut t1);
            }
            let tmin = if t0 > tmin {
                t0
            } else {
                tmin
            };
            let tmax = if t1 < tmax {
                t1
            } else {
                tmax
            };

            if tmax <= tmin {
                return false;
            }
        }

        true
    }

    fn bounding_box(&self, t0: f32, t1: f32, vox: &mut AABB) -> bool {
        false
    }
}

fn ffmin(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

fn ffmax(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

pub fn surrounding_box(b0: AABB, b1: AABB) -> AABB {
    let min = Vec3::new(b0.clone().min().x().min(b1.clone().min().x()),
                        b0.clone().min().y().min(b1.clone().min().y()),
                        b0.clone().min().z().min(b1.clone().min().z()));
    let max = Vec3::new(b0.clone().max().x().max(b1.clone().max().x()),
                        b0.clone().max().y().max(b1.clone().max().y()),
                        b0.max().z().max(b1.max().z()));
    AABB {
        min: min,
        max: max,
    }
}
