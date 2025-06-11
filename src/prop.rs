use crate::pixel::Rgb;
use crate::vector::{Ray, Vector};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub colour: Rgb,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

#[derive(Clone, Debug)]
pub struct HitRecord<'a> {
    pub prop: &'a dyn Prop,
    pub ray: Ray,
    pub distance: f64,
    pub position: Vector,
    pub normal: Vector,
    pub material: Material,
}

pub trait Prop: 'static + std::fmt::Debug {
    fn raycast(&self, ray: Ray, eps: f64) -> Option<HitRecord>;
}

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub centre: Vector,
    pub radius: f64,
    pub material: Material,
}

impl Prop for Sphere {
    fn raycast(&self, ray: Ray, eps: f64) -> Option<HitRecord> {
        #[inline]
        const fn sq(f: f64) -> f64 {
            f * f
        }

        let disp = ray.eye - self.centre;
        let d4 = sq(disp * ray.dir) - disp.sq() * ray.dir.sq() + ray.dir.sq() * sq(self.radius);
        if d4 >= 0. {
            let t1 = (-disp * ray.dir - d4.sqrt()) / ray.dir.sq();
            let t2 = (-disp * ray.dir + d4.sqrt()) / ray.dir.sq();
            if t1 >= eps {
                Some(HitRecord {
                    prop: self,
                    ray,
                    distance: ray.distance(t1),
                    position: ray.at(t1),
                    normal: (ray.at(t1) - self.centre) / self.radius,
                    material: self.material,
                })
            } else if t2 >= eps {
                Some(HitRecord {
                    prop: self,
                    ray,
                    distance: ray.distance(t2),
                    position: ray.at(t2),
                    normal: (ray.at(t2) - self.centre) / self.radius,
                    material: self.material,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}
