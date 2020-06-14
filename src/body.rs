use crate::math::vector3::Vector3;

pub struct Body {
    pub m: f64,
    pub pos: Vector3,
    pub vel: Vector3,
    pub acc: Vector3,
}

impl Body {
    pub fn init(m: f64, 
                pos: Vector3, 
                vel: Vector3, 
                acc: Vector3) -> Self {
        Body {
            m: m,
            pos: pos,
            vel: vel,
            acc: acc
        }
    }

    pub fn update(&mut self, t: f64){
        self.pos = self.acc.dotk(t*t*0.5).dot(self.acc).add( self.vel.dotk(t) ).add(self.pos);
        self.vel = self.acc.dotk(t).add(self.vel);
    }

    #[cfg(test)]
    pub fn test_part() -> Self {
        Body {
            m: 2.0,
            pos: Vector3::new(2.0, 3.0, 4.0),
            vel: Vector3::new(2.0, 3.0, 4.0),
            acc: Vector3::new(2.0, 3.0, 4.0),
        }
    }
}

