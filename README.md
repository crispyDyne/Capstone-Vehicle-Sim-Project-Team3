# Capstone-Vehicle-Sim-Project
## Repository for CS 46X

## Primary means of team communication: Discord
## Current weekly schedule for team mettings:
Sunday     - 
Monday     -
Tuesday    -
Wednesday  -
Thursday   - Project Partner @6:15pm (Discord), TA @6:45pm (Teams) 
Friday     - 
Saturday   - 


## Project Structure:

.
├── docs/
├── src/
├── wip/
├── .gitignore
├── LICENSE
└── README.md

The docs/ directory contains our supporting team documents for our team deliverables and assignments.

The src/ directory contains our codebase for the project.

The wip/ directory is our Work in Progress directory. It contains assignment templates that we will modify/update during the year (e.g., diagrams, design files, documents, reviews, progress reports, etc.)

**The LICENSE is currently CC0 and will need to be changed by the end of the year!



# Project Dependency:
This project is based on our Project Partner's original vehicle demo.
The original porject repo can be found here: https://github.com/crispyDyne/bevy_car_demo/tree/main



# Bevy Car Demo
This repo contains a simple car demo using rust and bevy. Consider it a proof of concept for a physics engine in bevy/rust, but there are still a lot of features missing, and likely some bugs. There are glitches that can cause the car to explode, especially when driving aggressively over rough terrain.

It includes several simple rigid body demos, which can be run with:
```bash
cargo run --example <example_name>
```
The examples are:
- `car`: simple car demo
- `00_1dof`: A single rigid body with a single translational degree of freedom and a spring force
- `01_pendulum`: A pendulum with a revolute joint
- `02_double_pendulum`: A double pendulum with two revolute joints

## Car Controls
Keyboard controls for the car demo:
- `W`/`S`: Accelerate/brake
- `A`/`D`: Steer left/right

Gamepad controls for the car demo:
- `Right Stick`: Accelerate/brake
- `Left Stick`: Steer
- `Right Trigger`: Accelerate
- `Left Trigger`: Brake

## Crates
- `car`: car demo
    - Demonstrates a simple car with suspension, engine, brakes, and steering.
    - Tires are modeled as a cylinder of points, each of which can interact with the terrain with a simple friction model.
- `rigid_body`: rigid body dynamics library
    - based on [Rigid Body Dynamics Algorithms](https://link.springer.com/book/10.1007/978-1-4899-7560-7) by Roy Featherstone
    - uses the `nalgebra` crate for linear algebra
    - Revolute and prismatic joints are supported
- `integrator`: numerical integrators for rigid body dynamics
    - uses a `FixedTime` schedule to integrate the rigid bodies independently of the bevy update and rendering loops.
    - Several numerical integrators are available, including forward Euler (`Euler`), `Midpoint`, `Heun`, and fourth order Runge-Kutta (`RK4`). 
- `grid_terrain`: used to generate terrain meshes that the car can drive on. 
    - a rectangular grid of terrain elements (ramp, step, function, etc.) is use to specify the terrain. 
- `cameras`: basic camera controls for bevy
