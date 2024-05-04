use std::time::UNIX_EPOCH;

use eframe::emath::Vec2;
use egui::RichText;
use egui_plot::{MarkerShape, PlotPoints, Points};
use gilrs::{
    Axis,
    ev::{
        AxisOrBtn,
        state::GamepadState,
    },
    Gamepad,
    GamepadId,
    Gilrs,
    GilrsBuilder,
};

pub struct GamepadControlPanel {
    gilrs: Gilrs,
    current_gamepad: Option<GamepadId>,
    log_messages: [Option<String>; 300],
}

impl Default for GamepadControlPanel {
    fn default() -> Self {
        const INIT: Option<String> = None;
        let gilrs: Gilrs = GilrsBuilder::new().set_update_state(false).build().unwrap();
        Self {
            gilrs,
            current_gamepad: None,
            log_messages: [INIT; 300],
        }
    }
}

impl GamepadControlPanel {
    pub fn update(&mut self) {
        while let Some(event) = self.gilrs.next_event() {
            self.log(format!(
                "{} : {} : {:?}",
                event
                    .time
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis(),
                event.id,
                event.event
            ));
            self.gilrs.update(&event);
            if self.current_gamepad.is_none() {
                self.current_gamepad = Some(event.id);
            }
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        self.gamepad_list_ui(ui);

        ui.separator();

        self.current_gamepad_details_ui(ui);

        ui.separator();

        gamepad_logs(ui, &self.log_messages);
    }

    fn log(&mut self, message: String) {
        self.log_messages[0..].rotate_right(1);
        self.log_messages[0] = Some(message);
    }

    fn gamepad_list_ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("📃 List");
        });

        ui.separator();

        egui::ScrollArea::vertical()
            .max_height(150f32)
            .id_source("gamepad_list_ui")
            .show(ui, |ui| {
                for (id, gamepad) in self.gilrs.gamepads() {
                    if ui
                        .selectable_label(
                            self.current_gamepad == Some(id),
                            format!("{id}: {}", gamepad.name()),
                        )
                        .clicked()
                    {
                        self.current_gamepad = Some(id);
                    };
                }
            });
    }

    fn current_gamepad_details_ui(&mut self, ui: &mut egui::Ui) {
        if let Some(gamepad_id) = self.current_gamepad {
            let gamepad: Gamepad = self.gilrs.gamepad(gamepad_id);
            let gamepad_state: &GamepadState = gamepad.state();

            gamepad_sticks_plotter(ui, &gamepad, gamepad_state);

            ui.separator();

            gamepad_buttons(ui, &gamepad, gamepad_state);

            ui.separator();

            gamepad_info(ui, &gamepad);
        } else {
            ui.label("Press a button on a controller or select it from the left.");
        }
    }
}

fn gamepad_sticks_plotter(ui: &mut egui::Ui, gamepad: &Gamepad, gamepad_state: &GamepadState) {
    ui.vertical(|ui| {
        ui.vertical_centered(|ui| {
            ui.heading("🕹 Sticks Axes");
        });

        ui.separator();

        ui.horizontal(|ui| {
            for (name, x, y) in [
                ("Left Stick", Axis::LeftStickX, Axis::LeftStickY),
                ("Right Stick", Axis::RightStickX, Axis::RightStickY),
            ] {
                ui.vertical(|ui| {
                    ui.label(name);
                    let y_axis: f64 = gamepad
                        .axis_data(y)
                        .map(|a| a.value())
                        .unwrap_or_default()
                        as f64;
                    let x_axis: f64 = gamepad
                        .axis_data(x)
                        .map(|a| a.value())
                        .unwrap_or_default()
                        as f64;
                    egui_plot::Plot::new(format!("{name}_plot"))
                        .width(125.0)
                        .view_aspect(1f32)
                        .min_size(Vec2::splat(3f32))
                        .include_x(1.25)
                        .include_y(1.25)
                        .include_x(-1.25)
                        .include_y(-1.25)
                        .allow_drag(false)
                        .allow_zoom(false)
                        .allow_boxed_zoom(false)
                        .allow_scroll(false)
                        .show(ui, |plot_ui| {
                            plot_ui.points(
                                Points::new(PlotPoints::new(vec![[
                                    x_axis, y_axis,
                                ]]))
                                    .shape(MarkerShape::Circle)
                                    .radius(4.0),
                            );
                        });
                });
            }
        });
        for (code, axis_data) in gamepad_state.axes() {
            let name: String = match gamepad.axis_or_btn_name(code) {
                None => code.to_string(),
                Some(AxisOrBtn::Btn(b)) => format!("{b:?}"),
                Some(AxisOrBtn::Axis(a)) => format!("{a:?}"),
            };
            ui.add(
                egui::widgets::ProgressBar::new(
                    (axis_data.value() * 0.5) + 0.5,
                )
                    .text(
                        RichText::new(format!(
                            "{:+.4} {name:<15} {}",
                            axis_data.value(),
                            code
                        ))
                            .monospace(),
                    ),
            );
        }
    });
}

fn gamepad_buttons(ui: &mut egui::Ui, gamepad: &Gamepad, gamepad_state: &GamepadState) {
    ui.vertical(|ui| {
        ui.vertical_centered(|ui| {
            ui.heading("🔘 Buttons");
        });

        ui.separator();

        for (code, button_data) in gamepad_state.buttons() {
            let name: String = match gamepad.axis_or_btn_name(code) {
                Some(AxisOrBtn::Btn(b)) => format!("{b:?}"),
                _ => "Unknown".to_string(),
            };

            ui.add(
                egui::widgets::ProgressBar::new(button_data.value()).text(
                    RichText::new(format!(
                        "{name:<14} {:<5} {:.4} {}",
                        button_data.is_pressed(),
                        button_data.value(),
                        code
                    ))
                        .monospace(),
                ),
            );
        }
    });
}

fn gamepad_info(ui: &mut egui::Ui, gamepad: &Gamepad) {
    ui.vertical_centered(|ui| {
        ui.heading("ℹ Info");
    });

    ui.separator();

    egui::Grid::new("gamepad_info_grid")
        .striped(true)
        .num_columns(2)
        .max_col_width(250f32)
        .show(ui, |ui| {
            ui.label("Name");
            ui.label(gamepad.name());
            ui.end_row();

            if let Some(vendor) = gamepad.vendor_id() {
                ui.label("Vendor ID");
                ui.label(format!("{vendor:04x}"));
                ui.end_row();
            }

            if let Some(product) = gamepad.product_id() {
                ui.label("Product ID");
                ui.label(format!("{product:04x}"));
                ui.end_row();
            }

            ui.label("Gilrs ID");
            ui.label(gamepad.id().to_string());
            ui.end_row();

            if let Some(map_name) = gamepad.map_name() {
                ui.label("Map Name");
                ui.label(map_name);
                ui.end_row();
            }

            ui.label("Map Source");
            ui.label(format!("{:?}", gamepad.mapping_source()));
            ui.end_row();
        });
}

fn gamepad_logs(ui: &mut egui::Ui, messages: &[Option<String>; 300]) {
    ui.vertical_centered(|ui| {
        ui.heading("📝 Logs");
    });

    ui.separator();

    egui::ScrollArea::vertical()
        .max_height(ui.available_height())
        .show(ui, |ui| {
            for message in messages.iter().flatten() {
                ui.label(message);
            }
            ui.allocate_space(ui.available_size());
        });
}
