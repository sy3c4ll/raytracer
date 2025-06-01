use raytracer::Image;
use std::io::{Result, Write, stdout};

fn predicate_circle(centre: [usize; 2], radius: usize) -> impl Fn([usize; 2]) -> bool {
    move |[x, y]| x.abs_diff(centre[0]).pow(2) + y.abs_diff(centre[1]).pow(2) <= radius.pow(2)
}

fn main() -> Result<()> {
    const W: usize = 64;
    const H: usize = 48;
    const R: usize = 5;

    let predicate = predicate_circle([W / 2, H / 2], R);

    let mut image = Image::<bool, W, H>::white();
    for y in 0..H {
        for x in 0..W {
            image[[x, y]] = predicate([x, y]);
        }
    }

    let qoi = image.to_qoi();
    stdout().write_all(&qoi)
}
