#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(clippy::never_loop)] // False positive

use eframe::WebOptions;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    env_logger::init();
    let native_options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "Gilrs Input Tester",
        native_options,
        Box::new(|cc| Box::new(frontend::MyEguiApp::new(cc))),
    ).expect("TODO: panic message");
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options: WebOptions = WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "egui_canvas_id",
                web_options,
                Box::new(|cc| Box::new(frontend::MyEguiApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
