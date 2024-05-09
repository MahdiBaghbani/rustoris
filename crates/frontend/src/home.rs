use eframe::egui;
use egui::Frame;
use egui_dock::{DockArea, DockState, NodeIndex, Style, SurfaceIndex};

use crate::command::joints::JointState;
use crate::gamepad::control_panel::GamepadControlPanel;
use crate::wasm::info_panel::WasmInfoPanel;

struct TabViewer<'a> {
    added_nodes: &'a mut Vec<(SurfaceIndex, NodeIndex)>,
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = usize;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        format!("Tab {tab}").into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        ui.label(format!("Content of tab {tab}"));
    }

    fn on_add(&mut self, surface: SurfaceIndex, node: NodeIndex) {
        self.added_nodes.push((surface, node));
    }
}

struct Docks {
    tree: DockState<usize>,
    counter: usize,
}

impl Default for Docks {
    fn default() -> Self {
        let mut tree: DockState<usize> = DockState::new(vec![1]);

        // You can modify the tree before constructing the dock
        let [a, b] = tree
            .main_surface_mut()
            .split_left(NodeIndex::root(), 0.5, vec![2]);
        let [_, _] = tree.main_surface_mut().split_below(a, 0.5, vec![3]);
        let [_, _] = tree.main_surface_mut().split_below(b, 0.5, vec![4]);

        Self { tree, counter: 4 }
    }
}

/// The state that we persist (serialize).
#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct State {
    docks: Docks,
    gamepad_control_panel: GamepadControlPanel,
    wasm_info_panel: WasmInfoPanel,
    joints: JointState,
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

    fn gamepad_control_panel_contents(
        &mut self,
        ui: &mut egui::Ui,
    ) {
        self.state.gamepad_control_panel.ui(ui);
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
        self.state.gamepad_control_panel.update(&mut self.state.joints);
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
            });

        egui::SidePanel::right("side_panel_right")
            .resizable(false)
            .min_width(300f32)
            .max_width(300f32)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("🎮 Gamepad Control Panel");
                });

                ui.separator();

                self.gamepad_control_panel_contents(ui);
            });

        egui::CentralPanel::default()
            .frame(
                Frame::central_panel(&ctx.style()).inner_margin(0.)
            )
            .show(
                ctx, |ui| {
                    let mut added_nodes = Vec::new();
                    DockArea::new(&mut self.state.docks.tree)
                        .draggable_tabs(false)
                        .show_add_popup(false)
                        .show_add_buttons(false)
                        .show_close_buttons(false)
                        .style({
                            let mut style = Style::from_egui(ctx.style().as_ref());
                            style.tab_bar.fill_tab_bar = true;
                            style
                        })
                        .show_inside(
                            ui,
                            &mut TabViewer {
                                added_nodes: &mut added_nodes,
                            },
                        );

                    added_nodes.drain(..).for_each(|(surface, node)| {
                        self.state.docks.tree.set_focused_node_and_surface((surface, node));
                        self.state.docks.tree.push_to_focused_leaf(self.state.docks.counter);
                        self.state.docks.counter += 1;
                    });
                });

        ctx.request_repaint();
    }
}
