use egui::emath::RectTransform;
use egui::style::WidgetVisuals;
use egui::util::History;

pub(super) struct FrameHistory {
    frame_times: History<f32>,
}

impl Default for FrameHistory {
    fn default() -> Self {
        let max_age: f32 = 1.0;
        let max_len: usize = (max_age * 300.0).round() as usize;
        Self {
            frame_times: History::new(0..max_len, max_age),
        }
    }
}

impl FrameHistory {
    // Called first.
    pub(super) fn on_new_frame(&mut self, now: f64, previous_frame_time: Option<f32>) {
        let previous_frame_time: f32 = previous_frame_time.unwrap_or_default();
        if let Some(latest) = self.frame_times.latest_mut() {
            // rewrite history now that we know.
            *latest = previous_frame_time;
        }
        // projected.
        self.frame_times.add(now, previous_frame_time);
    }

    pub(super) fn mean_frame_time(&self) -> f32 {
        self.frame_times.average().unwrap_or_default()
    }

    pub(super) fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label(
            format!(
                "Mean CPU usage: {:.2} ms / frame",
                1e3 * self.mean_frame_time()
            )
        )
            .on_hover_text(
                "Includes all app logic, egui layout, tessellation, and rendering.\n\
                Does not include waiting for vsync.",
            );

        egui::warn_if_debug_build(ui);
    }
}
