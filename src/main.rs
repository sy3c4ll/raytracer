use raytracer::{Pixel, Rgb, Scene, Sphere, Vector};
use std::io::{Result, Write, stdout};

fn main() -> Result<()> {
    let scene = Scene {
        props: vec![
            Box::new(Sphere {
                centre: Vector::new(-3., 5., -10.),
                radius: 5.,
                colour: Rgb {
                    r: 0xff,
                    g: 0,
                    b: 0,
                },
            }),
            Box::new(Sphere {
                centre: Vector::new(4., 5., 10.),
                radius: 5.,
                colour: Rgb {
                    r: 0,
                    g: 0xff,
                    b: 0,
                },
            }),
        ],
        bg: Rgb::white(),
        camera: Vector::new(0., 0., -20.),
        fov: 120.,
    };

    stdout().write_all(&scene.render::<7680, 4320>().to_qoi())
}
