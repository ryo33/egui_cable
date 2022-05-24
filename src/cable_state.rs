use egui::Vec2;

#[derive(Clone, Debug)]
pub(crate) struct CableState {
    pub relative_control_point_pos: Vec2,
    pub dragged: bool,
    pub active: bool,
}
