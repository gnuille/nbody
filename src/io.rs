use std::env;
use std::string::String;
use crate::controller::{Config, SimSetup, Timmer};
use crate::body::{Body};
use crate::exit;
use na::{Vector3, Point3};
use itertools::Itertools;

pub fn parse_args() -> Config {

    let arguments: Vec<String> = env::args().collect();

    if arguments.len() < 2 {
        exit("--| ERROR: Input set not provided!", -1);
    }

    Config {
        input_file: arguments[1].to_string()
    }
}

pub fn parse_input_file(inp: String) -> SimSetup {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let reader = BufReader::new(File::open(inp).unwrap());

    let mut ret = SimSetup {
        nbody: 0,
        bodies: Vec::new(),
        timmer: Timmer {
            to: 0.0,
            ts: 0.0,
            tf: 0.0,
        },
    };

    for line in reader.lines() {
        let line = line.unwrap();
        let words: Vec<&str> = line.split(" ").collect();

        let first = words[0];
        if first.len() != 0 && first.chars().next().unwrap() == '#' {
            continue;
        }

        match first {
            ".N" => {
                let n = words[1];
                ret.nbody = n.parse::<u64>().unwrap(); 
            },
            ".T" => {
                ret.timmer.to = words[1].parse::<f64>().unwrap();
                ret.timmer.ts = words[2].parse::<f64>().unwrap();
                ret.timmer.tf = words[3].parse::<f64>().unwrap();
                
            }
            ".B" => {
                let mut b = Body {
                    m: 0.0,
                    pos: Point3::<f64>::new(0.0, 0.0, 0.0),
                    vel: Vector3::<f64>::new(0.0, 0.0, 0.0),
                    acc: Vector3::<f64>::new(0.0, 0.0, 0.0),
                };
                let mut words_paired = Vec::new();
                for (a,b) in (words[1..]).iter().tuples() {
                    words_paired.push((a,b)); 
                }

                for spec in words_paired {
                    match *spec.0 {
                        ".B" => {
                            continue; 
                        },
                        ".M" => {
                            b.m = spec.1.parse::<f64>().unwrap();
                        },
                        ".X" => {
                            b.pos.x = spec.1.parse::<f64>().unwrap();
                        },
                        ".Y" => {
                            b.pos.y = spec.1.parse::<f64>().unwrap();
                        },
                        ".Z" => {
                            b.pos.z = spec.1.parse::<f64>().unwrap();
                        },
                        ".VX" => {
                            b.vel.x = spec.1.parse::<f64>().unwrap();
                        },
                        ".VY" => {
                            b.vel.y = spec.1.parse::<f64>().unwrap();
                        },
                        ".VZ" => {
                            b.vel.z = spec.1.parse::<f64>().unwrap();
                        },
                        ".AX" => {
                            b.acc.x = spec.1.parse::<f64>().unwrap();
                        },
                        ".AY" => {
                            b.acc.y = spec.1.parse::<f64>().unwrap();
                        },
                        ".AZ" => {
                            b.acc.z = spec.1.parse::<f64>().unwrap();
                        },
                        _ => {
                            exit("Error parsing input file!", -1);
                        },
                    }
                }
                ret.bodies.push(b);

            }, 
            _ => {
                exit("Error parsing input file!", -1);
            },

        }
    }

    return ret;
}

