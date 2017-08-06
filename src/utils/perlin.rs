use utils::vec3::Vec3;
use utils::random::drand48;

#[derive(Clone)]
pub struct Perlin {
    ranfloat: Vec<f32>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

#[allow(dead_code)]
impl Perlin {
    pub fn new() -> Self {
        Self {
            ranfloat: perlin_generate(),
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: Vec3) -> f32 {
        //let u = p.x() - p.x().floor();
        //let v = p.y() - p.y().floor();
        //let w = p.z() - p.z().floor();
        let i = (4. * p.x()) as i32 & 255;
        let j = (4. * p.y()) as i32 & 255;
        let k = (4. * p.z()) as i32 & 255;
        self.ranfloat[(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize]
    }
}

#[allow(dead_code)]
fn perlin_generate() -> Vec<f32> {
    let mut p: Vec<f32> = vec![0f32; 256];
    for i in 0..256 {
        p[i] = drand48();
    }
    p
}

#[allow(dead_code)]
fn permute(p: &mut Vec<i32>) {
    for i in (0..p.len()).rev() {
        let target = (drand48() * (i + 1) as f32) as i32;
        let tmp = p[i];
        p[i] = p[target as usize];
        p[target as usize] = tmp;
    }
}

#[allow(dead_code)]
fn perlin_generate_perm() -> Vec<i32> {
    let mut p = vec![0i32; 256];
    for (i, v) in p.iter_mut().enumerate() {
        *v = i as i32;
    }
    permute(&mut p);
    p
}
