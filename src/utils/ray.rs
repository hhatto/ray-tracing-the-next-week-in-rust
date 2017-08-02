use utils::vec3;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Ray {
    pub a: vec3::Vec3,
    pub b: vec3::Vec3,
    time: f32,
}

#[allow(dead_code)]
impl Ray {
    pub fn new(a: &vec3::Vec3, b: &vec3::Vec3, ti: f32) -> Self {
        Self {
            a: a.clone(),
            b: b.clone(),
            time: ti,
        }
    }

    pub fn origin(&self) -> &vec3::Vec3 {
        &self.a
    }

    pub fn direction(&self) -> &vec3::Vec3 {
        &self.b
    }

    pub fn time(self) -> f32 {
        self.time
    }

    pub fn point_at_parameter(&self, t: f32) -> vec3::Vec3 {
        let z: vec3::Vec3 = self.a.clone() + self.b.clone() * t;
        z
    }
}
