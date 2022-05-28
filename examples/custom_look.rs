use eframe::egui;
use egui::pos2;
use egui_cable::prelude::*;

#[derive(Debug)]
struct CustomPort;

impl egui::Widget for CustomPort {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        // You can use the params for rendering.
        let _params = PortParams::get(ui.data());

        // You can use this if you like the default look. If not, you implement your own in here.
        let response = ui.add(DefaultPort);

        ui.label("This is custom port");

        response
    }
}

#[derive(Debug)]
struct CustomPlug;

impl egui::Widget for CustomPlug {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        // You can use the params for rendering.
        let _params = PlugParams::get(ui.data());

        // You can use this if you like the default look. If not, you implement your own in here.
        let response = ui.add(DefaultPlug);

        ui.label("This is custom plug");

        response
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|_| Box::new(MyEguiApp::default())),
    );
}

#[derive(Default)]
struct MyEguiApp {}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("My window")
            .default_pos(pos2(20.0, 100.0))
            .show(ctx, |ui| {
                ui.add(Port::new(0).widget(CustomPort));
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

                ui.add(Cable::new(0, Plug::to(0), Plug::to(1).widget(CustomPlug)));
                ui.add(Cable::new(1, Plug::to(0), Plug::to(3)));
                ui.add(Cable::new(2, Plug::to(2), Plug::unplugged()));
            });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::WHITE
    }
}
