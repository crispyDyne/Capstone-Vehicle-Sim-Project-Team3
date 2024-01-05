#![allow(unused_imports)]

use bevy::prelude::*;
use bevy_ggrs::*;
use bevy_matchbox::prelude::*;

// Some of the following code adapted from example code: https://github.com/johanhelsing/matchbox/tree/main/examples/bevy_ggrs

use bevy_integrator::{SimTime, Solver};
use car::{
    build::{build_car, car_startup_system},
    environment::build_environment,
    setup::{camera_setup, simulation_setup},
};
use rigid_body::plugin::RigidBodyPlugin;

// Main function
fn main() {
    let car_definition = build_car();
    // Create App
    App::new()
        .add_plugins(RigidBodyPlugin {
            time: SimTime::new(0.002, 0.0, None),
            solver: Solver::RK4,
            simulation_setup: vec![simulation_setup],
            environment_setup: vec![camera_setup],
            name: "car_demo".to_string(),
        })
        .insert_resource(car_definition)
        .add_systems(Startup, car_startup_system)
        .add_systems(Startup, build_environment)
        //.add_systems(Startup, start_matchbox_socket) // Add create_player here later
        .run();
}