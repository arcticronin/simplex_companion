use egui::{TextEdit, Ui};
use num_rational::ParseRatioError;
use num_rational::Ratio;
use num_rational::Rational32;
use std::str::FromStr;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old statepub
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
    mode: Mode,
    output_text: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    matrix: Vec<Vec<Rational32>>,
    n: i32,
    m: i32,
}

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
enum Mode {
    Edit,
    Pivot,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            mode: Mode::Edit,
            output_text: "".to_owned(),
            n: 4,
            m: 3,
            matrix: vec![vec![Rational32::new(0, 1); 3]; 4],
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }
                ui.menu_button("Tableau", |ui| {
                    //NOTE behaviour!
                    if ui.button("Add Constraint").clicked() {};
                    if ui.button("Add Variable").clicked() {};
                    if ui.button("Undo Last Tableau").clicked() {};
                });

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::SidePanel::left("Options").show(ctx, |ui| {
            ui.heading("Mode");
            ui.radio_value(&mut self.mode, Mode::Edit, "Edit");
            ui.radio_value(&mut self.mode, Mode::Pivot, "Pivot");

            ui.separator();

            if ui.button("Click me!").clicked() {
                // Perform some action when the button is clicked
                // For example, update some output text
                self.output_text = "Button clicked!".to_string();
            }

            ui.separator();

            // Add a read-only text edit field to display output
            ui.label(&self.output_text);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Enter your matrix:");

            for row in &mut self.matrix {
                ui.horizontal(|ui| {
                    for rational in row {
                        // ui for the string that must be parsed
                        //ui.add(egui::DragValue::new(&mut temp).speed(1));

                        let mut temp = String::new();

                        ui.add(TextEdit::singleline(&mut temp));

                        let parsed_num = Ratio::from_str(&temp);

                        match parsed_num {
                            Ok(parsed_num) => {
                                *rational = parsed_num;
                            }
                            Err(_) => {
                                println!("An unknown error occurred");
                            }
                        }
                    }
                });
            }

            //footer
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
