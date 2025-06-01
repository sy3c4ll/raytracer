use raytracer::{Image, Scene};
use std::io::{Result, Write, stdout};
use std::rc::Rc;

fn main() -> Result<()> {
    const W: usize = 64;
    const H: usize = 48;

    let scene = Scene {
        props: Rc::new(|[x, y, z]| x * x + y * y + z * z < 25.),
        camera: [0., 0., -20.],
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
