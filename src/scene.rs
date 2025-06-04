use crate::{Prop, Rgb, Vector};

pub struct Scene {
    pub props: Vec<Box<dyn Prop>>,
    pub camera: Vector,
    // Horizontal FoV in degrees
    pub fov: f64,
}

impl Scene {
    pub const RIGHT: Vector = Vector::new(1., 0., 0.);
    pub const UP: Vector = Vector::new(0., 1., 0.);
    pub const FRONT: Vector = Vector::new(0., 0., 1.);

    pub fn new(camera: Vector, fov: f64) -> Self {
        Self {
            props: Vec::new(),
            camera,
            fov,
        }
    }
    pub fn clear(&mut self) {
        self.props.clear();
    }
    pub fn push(&mut self, prop: impl Prop) {
        self.props.push(Box::new(prop));
    }
    pub fn raycast(&self, [x, y]: [usize; 2], [w, h]: [usize; 2]) -> Option<Rgb> {
        let right = (x as isize - w as isize / 2) as f64;
        let up = -(y as isize - h as isize / 2) as f64;
        let focus = w as f64 / 2. / (self.fov / 2.).to_radians().tan();
        let ray = right * Self::RIGHT + up * Self::UP + focus * Self::FRONT;
        self.props
            .iter()
            .filter_map(|p| Some((p, p.raycast(self.camera, ray)?)))
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(p, _d)| p.colour())
    }
}
