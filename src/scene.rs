use std::rc::Rc;

pub struct Scene {
    pub props: Rc<dyn Fn([f64; 3]) -> bool>,
    pub camera: [f64; 3],
    pub focus: f64,
}

impl Scene {
    pub const RIGHT: [f64; 3] = [1., 0., 0.];
    pub const UP: [f64; 3] = [0., 1., 0.];
    pub const FRONT: [f64; 3] = [0., 0., 1.];

    pub fn new(camera: [f64; 3], focus: f64) -> Self {
        Self {
            props: Rc::new(|_| false),
            camera,
            focus,
        }
    }
    pub fn clear(&mut self) {
        self.props = Rc::new(|_| false);
    }
    pub fn stack(&mut self, prop: Rc<dyn Fn([f64; 3]) -> bool>) {
        let props = self.props.clone();
        self.props = Rc::new(move |c| props(c) || prop(c));
    }
    pub fn scoop(&mut self, space: Rc<dyn Fn([f64; 3]) -> bool>) {
        let props = self.props.clone();
        self.props = Rc::new(move |c| props(c) && !space(c));
    }
    pub fn raycast(&self, flat: [f64; 2]) -> bool {
        let _ray = [flat[0], flat[1], self.focus];
        false
    }
}
