use crate::body::Body;

pub struct Config {
    pub input_file: String,
}

pub struct SimSetup {
    pub nbody: u64,
    pub bodies: Vec<Body>,
    pub timmer: Timmer,
}

pub struct Timmer {
    pub to: f64,
    pub ts: f64,
    pub tf: f64,
    pub tc: f64,
}

impl Timmer {
    pub fn increment(&mut self) -> bool {
        self.tc += self.ts;
        return self.tc >= self.tf;
    }
}
