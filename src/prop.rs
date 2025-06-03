use crate::Vector;

pub trait Prop: 'static {
    fn raycast(&self, camera: Vector, ray: Vector) -> Option<Vector>;
}

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub centre: Vector,
    pub radius: f64,
}

impl Prop for Sphere {
    fn raycast(&self, camera: Vector, ray: Vector) -> Option<Vector> {
        let disp = camera - self.centre;
        let disc = (disp * ray).powf(2.) - disp.sq() * ray.sq() + ray.sq() * self.radius.powf(2.);
        if disc >= 0. {
            let t = ((disp * ray) - disc.sqrt()) / ray.sq();
            Some(camera + t * ray)
        } else {
            None
        }
    }
}
