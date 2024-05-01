use egui::emath::RectTransform;
use egui::style::WidgetVisuals;
use egui::util::History;

pub struct FrameHistory {
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
    // Called first
    pub fn on_new_frame(&mut self, now: f64, previous_frame_time: Option<f32>) {
        let previous_frame_time: f32 = previous_frame_time.unwrap_or_default();
        if let Some(latest) = self.frame_times.latest_mut() {
            // rewrite history now that we know.
            *latest = previous_frame_time;
        }
        // projected.
        self.frame_times.add(now, previous_frame_time);
    }

    pub fn mean_frame_time(&self) -> f32 {
        self.frame_times.average().unwrap_or_default()
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label(format!(
            "Mean CPU usage: {:.2} ms / frame",
            1e3 * self.mean_frame_time()
        ))
            .on_hover_text(
                "Includes all app logic, egui layout, tessellation, and rendering.\n\
            Does not include waiting for vsync.",
            );
        egui::warn_if_debug_build(ui);
        
        egui::CollapsingHeader::new("📊 CPU usage history")
            .default_open(false)
            .show(ui, |ui| {
                self.graph(ui);
            });
    }

    fn graph(&mut self, ui: &mut egui::Ui) -> egui::Response {
        use egui::*;

        ui.label("egui CPU usage history");

        let history: &History<f32> = &self.frame_times;

        // TODO(emilk): we should not use `slider_width` as default graph width.
        let height: f32 = ui.spacing().slider_width;
        let size: Vec2 = vec2(ui.available_size_before_wrap().x, height);
        let (rect, response) = ui.allocate_at_least(size, Sense::hover());
        let style: &WidgetVisuals = ui.style().noninteractive();

        let graph_top_cpu_usage: f32 = 0.010;
        let graph_rect: Rect = Rect::from_x_y_ranges(history.max_age()..=0.0, graph_top_cpu_usage..=0.0);
        let to_screen: RectTransform = RectTransform::from_to(graph_rect, rect);

        let mut shapes: Vec<Shape> = Vec::with_capacity(3 + 2 * history.len());
        shapes.push(Shape::Rect(epaint::RectShape::new(
            rect,
            style.rounding,
            ui.visuals().extreme_bg_color,
            ui.style().noninteractive().bg_stroke,
        )));

        let rect: Rect = rect.shrink(4.0);
        let color: Color32 = ui.visuals().text_color();
        let line_stroke: Stroke = Stroke::new(1.0, color);

        if let Some(pointer_pos) = response.hover_pos() {
            let y: f32 = pointer_pos.y;
            shapes.push(Shape::line_segment(
                [pos2(rect.left(), y), pos2(rect.right(), y)],
                line_stroke,
            ));
            let cpu_usage: f32 = to_screen.inverse().transform_pos(pointer_pos).y;
            let text: String = format!("{:.1} ms", 1e3 * cpu_usage);
            shapes.push(ui.fonts(|f| {
                Shape::text(
                    f,
                    pos2(rect.left(), y),
                    Align2::LEFT_BOTTOM,
                    text,
                    TextStyle::Monospace.resolve(ui.style()),
                    color,
                )
            }));
        }

        let circle_color: Color32 = color;
        let radius: f32 = 2.0;
        let right_side_time: f64 = ui.input(|i| i.time); // Time at right side of screen

        for (time, cpu_usage) in history.iter() {
            let age: f32 = (right_side_time - time) as f32;
            let pos: Pos2 = to_screen.transform_pos_clamped(Pos2::new(age, cpu_usage));

            shapes.push(Shape::line_segment(
                [pos2(pos.x, rect.bottom()), pos],
                line_stroke,
            ));

            if cpu_usage < graph_top_cpu_usage {
                shapes.push(Shape::circle_filled(pos, radius, circle_color));
            }
        }

        ui.painter().extend(shapes);

        response
    }
}
