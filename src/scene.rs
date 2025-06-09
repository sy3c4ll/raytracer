use crate::image::Image;
use crate::pixel::{Pixel, Rgb};
use crate::prop::{HitRecord, Prop};
use crate::vector::Vector;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Light {
    pub position: Vector,
    pub colour: Rgb,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    pub eye: Vector,
    centre: Vector,
    up: Vector,
    right: Vector,
    pub hfov: f64,
}

#[derive(Debug)]
pub struct Scene {
    pub props: Vec<Box<dyn Prop>>,
    pub light: Light,
    pub camera: Camera,
    pub eps: f64,
}

impl Scene {
    pub fn new(light: Light, camera: Camera) -> Self {
        Self {
            props: Vec::new(),
            light,
            camera,
            eps: 1e-6,
        }
    }
    pub fn clear(&mut self) {
        self.props.clear();
    }
    pub fn push(&mut self, prop: impl Prop) {
        self.props.push(Box::new(prop));
    }
    pub fn raycast(&self, [x, y]: [usize; 2], [w, h]: [usize; 2]) -> Option<Rgb> {
        let focus = w as f64 / 2. / (self.camera.hfov / 2.).to_radians().tan();
        let xproj = (x as isize - w as isize / 2) as f64;
        let yproj = -(y as isize - h as isize / 2) as f64;

        let ray =
            focus * self.camera.centre() + xproj * self.camera.right() + yproj * self.camera.up();

        self.props
            .iter()
            .filter_map(|p| p.raycast(self.camera.eye, ray, self.eps))
            .min_by(|a, b| a.distance.total_cmp(&b.distance))
            .map(|h| self.shade(h))
    }
    pub fn shade(&self, hit: HitRecord) -> Rgb {
        let disp = self.light.position - hit.position;
        let occluded = self.props.iter().any(|p| {
            p.raycast(hit.position, disp, self.eps)
                .is_some_and(|h| h.distance - disp.abs() <= -self.eps)
        });
        let attenuation = if occluded {
            0.
        } else {
            (disp.norm() * hit.normal.norm()).max(0.)
        };
        let shaded_colour = Rgb {
            r: (hit.colour.r as u16 * self.light.colour.r as u16 / 0xff) as u8,
            g: (hit.colour.g as u16 * self.light.colour.g as u16 / 0xff) as u8,
            b: (hit.colour.b as u16 * self.light.colour.b as u16 / 0xff) as u8,
        };
        Rgb {
            r: (shaded_colour.r as f64 * attenuation) as u8,
            g: (shaded_colour.g as f64 * attenuation) as u8,
            b: (shaded_colour.b as f64 * attenuation) as u8,
        }
    }
    pub fn render<P: Pixel, const W: usize, const H: usize>(
        &self,
        mut bg: impl FnMut([usize; 2]) -> P,
    ) -> Image<P, W, H> {
        Image::fill_with(|c| {
            self.raycast(c, [W, H])
                .map(Pixel::from_rgb)
                .unwrap_or_else(|| bg(c))
        })
    }
    pub fn render_on<P: Pixel, const W: usize, const H: usize>(
        &self,
        mut image: Image<P, W, H>,
    ) -> Image<P, W, H> {
        for (y, row) in image.iter_mut().enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                if let Some(rgb) = self.raycast([x, y], [W, H]) {
                    *pixel = Pixel::from_rgb(rgb);
                }
            }
        }
        image
    }
}

impl Camera {
    pub const fn centre(&self) -> Vector {
        self.centre
    }
    pub const fn up(&self) -> Vector {
        self.up
    }
    pub const fn right(&self) -> Vector {
        self.right
    }
    pub fn new(eye: Vector, centre: Vector, up: Vector, hfov: f64) -> Self {
        let centre = centre.norm();
        let up = up.norm();
        let right = (up ^ centre).norm();
        let up = (centre ^ right).norm();
        Self {
            eye,
            centre,
            up,
            right,
            hfov,
        }
    }
    pub const fn px_towards_origin(dist: f64, hfov: f64) -> Self {
        const CENTRE: Vector = Vector::I;
        const UP: Vector = Vector::J;
        const RIGHT: Vector = UP.cross(CENTRE);
        Self {
            eye: Vector::new(-dist, 0., 0.),
            hfov,
            up: UP,
            centre: CENTRE,
            right: RIGHT,
        }
    }
    pub const fn py_towards_origin(dist: f64, hfov: f64) -> Self {
        const CENTRE: Vector = Vector::J;
        const UP: Vector = Vector::K.neg();
        const RIGHT: Vector = UP.cross(CENTRE);
        Self {
            eye: Vector::new(0., -dist, 0.),
            centre: CENTRE,
            up: UP,
            right: RIGHT,
            hfov,
        }
    }
    pub const fn pz_towards_origin(dist: f64, hfov: f64) -> Self {
        const CENTRE: Vector = Vector::K;
        const UP: Vector = Vector::J;
        const RIGHT: Vector = UP.cross(CENTRE);
        Self {
            eye: Vector::new(0., 0., -dist),
            centre: CENTRE,
            up: UP,
            right: RIGHT,
            hfov,
        }
    }
    pub const fn nx_towards_origin(dist: f64, hfov: f64) -> Self {
        const CENTRE: Vector = Vector::I.neg();
        const UP: Vector = Vector::J;
        const RIGHT: Vector = UP.cross(CENTRE);
        Self {
            eye: Vector::new(dist, 0., 0.),
            centre: CENTRE,
            up: UP,
            right: RIGHT,
            hfov,
        }
    }
    pub const fn ny_towards_origin(dist: f64, hfov: f64) -> Self {
        const CENTRE: Vector = Vector::J.neg();
        const UP: Vector = Vector::K;
        const RIGHT: Vector = UP.cross(CENTRE);
        Self {
            eye: Vector::new(0., dist, 0.),
            centre: CENTRE,
            up: UP,
            right: RIGHT,
            hfov,
        }
    }
    pub const fn nz_towards_origin(dist: f64, hfov: f64) -> Self {
        const CENTRE: Vector = Vector::K.neg();
        const UP: Vector = Vector::J;
        const RIGHT: Vector = UP.cross(CENTRE);
        Self {
            eye: Vector::new(0., 0., dist),
            centre: CENTRE,
            up: UP,
            right: RIGHT,
            hfov,
        }
    }
}
