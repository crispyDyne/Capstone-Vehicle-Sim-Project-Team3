use bevy::prelude::*;

#[derive(Component, Default, Clone)]
#[component(storage = "SparseSet")]
pub struct CarControl {
    pub throttle: f32,
    pub steering: f32,
    pub brake: f32,
    pub steer_wheels: Vec<Entity>,
    pub brake_wheels: Vec<Entity>,
    pub drive_wheels: Vec<Entity>,
    pub control_type: ControlType,
}

#[derive(Default, Clone)]
pub enum ControlType {
    #[default]
    WASD,
    Arrow,
}

pub fn user_control_system(
    keyboard_input: Res<Input<KeyCode>>,
    gamepads: Res<Gamepads>,
    button_axes: Res<Axis<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    mut controls: Query<&mut CarControl>,
) {
    // Iterate once for each car
    for mut control in controls.iter_mut() {
        // gamepad controls
        for gamepad in gamepads.iter() {
            // trigger controls
            let throttle = button_axes
                .get(GamepadButton::new(
                    gamepad,
                    GamepadButtonType::RightTrigger2,
                ))
                .unwrap();

            if throttle > 0.01 {
                control.throttle = throttle;
            }

            let brake = button_axes
                .get(GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger2))
                .unwrap();

            if brake > 0.01 {
                control.brake = brake;
            }

            // right stick throttle/brake
            let throttle_brake = axes
                .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickY))
                .unwrap();
            if throttle_brake > 0.01 {
                control.throttle = throttle_brake;
            }
            if throttle_brake < -0.01 {
                control.brake = -throttle_brake;
            }

            // left stick steering
            let steering = -axes
                .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
                .unwrap();
            if steering.abs() > 0.01 {
                control.steering = steering;
            }
        }

        // Keyboard controls - these are rate controlled to make them feel more natural.
        // When a key is pressed, the control value is increased at a constant rate.
        // When a key is released, the control value is decreased at a constant rate.
        // The control value is clamped between 0 and 1 for throttle and brake, and
        // between -1 and 1 for steering.
        let response_time = 0.25;
        let time_constant = 1. / (response_time * 60.);

        let mut steer_active = false;

        match control.control_type {
            ControlType::WASD => {
                if keyboard_input.pressed(KeyCode::W) {
                    control.throttle += time_constant;
                    control.throttle = control.throttle.min(1.0);
                } else {
                    control.throttle -= time_constant;
                    control.throttle = control.throttle.max(0.0);
                }

                if keyboard_input.pressed(KeyCode::S) {
                    control.brake += time_constant;
                    control.brake = control.brake.min(1.0);
                } else {
                    control.brake -= time_constant;
                    control.brake = control.brake.max(0.0);
                }

                if keyboard_input.pressed(KeyCode::A) {
                    control.steering += time_constant;
                    control.steering = control.steering.min(1.0);
                    steer_active = true;
                }

                if keyboard_input.pressed(KeyCode::D) {
                    control.steering -= time_constant;
                    control.steering = control.steering.max(-1.0);
                    steer_active = true;
                }
            }
            ControlType::Arrow => {
                if keyboard_input.pressed(KeyCode::Up) {
                    control.throttle += time_constant;
                    control.throttle = control.throttle.min(1.0);
                } else {
                    control.throttle -= time_constant;
                    control.throttle = control.throttle.max(0.0);
                }

                if keyboard_input.pressed(KeyCode::Down) {
                    control.brake += time_constant;
                    control.brake = control.brake.min(1.0);
                } else {
                    control.brake -= time_constant;
                    control.brake = control.brake.max(0.0);
                }

                if keyboard_input.pressed(KeyCode::Left) {
                    control.steering += time_constant;
                    control.steering = control.steering.min(1.0);
                    steer_active = true;
                }

                if keyboard_input.pressed(KeyCode::Right) {
                    control.steering -= time_constant;
                    control.steering = control.steering.max(-1.0);
                    steer_active = true;
                }
            }
        }

        if !steer_active {
            if control.steering.abs() < time_constant {
                control.steering = 0.0;
            } else if control.steering > 0.0 {
                control.steering -= time_constant;
            } else {
                control.steering += time_constant;
            }
        }
    }
}
