use std::time::UNIX_EPOCH;

use eframe::egui;
use eframe::egui::Vec2;
use egui::RichText;
use egui_plot::{MarkerShape, PlotPoints, Points};
use gilrs::{Axis, GamepadId, Gilrs, GilrsBuilder};
use gilrs::ev::AxisOrBtn;
use gilrs_core::PowerInfo;
use uuid::Uuid;

pub struct MyEguiApp {
    gilrs: Gilrs,
    current_gamepad: Option<GamepadId>,
    log_messages: [Option<String>; 300],
}

impl Default for MyEguiApp {
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

impl MyEguiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn log(&mut self, message: String) {
        self.log_messages[0..].rotate_right(1);
        self.log_messages[0] = Some(message);
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Controllers");
            ui.separator();

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
            ui.allocate_space(ui.available_size());
        });

        egui::TopBottomPanel::bottom("log")
            .resizable(true)
            .default_height(200.0)
            .show(ctx, |ui| {
                ui.heading("Event Log");
                egui::ScrollArea::vertical()
                    .max_height(ui.available_height())
                    .show(ui, |ui| {
                        for message in self.log_messages.iter().flatten() {
                            ui.label(message);
                        }
                        ui.allocate_space(ui.available_size());
                    });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                if let Some(gamepad_id) = self.current_gamepad {
                    let gamepad = self.gilrs.gamepad(gamepad_id);
                    let gamepad_state = gamepad.state();
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.heading("Info");
                            egui::Grid::new("info_grid")
                                .striped(true)
                                .num_columns(2)
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

                                    ui.label("Uuid");
                                    let uuid = Uuid::from_bytes(gamepad.uuid()).to_string();
                                    ui.horizontal(|ui| {
                                        ui.label(&uuid);
                                        if ui.button("Copy").clicked() {
                                            ui.output_mut(|o| o.copied_text = uuid);
                                        }
                                    });
                                    ui.end_row();

                                    ui.label("Power");
                                    ui.label(match gamepad.power_info() {
                                        PowerInfo::Unknown => "Unknown".to_string(),
                                        PowerInfo::Wired => "Wired".to_string(),
                                        PowerInfo::Discharging(p) => format!("Discharging {p}"),
                                        PowerInfo::Charging(p) => format!("Charging {p}"),
                                        PowerInfo::Charged => "Charged".to_string(),
                                    });
                                    ui.end_row();
                                });
                        });
                    });
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.set_width(300.0);
                            ui.heading("Buttons");

                            for (code, button_data) in gamepad_state.buttons() {
                                let name = match gamepad.axis_or_btn_name(code) {
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
                        ui.vertical(|ui| {
                            ui.set_width(300.0);
                            ui.heading("Axes");
                            ui.horizontal(|ui| {
                                for (name, x, y) in [
                                    ("Left Stick", Axis::LeftStickX, Axis::LeftStickY),
                                    ("Right Stick", Axis::RightStickX, Axis::RightStickY),
                                ] {
                                    ui.vertical(|ui| {
                                        ui.label(name);
                                        let y_axis = gamepad
                                            .axis_data(y)
                                            .map(|a| a.value())
                                            .unwrap_or_default()
                                            as f64;
                                        let x_axis = gamepad
                                            .axis_data(x)
                                            .map(|a| a.value())
                                            .unwrap_or_default()
                                            as f64;
                                        egui_plot::Plot::new(format!("{name}_plot"))
                                            .width(150.0)
                                            .height(150.0)
                                            .min_size(Vec2::splat(3.25))
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
                                let name = match gamepad.axis_or_btn_name(code) {
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
                    });
                } else {
                    ui.label("Press a button on a controller or select it from the left.");
                }
                ui.allocate_space(ui.available_size());
            });
        });

        ctx.request_repaint();
    }
}
