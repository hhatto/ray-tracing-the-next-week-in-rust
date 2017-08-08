#[macro_use]
mod utils;

use std::thread;
use std::sync::Arc;
use std::sync::mpsc::{channel, Receiver, Sender};
use utils::vec3::Vec3;
use utils::ray::Ray;
use utils::hitable::{Hitable, HitableList, HitRecord};
use utils::sphere::Sphere;
use utils::camera::Camera;
use utils::material::{DummyMat, Lambertian, DiffuseLight};
use utils::texture::{NoiseTexture, ConstantTexture};
use utils::rect::XYRect;
use utils::random::drand48;

const CONCURRENCY: i32 = 2;
const NX: i32 = 1200;
const NY: i32 = 800;
const NS: i32 = 100;

fn simple_light() -> HitableList {
    let pertext = NoiseTexture::new(4.);
    let mut list = HitableList::new(vec![]);
    let ctex = ConstantTexture::new(Vec3::new(4., 4., 4.));
    let difflight = DiffuseLight::new(Box::new(ctex.clone()));

    list.list.push(get_sphere!(Lambertian, Box::new(pertext.clone()), Vec3::new(0., -1000., 0.), 1000.));
    list.list.push(get_sphere!(Lambertian, Box::new(pertext.clone()), Vec3::new(0., 2., 0.), 2.));
    // list.list.push(get_sphere!(DiffuseLight, Box::new(ctex), Vec3::new(0., 7., 0.), 2.));
    list.list.push(Box::new(XYRect::new(3., 5., 1., 3., -2., Box::new(difflight))));
    list
}

fn color(r: &Ray, world: &Arc<HitableList>, depth: i32) -> Vec3 {
    let mut rec = HitRecord::new(Box::new(DummyMat::new()));
    if world.hit(r, 0.001, std::f32::MAX, &mut rec) {
        let v1 = Vec3::new(0., 0., 0.);
        let v2 = Vec3::new(0., 0., 0.);
        let mut scattered = Ray::new(&v1, &v2, 0.);
        let mut attenuation = Vec3::new(0., 0., 0.);
        let emitted = rec.mat.emitted(rec.u, rec.v, rec.p.clone());
        if depth < 50 &&
           rec.mat
            .as_ref()
            .scatter(r, &rec, &mut attenuation, &mut scattered) {
            return emitted + attenuation * color(&scattered, world, depth + 1);
        }
        return emitted;
    }
    Vec3::new(0., 0., 0.)
}

fn get_color(i: f32, j: f32, nx: f32, ny: f32, cam: &Arc<Camera>, world: &Arc<HitableList>) -> Vec3 {
    let u: f32 = (i as f32 + drand48()) / nx as f32;
    let v: f32 = (j as f32 + drand48()) / ny as f32;
    let r = cam.get_ray(u, v);
    color(&r, world, 0)
}

fn exec_worker(cam: &Arc<Camera>,
               world: &Arc<HitableList>,
               rx: Receiver<Option<(f32, f32, f32, f32)>>,
               cx: Sender<Option<Vec3>>) {
    loop {
        match rx.recv().unwrap() {
            Some(arg) => {
                let r = get_color(arg.0, arg.1, arg.2, arg.3, cam, world);
                cx.send(Some(r)).unwrap();
            }
            None => {
                // kill thread
                return;
            }
        }
    }
}

fn main() {
    let nx = NX;
    let ny = NY;
    let ns = NS;
    println!("P3\n{} {}\n255", nx, ny);

    let world: HitableList = simple_light();

    let lookfrom = Vec3::new(13., 2., 3.);
    let lookat = Vec3::new(0., 0., 0.);
    let dist_to_focus = 10.;
    let aperture: f32 = 0.1;
    let cam = Camera::new(&lookfrom,
                          &lookat,
                          &Vec3::new(0., 1., 0.),
                          60.,
                          nx as f32 / ny as f32,
                          aperture,
                          dist_to_focus,
                          0.,
                          1.);
    let mut workers = vec![];
    let mut handles = vec![];
    let world_arc = Arc::new(world);
    let cam_arc = Arc::new(cam);
    let (calc_tx, calc_rx) = channel::<Option<Vec3>>();

    for _ in 0..CONCURRENCY {
        let world = world_arc.clone();
        let cam = cam_arc.clone();
        let (worker_tx, worker_rx) = channel::<Option<(f32, f32, f32, f32)>>();
        workers.push(worker_tx.clone());
        let c_tx = calc_tx.clone();
        handles.push(thread::spawn(move || exec_worker(&cam, &world, worker_rx, c_tx)));
    }

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0., 0., 0.);
            for cnt in 0..ns {
                let offset = cnt % CONCURRENCY;
                let req = workers[offset as usize].clone();
                req.send(Some((i as f32, j as f32, nx as f32, ny as f32)))
                    .unwrap();
            }

            for _ in 0..ns {
                match calc_rx.recv().unwrap() {
                    Some(ret) => col = col + ret,
                    None => break,
                }
            }
            col = col / ns as f32;
            col = Vec3::new(col.e[0].sqrt(), col.e[1].sqrt(), col.e[2].sqrt());
            let ir = (255.99 * col.e[0]) as i32;
            let ig = (255.99 * col.e[1]) as i32;
            let ib = (255.99 * col.e[2]) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }

    for worker in workers {
        let req = worker.clone();
        req.send(None).unwrap();
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
