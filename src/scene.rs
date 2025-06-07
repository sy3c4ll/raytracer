use crate::{Camera, Image, Pixel, Prop, Rgb};

pub struct Scene {
    pub props: Vec<Box<dyn Prop>>,
    pub bg: Rgb,
    pub camera: Camera,
}

impl Scene {
    pub fn new(camera: Camera) -> Self {
        Self {
            props: Vec::new(),
            bg: Rgb::white(),
            camera,
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
            .filter_map(|p| Some((p, p.raycast(self.camera.eye, ray)?)))
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(p, _d)| p.colour())
    }
    pub fn render<const W: usize, const H: usize>(&self) -> Image<Rgb, W, H> {
        Image::fill_with(|c| self.raycast(c, [W, H]).unwrap_or(self.bg))
    }
}
