use crate::{Prop, Vector};

pub struct Scene {
    pub props: Vec<Box<dyn Prop>>,
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
    pub fn push(&mut self, prop: impl Prop) {
        self.props.push(Box::new(prop));
    }
    pub fn raycast(&self, flat: [f64; 2]) -> bool {
        let ray = Vector::new(flat[0], flat[1], self.focus);
        self.props[0].raycast(self.camera, ray).is_some()
    }
}
