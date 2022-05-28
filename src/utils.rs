use egui::{pos2, style::WidgetVisuals, vec2, Pos2, Response, Ui, Vec2};

pub fn widget_visuals(ui: &mut Ui, response: &Response) -> WidgetVisuals {
    if response.hovered() {
        return ui.visuals().widgets.hovered;
    };
    if response.dragged() {
        return ui.visuals().widgets.active;
    }
    ui.visuals().widgets.inactive
}

pub const FAR: Pos2 = pos2(-10.0, -10.0);

// should be even number because SIZE / 2.0 produces .5 and it may break rendering.
pub const SIZE: Vec2 = vec2(16.0, 16.0);
