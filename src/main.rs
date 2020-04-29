use birthday_paradox::*;
use std::env;
#[allow(unused_variables)]
fn main() {
    let arguments: Vec<String> = env::args().collect();
    let simulation_result = Simulation::new(arguments);
    match simulation_result {
        Ok(mut simulation) => {
            simulation.start();
        }
        Err(msg) => println!("{}", msg),
    };
}
