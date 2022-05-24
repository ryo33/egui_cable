use eframe::egui;
use egui_cable::prelude::*;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|_| Box::new(MyEguiApp::default())),
    );
}

#[derive(Default)]
struct MyEguiApp {
    connected: Vec<(PortId, PortId)>,
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Connect me").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add(Port::new(0));
                ui.add_space(40.0);
                ui.add(Port::new(1));
            });

            if self.connected.is_empty() {
                if let Some(port_id) = ui
                    .add(Cable::new(-1, PortId::new(0), Plug::new()))
                    .connected_to()
                {
                    self.connected.push((PortId::new(0), port_id));
                }
            }
            for (i, (a, b)) in self.connected.iter().enumerate() {
                ui.add(Cable::new(i, *a, *b));
            }
        });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::WHITE
    }
}
