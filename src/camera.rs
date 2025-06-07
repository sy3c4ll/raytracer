use crate::Vector;

pub struct Camera {
    pub eye: Vector,
    centre: Vector,
    up: Vector,
    right: Vector,
    pub hfov: f64,
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
        }
    }
    pub const fn px_towards_origin(dist: f64, hfov: f64) -> Self {
        const CENTRE: Vector = Vector::I;
        const UP: Vector = Vector::J;
        const RIGHT: Vector = UP.cross(CENTRE);
        Self {
            eye: Vector::new(-dist, 0., 0.),
            hfov,
            up: UP,
            centre: CENTRE,
            right: RIGHT,
        }
    }
    pub const fn py_towards_origin(dist: f64, hfov: f64) -> Self {
        const CENTRE: Vector = Vector::J;
        const UP: Vector = Vector::K.neg();
        const RIGHT: Vector = UP.cross(CENTRE);
        Self {
            eye: Vector::new(0., -dist, 0.),
            centre: CENTRE,
            up: UP,
            right: RIGHT,
            hfov,
        }
    }
    pub const fn pz_towards_origin(dist: f64, hfov: f64) -> Self {
        const CENTRE: Vector = Vector::K;
        const UP: Vector = Vector::J;
        const RIGHT: Vector = UP.cross(CENTRE);
        Self {
            eye: Vector::new(0., 0., -dist),
            centre: CENTRE,
            up: UP,
            right: RIGHT,
            hfov,
        }
    }
    pub const fn nx_towards_origin(dist: f64, hfov: f64) -> Self {
        const CENTRE: Vector = Vector::I.neg();
        const UP: Vector = Vector::J;
        const RIGHT: Vector = UP.cross(CENTRE);
        Self {
            eye: Vector::new(dist, 0., 0.),
            centre: CENTRE,
            up: UP,
            right: RIGHT,
            hfov,
        }
    }
    pub const fn ny_towards_origin(dist: f64, hfov: f64) -> Self {
        const CENTRE: Vector = Vector::J.neg();
        const UP: Vector = Vector::K;
        const RIGHT: Vector = UP.cross(CENTRE);
        Self {
            eye: Vector::new(0., dist, 0.),
            centre: CENTRE,
            up: UP,
            right: RIGHT,
            hfov,
        }
    }
    pub const fn nz_towards_origin(dist: f64, hfov: f64) -> Self {
        const CENTRE: Vector = Vector::K.neg();
        const UP: Vector = Vector::J;
        const RIGHT: Vector = UP.cross(CENTRE);
        Self {
            eye: Vector::new(0., 0., dist),
            centre: CENTRE,
            up: UP,
            right: RIGHT,
            hfov,
        }
    }
}
