use raytracer::{Image, Scene, Sphere, Vector};
use std::io::{Result, Write, stdout};

fn main() -> Result<()> {
    const W: usize = 64;
    const H: usize = 48;

    let scene = Scene {
        props: vec![Box::new(Sphere {
            centre: Vector::new(0., 0., 0.),
            radius: 5.,
        })],
        camera: Vector::new(0., 0., -20.),
        focus: 10.,
    };

    let mut image = Image::<bool, W, H>::white();
    for y in 0..H {
        for x in 0..W {
            image[[x, y]] = scene.raycast([
                (x as isize - W as isize / 2) as f64,
                (y as isize - H as isize / 2) as f64,
            ]);
        }
    }

    let qoi = image.to_qoi();
    stdout().write_all(&qoi)
}
