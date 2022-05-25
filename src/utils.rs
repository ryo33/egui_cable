use egui::{pos2, style::WidgetVisuals, Pos2, Response, Ui};

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
