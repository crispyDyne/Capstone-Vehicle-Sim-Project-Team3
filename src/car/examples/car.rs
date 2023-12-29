use bevy::prelude::*;
use bevy_ggrs::*;
use bevy_matchbox::prelude::*;

// Some of the following code adapted from example code: https://github.com/johanhelsing/matchbox/tree/main/examples/bevy_ggrs

use bevy_integrator::{SimTime, Solver};
use car::{
    build::{create_player, build_car, car_startup_system},
    environment::build_environment,
    setup::{camera_setup, simulation_setup},
};
use rigid_body::plugin::RigidBodyPlugin;

// Main function
fn main() {
    let player1 = create_player();
    // Create App
    App::new()
        .add_plugins(RigidBodyPlugin {
            time: SimTime::new(0.002, 0.0, None),
            solver: Solver::RK4,
            simulation_setup: vec![simulation_setup],
            environment_setup: vec![camera_setup],
            name: "car_demo".to_string(),
        })
        .insert_resource(player1.car_definition)
        .add_systems(Startup, car_startup_system)
        .add_systems(Startup, build_environment)
        .add_systems(Startup, start_matchbox_socket) // Add create_player here later
        .run();
}

fn start_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://127.0.0.1:3536/vehicle_sim"; // Port 3536
    info!("connecting to matchbox server: {room_url}");
    commands.insert_resource(MatchboxSocket::new_ggrs(room_url));
}