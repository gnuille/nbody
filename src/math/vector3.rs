
#[derive(Copy, Clone)]
pub struct Vector3 {
   pub x: f64,
   pub y: f64,
   pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn update(&mut self,x: f64, y: f64, z: f64){
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn dot(&self, v: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z
        }
    }

    pub fn dotk(&self, k: f64) -> Vector3 {
        Vector3 {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }

    pub fn add(&self, v: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}
