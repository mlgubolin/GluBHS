mod modules;
use modules::grid;
use modules::utils;
extern crate fnv;
use std::{thread, time};
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>{
    let next_grid = grid::create_blank_grid();
    let initial_grid = grid::set_initial_condition(init);

    let mut tf = utils::get_float_in_eniroment("FINAL_TIME");
    let mut time_step = utils::get_float_in_eniroment("TIME_STEP");
    let mut t = 0.;

    while t < tf {
        t += time_step;
    }

    let mut f = File::create("data/foo.txt")?;
    f.write_all(b"Hello, world!")?;

    f.sync_data()?;
    Ok(())
}

//Function for inital conditions
fn init() -> grid::GridUnity {
    let grid = grid::GridUnity {
        density: 1.,
        x1: 0.,
        x2: 0.,
        x3: 0.,
        vx1: 0.,
        vx2: 0.,
        vx3: 0.,
        temperature: 0.,
        pression: 0.,
    };

    return grid;
}

//Function for potential
