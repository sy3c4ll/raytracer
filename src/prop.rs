use crate::pixel::Rgb;
use crate::vector::Vector;

#[derive(Clone, Debug)]
pub struct HitRecord<'a> {
    pub prop: &'a dyn Prop,
    pub distance: f64,
    pub position: Vector,
    pub normal: Vector,
    pub colour: Rgb,
}

pub trait Prop: 'static + std::fmt::Debug {
    fn raycast(&self, eye: Vector, ray: Vector, eps: f64) -> Option<HitRecord>;
}

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub centre: Vector,
    pub radius: f64,
    pub colour: Rgb,
}

impl Prop for Sphere {
    fn raycast(&self, eye: Vector, ray: Vector, eps: f64) -> Option<HitRecord> {
        #[inline]
        const fn sq(f: f64) -> f64 {
            f * f
        }

        let disp = eye - self.centre;
        let d4 = sq(disp * ray) - disp.sq() * ray.sq() + ray.sq() * sq(self.radius);
        if d4 >= 0. {
            let t1 = (-disp * ray - d4.sqrt()) / ray.sq();
            let t2 = (-disp * ray + d4.sqrt()) / ray.sq();
            if t1 >= eps {
                Some(HitRecord {
                    prop: self,
                    distance: t1 * ray.abs(),
                    position: eye + t1 * ray,
                    normal: eye + t1 * ray - self.centre,
                    colour: self.colour,
                })
            } else if t2 >= eps {
                Some(HitRecord {
                    prop: self,
                    distance: t2 * ray.abs(),
                    position: eye + t2 * ray,
                    normal: eye + t2 * ray - self.centre,
                    colour: self.colour,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}
