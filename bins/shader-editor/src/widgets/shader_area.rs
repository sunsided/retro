pub struct ShaderArea {
    // Optionally, specify desired dimensions here.
    size: egui::Vec2,
}

impl ShaderArea {
    pub(crate) fn new(width: f32, height: f32) -> Self {
        Self {
            size: egui::vec2(width, height),
        }
    }
}

impl egui::Widget for ShaderArea {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        // Allocate an exact space for the widget.
        let (rect, response) = ui.allocate_exact_size(self.size, egui::Sense::hover());

        // Draw a bright magenta rectangle as the placeholder.
        ui.painter()
            .rect_filled(rect, 0.0, egui::Color32::from_rgb(255, 0, 255));
        response
    }
}
