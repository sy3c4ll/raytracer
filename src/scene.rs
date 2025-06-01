use crate::Vector;

pub struct Scene {
    pub props: Vec<Sphere>,
    pub camera: Vector,
    pub focus: f64,
}

impl Scene {
    pub const RIGHT: Vector = Vector::new(1., 0., 0.);
    pub const UP: Vector = Vector::new(0., 1., 0.);
    pub const FRONT: Vector = Vector::new(0., 0., 1.);

    pub fn new(camera: Vector, focus: f64) -> Self {
        Self {
            props: Vec::new(),
            camera,
            focus,
        }
    }
    pub fn clear(&mut self) {
        self.props.clear();
    }
    pub fn push(&mut self, prop: Sphere) {
        self.props.push(prop);
    }
    pub fn raycast(&self, flat: [f64; 2]) -> bool {
        let ray = Vector::new(flat[0], flat[1], self.focus);
        let disp = self.camera - self.props[0].centre;
        (disp * ray).powf(2.) - disp.sq() * ray.sq() + ray.sq() * self.props[0].radius.powf(2.)
            >= 0.
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub centre: Vector,
    pub radius: f64,
}
