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
struct MyEguiApp {}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("My window").show(ctx, |ui| {
            ui.add(
                Port::new(PortId::new(0))
                    .with_cable(PortId::new(1), Cable::unit())
                    .with_cable(PortId::new(3), Cable::unit()),
            );
        });
        egui::Window::new("My window 2").show(ctx, |ui| {
            ui.add(Port::new(PortId::new(1)));
            ui.add(Port::new(PortId::new(2)).with_cable(PortId::new(3), Cable::unit()));
        });
        egui::Window::new("My window 3").show(ctx, |ui| {
            ui.add(Port::new(PortId::new(3)).with_cable(PortId::new(1), Cable::unit()));
        });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::WHITE
    }
}
