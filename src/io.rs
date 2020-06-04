use std::env;
use std::string::String;
use crate::controller::{Config, SimSetup};
use na::{Vector3, Point3};

pub fn parse_args() -> Config {

    let arguments: Vec<String> = env::args().collect();

    if arguments.len() < 2 {
        panic!("Input file not provided");
    }

    Config {
        input_file: arguments[1].to_string()
    }
}

pub fn parse_input_file(inp: String) {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let reader = BufReader::new(File::open(inp).unwrap());

    let mut ret = SimSetup {
        nbody: 0,
        bodies: Vec::new()
    }

    for line in reader.lines() {
        let line = line.unwrap();
        let words = line.split(" ");

        let word = words.next();
        if word.len() != 0 && word.chars().next().unwrap() != '#' {
            match word {
                ".N" => {
                    ret.nbody = words.next().parse::<u64>();
                },
                ".B" => {
                    let mut b = Body {
                        m: 0.0,
                        pos: Point3::<f64>::new(0.0, 0.0, 0.0),
                        vel: Vector3::<f64>::new(0.0, 0.0, 0.0),
                        acc: Vector3::<f64>::new(0.0, 0.0, 0.0),
                    }
                    for spec in words {
                        match spec {
                            ".M" => {
                                b.m = spec.next().parse::<f64>();
                            },
                            ".X" => {
                                b.pos.x = spec.next().parse::<f64>();
                            },
                            ".Y" => {
                                b.pos.y = spec.next().parse::<f64>();
                            },
                            ".Z" => {
                                b.pos.z = spec.next().parse::<f64>();
                            },
                            ".VX" => {
                                b.vel.x = spec.next().parse::<f64>();
                            },
                            ".VY" => {
                                b.vel.y = spec.next().parse::<f64>();
                            },
                            ".VZ" => {
                                b.vel.z = spec.next().parse::<f64>();
                            },
                            ".AX" => {
                                b.acc.x = spec.next().parse::<f64>();
                            },
                            ".AY" => {
                                b.acc.y = spec.next().parse::<f64>();
                            },
                            ".AZ" => {
                                b.acc.z = spec.next().parse::<f64>();
                            },
                        }
                    }
                    ret.bodies.push(b);
                },
            }
        }
    }
}

