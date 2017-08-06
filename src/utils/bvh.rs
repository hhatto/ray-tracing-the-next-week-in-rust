use std::cmp::Ordering;
use utils::aabb::{AABB, surrounding_box};
use utils::hitable::{Hitable, HitRecord};
use utils::random::drand48;
use utils::ray::Ray;
use utils::vec3::Vec3;

macro_rules! box_compare {
    ( $i:ident, $a:ident, $b:ident ) => {
        {
    let mut box_left = AABB::new(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 0.));
    let mut box_right = AABB::new(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 0.));
    if !$a.bounding_box(0., 0., &mut box_left) || !$b.bounding_box(0., 0., &mut box_right) {
        // error
    }
    let c = box_left.min().$i() - box_right.min().$i();
    if c < 0. {
        Ordering::Less
    } else if c > 0. {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
        }
    }
}

#[derive(Clone)]
struct BVHNode {
    left: Box<Hitable>,
    right: Box<Hitable>,
    vox: AABB,
}

#[allow(dead_code)]
impl BVHNode {
    pub fn new(l: &mut Vec<Box<Hitable>>, time0: f32, time1: f32) -> Self {
        // let mut left = Box::new(AABB::new(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 0.)));
        // let mut right = Box::new(AABB::new(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 0.)));
        let axis = (3. * drand48()) as i32;
        if axis == 0 {
            l.sort_by(|a, b| box_compare!(x, a, b));
        } else if axis == 1 {
            l.sort_by(|a, b| box_compare!(y, a, b));
        } else {
            l.sort_by(|a, b| box_compare!(z, a, b));
        }
        let (left, right): (Box<Hitable>, Box<Hitable>) = if l.len() == 1 {
            (l[0].clone(), l[0].clone())
        } else if l.len() == 2 {
            (l[0].clone(), l[1].clone())
        } else {
               let (svf, svl) = l.split_at(l.len() / 2);
               let mut vl: Vec<Box<Hitable>> = vec![];
               let mut vf: Vec<Box<Hitable>> = vec![];
               //let vf: Vec<Box<Hitable>> = svf.iter().map(|&i| i);
               vf.extend(svf.iter().map(|ref i| (**i).clone()));
               vl.extend(svl.iter().map(|ref i| (**i).clone()));
               (Box::new(BVHNode::new(&mut vf, time0, time1)), Box::new(BVHNode::new(&mut vl, time0, time1)))
        };

        let mut box_left = AABB::new(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 0.));
        let mut box_right = AABB::new(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 0.));
        if !left.bounding_box(time0, time1, &mut box_left) || !right.bounding_box(time0, time1, &mut box_right) {
            // error
        }
        Self {
            left: left,
            right: right,
            vox: surrounding_box(box_left, box_right),
        }
    }
}

impl Hitable for BVHNode {
    fn box_clone(&self) -> Box<Hitable> {
        Box::new((*self).clone())
    }
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new(rec.mat.clone());
        if self.vox.hit(r, t_min, t_max, &mut temp_rec) {
            let (mut left_rec, mut right_rec) = (HitRecord::new(rec.mat.clone()), HitRecord::new(rec.mat.clone()));
            let hit_left = self.left.hit(r, t_min, t_max, &mut left_rec);
            let hit_right = self.left.hit(r, t_min, t_max, &mut right_rec);
            if hit_left && hit_right {
                *rec = if left_rec.t < right_rec.t {
                    left_rec
                } else {
                    right_rec
                };
                return true;
            } else if hit_left {
                *rec = left_rec;
                return true;
            } else if hit_right {
                *rec = right_rec;
                return true;
            }
        }
        false
    }

    fn bounding_box(&self, _t0: f32, _t1: f32, b: &mut AABB) -> bool {
        *b = self.vox.clone();
        true
    }
}

// fn box_compare(a: hitable, b: hitable) -> ordering {
