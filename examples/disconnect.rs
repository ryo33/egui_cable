use eframe::egui;
use egui_cable::prelude::*;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|_| {
            Box::new(MyEguiApp {
                connected: vec![(0, Some(0), Some(1))],
            })
        }),
    );
}

#[derive(Default)]
struct MyEguiApp {
    connected: Vec<(usize, Option<usize>, Option<usize>)>,
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Disconnect me").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add(Port::new(0));
                ui.add_space(40.0);
                ui.add(Port::new(1));
            });

            for (key, a, b) in self.connected.iter_mut() {
                let in_plug = if let Some(a) = a {
                    Plug::to(*a)
                } else {
                    Plug::unplugged()
                };
                let out_plug = if let Some(b) = b {
                    Plug::to(*b)
                } else {
                    Plug::unplugged()
                };
                let response = ui.add(Cable::new(*key, in_plug, out_plug));
                if response.in_plug().disconnected() {
                    *a = None;
                }
                if response.out_plug().disconnected() {
                    *b = None;
                }
            }
        });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::WHITE
    }
}
