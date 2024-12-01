use crate::image::vector::Vector;

#[derive(Clone)]
pub struct ONB {
    axis: Vec<Vector>,
}

impl ONB {
    pub fn new(n: Vector) -> ONB {
        let mut axis = vec![Vector::zero(); 3];
        axis[2] = n.unit_vector();
        let a = if axis[2].x.abs() > 0.9 {
            Vector::new(0.0, 1.0, 0.0)
        } else {
            Vector::new(1.0, 0.0, 0.0)
        };
        axis[1] = axis[2].cross(a).unit_vector();
        axis[0] = axis[2].cross(axis[1]);
        Self { axis }
    }

    pub fn u(&self) -> Vector {
        self.axis[0]
    }

    pub fn v(&self) -> Vector {
        self.axis[1]
    }

    pub fn w(&self) -> Vector {
        self.axis[2]
    }

    pub fn transform(&self, v: Vector) -> Vector {
        v.x * self.axis[0] + v.y * self.axis[1] + v.z * self.axis[2]
    }
}
