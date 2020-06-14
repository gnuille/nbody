pub mod controller;
pub mod math;
pub mod body;
pub mod io;

pub fn init(){
    
    println!("--| Starting simulation");

    let config = io::parse_args();

    println!("--| Parsing configuration");
    println!("--|    Input file at: {}", config.input_file);

    let mut sim_setup = io::parse_input_file(config.input_file);

    while { 
        for i in 0..sim_setup.nbody {
            sim_setup.bodies[i as usize].update(sim_setup.timmer.ts);
        }

        sim_setup.timmer.increment() 
    }{}
}

pub fn exit( msg: &str, code: i32) {

    println!("{}", msg);
    std::process::exit(code);

}
