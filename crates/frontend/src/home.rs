use eframe::egui;

use crate::gamepad_panel::GamepadPanel;
use crate::wasm_info_panel::WasmInfoPanel;

/// The state that we persist (serialize).
#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct State {
    gamepad_panel: GamepadPanel,
    wasm_info_panel: WasmInfoPanel,
}

#[derive(Default)]
pub struct HomePage {
    state: State,
}

impl HomePage {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn bar_contents(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::widgets::global_dark_light_mode_switch(ui);

        ui.separator();
    }

    fn gamepad_panel_contents(
        &mut self,
        ui: &mut egui::Ui,
    ) {
        self.state.gamepad_panel.ui(ui);
    }

    fn wasm32_info_panel_contents(
        &mut self,
        ui: &mut egui::Ui,
        frame: &mut eframe::Frame,
    ) {
        self.state.wasm_info_panel.ui(ui, frame);
    }
}

impl eframe::App for HomePage {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.state.gamepad_panel.update();
        self.state.wasm_info_panel.update(ctx, frame);

        egui::TopBottomPanel::top("top_p").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.visuals_mut().button_frame = false;
                self.bar_contents(ui, frame);
            });
        });

        egui::TopBottomPanel::bottom("log")
            .show(ctx, |ui| {
                ui.heading("Event Log");
            });

        egui::SidePanel::left("side_panel_left")
            .resizable(false)
            .min_width(300f32)
            .max_width(300f32)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("💻 WebAssembly Info Panel");
                });

                ui.separator();

                self.wasm32_info_panel_contents(ui, frame);

                ui.separator();

                ui.vertical_centered(|ui| {
                    ui.heading("🎮 Gamepad Panel");
                });

                ui.separator();

                self.gamepad_panel_contents(ui);
            });

        egui::SidePanel::right("side_panel_right")
            .resizable(false)
            .min_width(100f32)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("🎮 Controllers");
                });
            });


        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.heading("Vertical 1");
                    });
                    ui.vertical(|ui| {
                        ui.heading("Vertical 2");
                    });
                });
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.heading("Vertical 3");
                    });
                    ui.vertical(|ui| {
                        ui.heading("Vertical 4");
                    });
                });
                ui.allocate_space(ui.available_size());
            });
        });

        ctx.request_repaint();
    }
}
