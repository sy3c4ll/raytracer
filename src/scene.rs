use crate::image::Image;
use crate::pixel::{Pixel, Rgb};
use crate::prop::{HitRecord, Prop};
use crate::vector::{Ray, Vector};

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
    unit_focus: f64,
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
        let focus = self.camera.focus(w);
        let xproj = (x as isize - w as isize / 2) as f64;
        let yproj = -(y as isize - h as isize / 2) as f64;

        let ray = Ray {
            eye: self.camera.eye,
            dir: focus * self.camera.centre()
                + xproj * self.camera.right()
                + yproj * self.camera.up(),
        };

        self.props
            .iter()
            .filter_map(|p| p.raycast(ray, self.eps))
            .min_by(|a, b| a.distance.total_cmp(&b.distance))
            .map(|h| self.shade(h))
    }
    pub fn shade(&self, hit: HitRecord) -> Rgb {
        let disp = self.light.position - hit.position;
        let occluded = self.props.iter().any(|p| {
            p.raycast(Ray::new(hit.position, disp), self.eps)
                .is_some_and(|h| h.distance - disp.abs() <= -self.eps)
        });

        let ambient = hit.material.ambient * hit.material.colour * self.light.colour;

        if occluded {
            ambient
        } else {
            let l = disp.norm();
            let cos_d = l * hit.normal;
            let diffuse = hit.material.diffuse * cos_d.max(0.) * hit.material.colour;

            let r = 2. * cos_d * hit.normal - l;
            let cos_s = r * -hit.ray.dir.norm();
            let specular = hit.material.specular
                * cos_s.max(0.).powf(hit.material.shininess)
                * self.light.colour;

            ambient + diffuse + specular
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
    pub const fn focus(&self, w: usize) -> f64 {
        w as f64 * self.unit_focus
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
            unit_focus: 0.5 / (hfov / 2.).to_radians().tan(),
        }
    }
    pub fn px_towards_origin(dist: f64, hfov: f64) -> Self {
        const CENTRE: Vector = Vector::I;
        const UP: Vector = Vector::J;
        const RIGHT: Vector = UP.cross(CENTRE);
        Self {
            eye: Vector::new(-dist, 0., 0.),
            up: UP,
            centre: CENTRE,
            right: RIGHT,
            hfov,
            unit_focus: 0.5 / (hfov / 2.).to_radians().tan(),
        }
    }
    pub fn py_towards_origin(dist: f64, hfov: f64) -> Self {
        const CENTRE: Vector = Vector::J;
        const UP: Vector = Vector::K.neg();
        const RIGHT: Vector = UP.cross(CENTRE);
        Self {
            eye: Vector::new(0., -dist, 0.),
            centre: CENTRE,
            up: UP,
            right: RIGHT,
            hfov,
            unit_focus: 0.5 / (hfov / 2.).to_radians().tan(),
        }
    }
    pub fn pz_towards_origin(dist: f64, hfov: f64) -> Self {
        const CENTRE: Vector = Vector::K;
        const UP: Vector = Vector::J;
        const RIGHT: Vector = UP.cross(CENTRE);
        Self {
            eye: Vector::new(0., 0., -dist),
            centre: CENTRE,
            up: UP,
            right: RIGHT,
            hfov,
            unit_focus: 0.5 / (hfov / 2.).to_radians().tan(),
        }
    }
    pub fn nx_towards_origin(dist: f64, hfov: f64) -> Self {
        const CENTRE: Vector = Vector::I.neg();
        const UP: Vector = Vector::J;
        const RIGHT: Vector = UP.cross(CENTRE);
        Self {
            eye: Vector::new(dist, 0., 0.),
            centre: CENTRE,
            up: UP,
            right: RIGHT,
            hfov,
            unit_focus: 0.5 / (hfov / 2.).to_radians().tan(),
        }
    }
    pub fn ny_towards_origin(dist: f64, hfov: f64) -> Self {
        const CENTRE: Vector = Vector::J.neg();
        const UP: Vector = Vector::K;
        const RIGHT: Vector = UP.cross(CENTRE);
        Self {
            eye: Vector::new(0., dist, 0.),
            centre: CENTRE,
            up: UP,
            right: RIGHT,
            hfov,
            unit_focus: 0.5 / (hfov / 2.).to_radians().tan(),
        }
    }
    pub fn nz_towards_origin(dist: f64, hfov: f64) -> Self {
        const CENTRE: Vector = Vector::K.neg();
        const UP: Vector = Vector::J;
        const RIGHT: Vector = UP.cross(CENTRE);
        Self {
            eye: Vector::new(0., 0., dist),
            centre: CENTRE,
            up: UP,
            right: RIGHT,
            hfov,
            unit_focus: 0.5 / (hfov / 2.).to_radians().tan(),
        }
    }
}
