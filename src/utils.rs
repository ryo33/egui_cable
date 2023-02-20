use egui::{style::WidgetVisuals, vec2, Response, Ui, Vec2};

pub fn widget_visuals(ui: &mut Ui, response: &Response) -> WidgetVisuals {
    if response.hovered() {
        return ui.visuals().widgets.hovered;
    };
    if response.dragged() {
        return ui.visuals().widgets.active;
    }
    ui.visuals().widgets.inactive
}

// should be even number because SIZE / 2.0 produces .5 and it may break rendering.
pub const SIZE: Vec2 = vec2(16.0, 16.0);
