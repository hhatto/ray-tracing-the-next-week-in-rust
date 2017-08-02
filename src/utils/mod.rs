extern crate rand;

pub mod vec3;
pub mod ray;
pub mod hitable;
pub mod sphere;
pub mod camera;
pub mod material;
pub mod random;

#[macro_export]
macro_rules! get_sphere {
    ( $m:ident, $x:expr, $y:expr, $r:expr ) => {
        {
            Box::new(Sphere::new($y, $r, Box::new($m::new($x))))
        }
    };
    ( $m:ident, $x:expr, $f:expr, $y:expr, $r:expr ) => {
        {
            Box::new(Sphere::new($y, $r, Box::new($m::new($x, $f))))
        }
    };
}

#[macro_export]
macro_rules! get_moving_sphere {
    ( $m:ident, $x:expr, $y:expr, $z:expr, $r:expr ) => {
        {
            Box::new(MovingSphere::new($y, $z, 0., 1., $r, Box::new($m::new($x))))
        }
    };
    ( $m:ident, $x:expr, $f:expr, $y:expr, $z:expr, $r:expr ) => {
        {
            Box::new(MovingSphere::new($y, $z, 0., 1., $r, Box::new($m::new($x, $f))))
        }
    };
}
