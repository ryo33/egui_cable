use eframe::egui;
use egui::Pos2;
use egui_cable::prelude::*;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|_| {
            Box::new(MyEguiApp {
                name: "".into(),
                ports: vec![],
                cables: vec![],
            })
        }),
    );
}

struct MyPort {
    name: String,
    pos: Pos2,
}

struct MyCable {
    name: String,
    in_plug: MyPlug,
    out_plug: MyPlug,
}

struct MyPlug {
    pos: Pos2,
    to: Option<String>,
}

#[derive(Default)]
struct MyEguiApp {
    name: String,
    ports: Vec<MyPort>,
    cables: Vec<MyCable>,
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("full control").show(ctx, |ui| {
            ui.text_edit_singleline(&mut self.name);
            if ui.button("Add port").clicked() {
                self.ports.push(MyPort {
                    name: self.name.clone(),
                    pos: ui.next_widget_position(),
                });
                self.name = format!("{}'", self.name);
            }
            for port in &self.ports {
                ui.horizontal(|ui| {
                    if ui.add(Port::new(port.name.clone())).clicked() {
                        self.cables.push(MyCable {
                            name: format!("{}", self.cables.len()),
                            in_plug: MyPlug {
                                pos: port.pos,
                                to: Some(port.name.clone()),
                            },
                            out_plug: MyPlug {
                                pos: ui.next_widget_position(),
                                to: None,
                            },
                        });
                    }
                });
            }
            for cable in &mut self.cables {
                let response = ui.add(Cable::new(
                    cable.name.clone(),
                    cable
                        .in_plug
                        .to
                        .as_ref()
                        .map(|to| Plug::to(to.clone()))
                        .unwrap_or_else(Plug::unplugged)
                        .pos(cable.in_plug.pos),
                    cable
                        .out_plug
                        .to
                        .as_ref()
                        .map(|to| Plug::to(to.clone()))
                        .unwrap_or_else(Plug::unplugged)
                        .pos(cable.out_plug.pos),
                ));

                let in_plug = response.in_plug();
                if let Some(to) = in_plug.connected_to() {
                    cable.in_plug.to = Some(to.downcast_ref::<String>().unwrap().clone());
                }
                if in_plug.disconnected() {
                    cable.in_plug.to = None;
                }
                if let Some(on) = in_plug.hovered_on() {
                    println!(
                        "in-plug of {} hovered on {}",
                        cable.name,
                        on.downcast_ref::<String>().unwrap()
                    );
                }
                cable.in_plug.pos = in_plug.next_position();

                let out_plug = response.out_plug();
                if let Some(to) = out_plug.connected_to() {
                    cable.out_plug.to = Some(to.downcast_ref::<String>().unwrap().clone());
                }
                if out_plug.disconnected() {
                    cable.out_plug.to = None;
                }
                if let Some(on) = out_plug.hovered_on() {
                    println!(
                        "out-plug of {} hovered on {}",
                        cable.name,
                        on.downcast_ref::<String>().unwrap()
                    );
                }
                cable.out_plug.pos = out_plug.next_position();
            }
        });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::WHITE
    }
}
