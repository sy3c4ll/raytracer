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
        f.write_str("P1\n")?;
        writeln!(f, "{W} {H}")?;
        for y in 0..H {
            for x in 0..W {
                f.write_str(if self.pixels[x][y] { "1" } else { "0" })?;
            }
        }
        f.write_str("\n")?;
        Ok(())
    }
}

fn predicate_circle(centre: (usize, usize), radius: usize) -> impl Fn((usize, usize)) -> bool {
    move |(x, y)| x.abs_diff(centre.0).pow(2) + y.abs_diff(centre.1).pow(2) <= radius.pow(2)
}

fn main() {
    const W: usize = 64;
    const H: usize = 48;
    const R: usize = 5;
    let predicate = predicate_circle((W / 2, H / 2), R);
    let mut image = Image::<W, H>::default();
    for y in 0..H {
        for x in 0..W {
            image.pixels[x][y] = predicate((x, y));
        }
    }
    println!("{image}");
}
