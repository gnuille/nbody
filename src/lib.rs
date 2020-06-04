pub mod controller;
pub mod body;
pub mod io;

extern crate nalgebra as na;

pub fn init(){
    
    println!("--| Starting simulation");

    let config = io::parse_args();

    println!("--| Parsing configuration");
    println!("--|    Input file at: {}", config.input_file);

    let sim_setup = io::parse_input_file(config.input_file);
}
