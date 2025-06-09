use raytracer::pixel::{Pixel, Rgb, Rgba};
use raytracer::prop::Sphere;
use raytracer::scene::{Camera, Light, Scene};
use raytracer::vector::Vector;
use std::io::{Result, Write, stdout};

fn main() -> Result<()> {
    let scene = Scene {
        props: vec![
            Box::new(Sphere {
                centre: Vector::new(-3., 5., -10.),
                radius: 5.,
                colour: Rgb::red(),
            }),
            Box::new(Sphere {
                centre: Vector::new(4., 5., 10.),
                radius: 5.,
                colour: Rgb::green(),
            }),
        ],
        light: Light {
            position: Vector::new(-5., 13., -15.),
            colour: Rgb::white(),
        },
        camera: Camera::pz_towards_origin(20., 120.),
        eps: 1e-6,
    };

    stdout().write_all(
        &scene
            .render::<Rgba, 2048, 1536>(|_| Rgba::transparent())
            .to_qoi(),
    )
}
