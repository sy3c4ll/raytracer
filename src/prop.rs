use crate::{Rgb, Vector};

pub trait Prop: 'static {
    fn colour(&self) -> Rgb;
    fn raycast(&self, eye: Vector, ray: Vector) -> Option<f64>;
}

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub centre: Vector,
    pub radius: f64,
    pub colour: Rgb,
}

impl Prop for Sphere {
    fn colour(&self) -> Rgb {
        self.colour
    }
    fn raycast(&self, camera: Vector, ray: Vector) -> Option<f64> {
        let disp = self.centre - camera;
        let disc = (disp * ray).powf(2.) - disp.sq() * ray.sq() + ray.sq() * self.radius.powf(2.);
        if disc >= 0. {
            let t = ((disp * ray) - disc.sqrt()) / ray.sq();
            Some(t * ray.abs())
        } else {
            None
        }
    }
}
