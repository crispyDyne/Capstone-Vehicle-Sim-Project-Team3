use bevy::prelude::*;

// Some of the following code adapted from example code: https://github.com/johanhelsing/matchbox/tree/main/examples/bevy_ggrs

use bevy_integrator::{SimTime, Solver};
use car::{
    build::{build_car, car_startup_system, CarList},
    control::ControlType,
    environment::build_environment,
    setup::{camera_setup, simulation_setup},
};
use rigid_body::plugin::RigidBodyPlugin;

// Main function
fn main() {
    // Create cars
    let mut car_definitions = Vec::new();
    car_definitions.push(build_car([0., 0., 0.], ControlType::WASD, 0));
    car_definitions.push(build_car([0., 2., 0.], ControlType::Arrow, 1));

    let players = CarList {
        cars: car_definitions,
    };

    // Create App
    App::new()
        .add_plugins(RigidBodyPlugin {
            time: SimTime::new(0.002, 0.0, None),
            solver: Solver::RK4,
            simulation_setup: vec![simulation_setup],
            environment_setup: vec![camera_setup],
            name: "car_demo".to_string(),
        })
        .insert_resource(players)
        .add_systems(Startup, car_startup_system)
        .add_systems(Startup, build_environment)
        .run();
}
