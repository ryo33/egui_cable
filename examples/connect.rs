use eframe::egui;
use egui_cable::prelude::*;

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.default_theme = eframe::Theme::Light;
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|_| Box::new(MyEguiApp::default())),
    )
    .expect("Failed to start native platform");
}

#[derive(Default)]
struct MyEguiApp {
    connected: Vec<(usize, usize)>,
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Connect me").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add(Port::new(0usize));
                ui.add_space(40.0);
                ui.add(Port::new(1usize));
            });

            if self.connected.is_empty() {
                if let Some(port_id) = ui
                    .add(Cable::new(0, Plug::to(0), Plug::unplugged()))
                    .out_plug()
                    .connected_to()
                {
                    self.connected.push((0, *port_id.downcast_ref().unwrap()));
                }
            }
            for (a, b) in self.connected.iter() {
                ui.add(Cable::new((*a, *b), Plug::to(*a), Plug::to(*b)));
            }
        });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::WHITE.to_array()
    }
}
