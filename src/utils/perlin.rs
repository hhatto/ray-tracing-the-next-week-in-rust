use utils::vec3::{Vec3, unit_vector, dot};
use utils::random::drand48;

#[derive(Clone)]
pub struct Perlin {
    //ranfloat: Vec<f31>,
    ranvec: Vec<(f32, f32, f32)>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

#[allow(dead_code)]
impl Perlin {
    pub fn new() -> Self {
        Self {
            //ranfloat: perlin_generate(),
            ranvec: perlin_generate(),
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }

    pub fn turb(&self, p: Vec3, depth: i32) -> f32 {
        let mut accum: f32 = 0.;
        let mut temp_p: Vec3 = p;
        let mut weight: f32 = 1.0;
        for _ in 0..depth {
            accum += weight * self.noise(temp_p.clone());
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }
        accum.abs()
    }

    pub fn noise(&self, p: Vec3) -> f32 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
        //u = u * u * (3. - 2.*u);
        //v = v * v * (3. - 2.*v);
        //w = w * w * (3. - 2.*w);
        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        //let i = (4. * p.x()) as i32 & 255;
        //let j = (4. * p.y()) as i32 & 255;
        //let k = (4. * p.z()) as i32 & 255;
        //self.ranfloat[(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize]

        let mut c = [[[(0f32, 0f32, 0f32);2];2];2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di as usize][dj as usize][dk as usize] =
                        self.ranvec[(self.perm_x[((i+di)&255) as usize] ^ self.perm_y[((j+dj)&255) as usize] ^ self.perm_z[(k+dk&255) as usize]) as usize];
                }
            }
        }
        //trilinear_interp(&c, u, v, w)
        perlin_interp(&c, u, v, w)
    }
}

#[allow(dead_code)]
fn perlin_generate() -> Vec<(f32, f32, f32)> {
    let mut p: Vec<(f32, f32, f32)> = vec![(0., 0., 0.); 256];
    for i in 0..256 {
        let v = unit_vector(Vec3::new(-1. + 2. * drand48(),
                                      -1. + 2. * drand48(),
                                      -1. + 2. * drand48()));
        p[i] = (v.e[0], v.e[1], v.e[2]);
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

#[allow(dead_code)]
fn trilinear_interp(c: &[[[f32;2];2];2], u: f32, v: f32, w: f32) -> f32 {
    let mut accum: f32 = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                accum += (i as f32 * u + (1-i) as f32 * (1.-u))*
                         (j as f32 * v + (1-j) as f32 * (1.-v))*
                         (k as f32 * w + (1-k) as f32 * (1.-w))*
                         c[i][j][k];
            }
        }
    }
    accum
}

#[allow(dead_code)]
fn perlin_interp(c: &[[[(f32, f32, f32);2];2];2], u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3. - 2.*u);
    let vv = v * v * (3. - 2.*v);
    let ww = w * w * (3. - 2.*w);
    let mut accum: f32 = 0.;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let vp = c[i][j][j];
                let dot_v = Vec3::new(vp.0, vp.1, vp.2);
                let weight_v = Vec3::new(u  - (i as f32), v - (j as f32), w - (k as f32));
                accum += (i as f32 * uu + (1.-i as f32) * (1.-uu))*
                         (j as f32 * vv + (1.-j as f32) * (1.-vv))*
                         (k as f32 * ww + (1.-k as f32) * (1.-ww))*
                         dot(&dot_v, &weight_v);
            }
        }
    }
    accum
}
