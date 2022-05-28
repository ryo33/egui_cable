use eframe::egui;
use egui::pos2;
use egui_cable::prelude::*;

#[derive(Debug)]
struct CustomPort;

impl egui::Widget for CustomPort {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        // You can use the params for rendering.
        let _params = PortParams::get(ui.data());

        // Derive the default or implement your own.
        // You should take a look into the default implementation.
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

        // Derive the default or implement your own.
        // You should take a look into the default implementation.
        let response = ui.add(DefaultPlug);

        ui.label("This is custom plug");

        response
    }
}

#[derive(Debug)]
struct CustomCable;

impl egui::Widget for CustomCable {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        // You can use the params for rendering.
        let params = CableParams::get(ui.data());

        let mut bezier = params.bezier;
        bezier.stroke = (5.0, epaint::Color32::GOLD).into();

        ui.painter().add(bezier);

        ui.add(params.cable_control)
    }
}

#[derive(Debug)]
struct CustomControl;

impl egui::Widget for CustomControl {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.add(egui::Label::new("This is custom control").sense(egui::Sense::drag()))
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
            .default_pos(pos2(20.0, 190.0))
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
            .default_pos(pos2(200.0, 300.0))
            .show(ctx, |ui| {
                ui.add(Port::new(3));

                ui.add(
                    Cable::new(0, Plug::to(0), Plug::to(1).widget(CustomPlug))
                        .widget(CustomCable)
                        .control_widget(CustomControl),
                );
                ui.add(Cable::new(1, Plug::to(0), Plug::to(3)).control_widget(CustomControl));
                ui.add(Cable::new(2, Plug::to(2), Plug::unplugged()));
            });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::WHITE
    }
}
