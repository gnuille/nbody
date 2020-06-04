use na::{Vector3, Point3};

pub struct Body {
    m: f64,
    pos: Point3<f64>,
    vel: Vector3<f64>,
    acc: Vector3<f64>,
}

impl Body {
    pub fn init(m: f64, 
                pos: Point3<f64>, 
                vel: Vector3<f64>, 
                acc: Vector3<f64>) -> Self {
        Particle {
            m: m,
            pos: pos,
            vel: vel,
            acc: acc
        }
    }

    #[cfg(test)]
    pub fn test_part() -> Self {
        Particle {
            m: 2.0,
            pos: Point3::<f64>::new(2.0, 3.0, 4.0),
            vel: Vector3::<f64>::new(2.0, 3.0, 4.0),
            acc: Vector3::<f64>::new(2.0, 3.0, 4.0),
        }
    }
}

