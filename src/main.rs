use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Image<const W: usize, const H: usize> {
    pub pixels: [[bool; H]; W],
}

impl<const W: usize, const H: usize> Default for Image<W, H> {
    fn default() -> Self {
        Self {
            pixels: [[false; H]; W],
        }
    }
}

impl<const W: usize, const H: usize> fmt::Display for Image<W, H> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..H {
            for x in 0..W {
                f.write_str(if self.pixels[x][y] { "x" } else { "." })?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

fn predicate_circle(centre: (usize, usize), radius: usize) -> impl Fn((usize, usize)) -> bool {
    move |(x, y)| x.abs_diff(centre.0).pow(2) + y.abs_diff(centre.1).pow(2) <= radius.pow(2)
}

fn main() {
    const W: usize = 64;
    const H: usize = 48;
    let predicate = predicate_circle((32, 24), 5);
    let mut image = Image::<W, H>::default();
    for y in 0..H {
        for x in 0..W {
            image.pixels[x][y] = predicate((x, y));
        }
    }
    println!("{image}");
}
