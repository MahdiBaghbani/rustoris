use eframe::web_sys::js_sys::Math::{abs, atan2, cos, sin, sqrt};
use gilrs::{Axis, Button, EventType};

#[derive(Default)]
pub(crate) struct JointState {
    // buttons.
    start: f32,
    select: f32,
    // trigger.
    left_trigger_1: f32,
    left_trigger_2: f32,
    right_trigger_1: f32,
    right_trigger_2: f32,
    // axis.
    left_axis_x: f32,
    left_axis_y: f32,
    right_axis_x: f32,
    right_axis_y: f32,
}

impl JointState {
    pub(crate) fn update(&mut self, event_type: EventType) {
        match event_type {
            EventType::ButtonChanged(button, value, ..) => {
                self.update_button(button, value);
            }
            EventType::AxisChanged(axis, value, ..) => {
                self.update_axis(axis, value);
            }
            _ => (),
        }
    }

    fn update_button(&mut self, button: Button, value: f32) {
        match button {
            Button::Start => self.start = value,
            Button::Select => self.select = value,
            Button::LeftTrigger => self.left_trigger_1 = value,
            Button::LeftTrigger2 => self.left_trigger_2 = value,
            Button::RightTrigger => self.right_trigger_1 = value,
            Button::RightTrigger2 => self.right_trigger_2 = value,
            _ => (),
        }
    }

    fn update_axis(&mut self, axis: Axis, value: f32) {
        match axis {
            Axis::LeftStickX => self.left_axis_x = value,
            Axis::LeftStickY => self.left_axis_y = value,
            Axis::RightStickX => self.right_axis_x = value,
            Axis::RightStickY => self.right_axis_y = value,
            _ => (),
        }
    }

    pub(crate) fn axis_to_differential_drive(&mut self) -> (f64, f64) {
        let x: f64 = self.left_axis_x as f64;
        let y: f64 = self.left_axis_y as f64;

        if x == 0f64 && y == 0f64 {
            (0f64, 0f64)
        } else {
            // convert to polar coordinates.
            let theta: f64 = atan2(y, x);
            let radius: f64 = sqrt(x * x + y * y);

            // this is the maximum radius for a given angle.
            let maximum_radius: f64 = if abs(x) > abs(y) {
                abs(radius / (x))
            } else {
                abs(radius / (y))
            };

            // this is the actual throttle.
            let magnitude: f64 = radius / maximum_radius;

            let turn_damping: f64 = 3f64;

            let left: f64 = magnitude * (sin(theta) + cos(theta) / turn_damping);
            let right: f64 = magnitude * (sin(theta) - cos(theta) / turn_damping);

            (left, right)
        }
    }
}

