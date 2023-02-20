use eframe::egui;
use egui::pos2;
use egui_cable::prelude::*;

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.default_theme = eframe::Theme::Light;
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|_| Box::new(MyEguiApp::default())),
    )
    .expect("Failed to start native application");
}

#[derive(Default)]
struct MyEguiApp {}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("My window")
            .default_pos(pos2(20.0, 100.0))
            .show(ctx, |ui| {
                ui.add(Port::new(0));
            });
        egui::Window::new("My window 2")
            .default_pos(pos2(200.0, 20.0))
            .show(ctx, |ui| {
                ui.add(Port::new(1));
                ui.add_space(10.0);
                ui.add(Port::new(2));
            });
        egui::Window::new("My window 3")
            .default_pos(pos2(200.0, 200.0))
            .show(ctx, |ui| {
                ui.add(Port::new(3));

                ui.add(Cable::new(0, Plug::to(0), Plug::to(1)));
                ui.add(Cable::new(1, Plug::to(0), Plug::to(3)));
                ui.add(Cable::new(2, Plug::to(2), Plug::unplugged()));
            });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::WHITE.to_array()
    }
}
