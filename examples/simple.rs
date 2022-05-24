use eframe::egui;
use egui::{pos2, Order};
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
        egui::Window::new("My window")
            .default_pos(pos2(20.0, 100.0))
            .show(ctx, |ui| {
                ui.add(Port::new(PortId::new(0)));
            });
        egui::Window::new("My window 2")
            .default_pos(pos2(200.0, 20.0))
            .show(ctx, |ui| {
                ui.add(Port::new(PortId::new(1)));
                ui.add(Port::new(PortId::new(2)));
            });
        egui::Window::new("My window 3")
            .default_pos(pos2(200.0, 200.0))
            .show(ctx, |ui| {
                ui.add(Port::new(PortId::new(3)));
            });

        egui::Area::new("cables")
            .order(Order::Foreground)
            .show(ctx, |ui| {
                ui.add(Cable::new(0, PortId::new(0), PortId::new(1)));
                ui.add(Cable::new(1, PortId::new(0), PortId::new(3)));
                ui.add(Cable::new(2, PortId::new(2), PortId::new(3)));
                ui.add(Cable::new(3, PortId::new(1), PortId::new(1)));
            });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::WHITE
    }
}
