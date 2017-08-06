use utils::vec3::Vec3;
use utils::perlin::Perlin;

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3;
    fn box_clone(&self) -> Box<Texture>;
}

impl Clone for Box<Texture> {
    fn clone(&self) -> Box<Texture> {
        self.box_clone()
    }
}

#[derive(Clone)]
pub struct ConstantTexture {
    color: Vec3,
}

#[allow(dead_code)]
impl ConstantTexture {
    pub fn new(c: Vec3) -> Self {
        Self { color: c }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _u: f32, _v: f32, _p: Vec3) -> Vec3 {
        self.color.clone()
    }

    fn box_clone(&self) -> Box<Texture> {
        Box::new((*self).clone())
    }
}

#[derive(Clone)]
pub struct CheckerTexture {
    odd: Box<Texture>,
    even: Box<Texture>,
}

#[allow(dead_code)]
impl CheckerTexture {
    pub fn new(t0: Box<Texture>, t1: Box<Texture>) -> Self {
        Self { odd: t0, even: t1 }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        let sines = (10. * p.x()).sin() * (10. * p.y()).sin() * (10. * p.z()).sin();
        if sines < 0. {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }

    fn box_clone(&self) -> Box<Texture> {
        Box::new((*self).clone())
    }
}

#[derive(Clone)]
pub struct NoiseTexture {
    noise: Perlin,
}

#[allow(dead_code)]
impl NoiseTexture {
    pub fn new() -> Self {
        Self { noise: Perlin::new() }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: Vec3) -> Vec3 {
        Vec3::new(1., 1., 1.) * self.noise.noise(p)
    }

    fn box_clone(&self) -> Box<Texture> {
        Box::new((*self).clone())
    }
}
