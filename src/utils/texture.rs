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
    scale: f32,
}

#[allow(dead_code)]
impl NoiseTexture {
    pub fn new(sc: f32) -> Self {
        Self { noise: Perlin::new(), scale: sc }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: Vec3) -> Vec3 {
        //Vec3::new(1., 1., 1.) * self.noise.noise(p * self.scale)

        //Vec3::new(1., 1., 1.) * 0.5 * (1. + self.noise.turb(p * self.scale, 7))
        //Vec3::new(1., 1., 1.) * self.noise.turb(p * self.scale, 7)  // scale: 10.

        // marble
        //Vec3::new(1., 1., 1.) * 0.5 * (1. + (self.scale * p.z() + 50. * self.noise.turb(p*0.1, 3)).sin())  // scale: 3.1
        //Vec3::new(1., 1., 1.) * 0.5 * (1. + (self.scale * p.z() + 20. * self.noise.turb(p*10., 3)).sin())  // scale: 3.1
        //Vec3::new(1., 1., 1.) * 0.5 * (1. + (self.scale * p.z() + 20. * self.noise.turb(p*2., 3)).sin())  // scale: 3.1
        Vec3::new(1., 1., 1.) * 0.5 * (1. + (self.scale * p.z() + 50. * self.noise.turb(p*2., 3)).sin())  // scale: 3.1
    }

    fn box_clone(&self) -> Box<Texture> {
        Box::new((*self).clone())
    }
}
