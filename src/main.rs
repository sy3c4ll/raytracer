use raytracer::{Image, Pixel, Rgb, Scene, Sphere, Vector};
use std::io::{Result, Write, stdout};

fn main() -> Result<()> {
    const W: usize = 64;
    const H: usize = 48;

    let scene = Scene {
        props: vec![
            Box::new(Sphere {
                centre: Vector::new(-7., 5., -10.),
                radius: 5.,
                colour: Rgb {
                    r: 0xff,
                    g: 0,
                    b: 0,
                },
            }),
            Box::new(Sphere {
                centre: Vector::new(6., 5., 10.),
                radius: 5.,
                colour: Rgb {
                    r: 0,
                    g: 0xff,
                    b: 0,
                },
            }),
        ],
        camera: Vector::new(0., 0., -20.),
        fov: 145.3,
    };

    let mut image = Image::<Rgb, W, H>::white();
    for y in 0..H {
        for x in 0..W {
            image[[x, y]] = scene.raycast([x, y], [W, H]).unwrap_or(Rgb::white());
        }
    }

    let qoi = image.to_qoi();
    stdout().write_all(&qoi)
}
