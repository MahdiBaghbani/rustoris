use eframe::wgpu;
use eframe::wgpu::AdapterInfo;

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct WasmInfoPanel {
    #[cfg_attr(feature = "serde", serde(skip))]
    frame_history: crate::frame_history::FrameHistory,
}

impl WasmInfoPanel {
    pub fn update(&mut self, ctx: &egui::Context, frame: &eframe::Frame) {
        self.frame_history
            .on_new_frame(ctx.input(|i| i.time), frame.info().cpu_usage);
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        integration_ui(ui, frame);

        ui.separator();

        self.frame_history.ui(ui);
    }
}

fn integration_ui(ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
    #[cfg(target_arch = "wasm32")]
    ui.collapsing("🌐 Web info (location)", |ui| {
        ui.style_mut().wrap = Some(false);
        ui.monospace(format!("{:#?}", _frame.info().web_info.location));
    });

    if let Some(render_state) = _frame.wgpu_render_state() {
        let wgpu_adapter_details_ui = |ui: &mut egui::Ui, adapter: &wgpu::Adapter| {
            let info: &AdapterInfo = &adapter.get_info();

            let AdapterInfo {
                name,
                vendor,
                device,
                device_type,
                driver,
                driver_info,
                backend,
            } = &info;

            // Example values:
            // > name: "llvmpipe (LLVM 16.0.6, 256 bits)", device_type: Cpu, backend: Vulkan, driver: "llvmpipe", driver_info: "Mesa 23.1.6-arch1.4 (LLVM 16.0.6)"
            // > name: "Apple M1 Pro", device_type: IntegratedGpu, backend: Metal, driver: "", driver_info: ""
            // > name: "ANGLE (Apple, Apple M1 Pro, OpenGL 4.1)", device_type: IntegratedGpu, backend: Gl, driver: "", driver_info: ""

            egui::Grid::new("adapter_info").show(ui, |ui| {
                ui.label("Backend:");
                ui.label(format!("{backend:?}"));
                ui.end_row();

                ui.label("Device Type:");
                ui.label(format!("{device_type:?}"));
                ui.end_row();

                if !name.is_empty() {
                    ui.label("Name:");
                    ui.label(format!("{name:?}"));
                    ui.end_row();
                }
                if !driver.is_empty() {
                    ui.label("Driver:");
                    ui.label(format!("{driver:?}"));
                    ui.end_row();
                }
                if !driver_info.is_empty() {
                    ui.label("Driver info:");
                    ui.label(format!("{driver_info:?}"));
                    ui.end_row();
                }
                if *vendor != 0 {
                    // TODO(emilk): decode using https://github.com/gfx-rs/wgpu/blob/767ac03245ee937d3dc552edc13fe7ab0a860eec/wgpu-hal/src/auxil/mod.rs#L7
                    ui.label("Vendor:");
                    ui.label(format!("0x{vendor:04X}"));
                    ui.end_row();
                }
                if *device != 0 {
                    ui.label("Device:");
                    ui.label(format!("0x{device:02X}"));
                    ui.end_row();
                }
            });
        };

        let wgpu_adapter_ui = |ui: &mut egui::Ui, adapter: &wgpu::Adapter| {
            let info: &AdapterInfo = &adapter.get_info();
            ui.label(format!("{:?}", info.backend)).on_hover_ui(|ui| {
                wgpu_adapter_details_ui(ui, adapter);
            });
        };

        egui::Grid::new("wgpu_info").num_columns(2).show(ui, |ui| {
            ui.label("Renderer:");
            ui.label("wgpu");
            ui.end_row();

            ui.label("Backend:");
            wgpu_adapter_ui(ui, &render_state.adapter);
            ui.end_row();
        });
    }
}