//use egui::{TextEdit, Ui, ScrollArea};
//use num_rational::ParseRatioError;
use egui::TextEdit;
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

    #[serde(skip)] // This how you opt-out of serialization of a field
    mode: Mode,

    #[serde(skip)] // This how you opt-out of serialization of a field
    output_text: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    matrix: Vec<Vec<Rational32>>,

    #[serde(skip)] // This how you opt-out of serialization of a field
    input_strings: Vec<Vec<String>>,

    n: usize,
    m: usize,
}

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
enum Mode {
    Edit,
    Pivot,
}
pub struct Tableau {
    iteration: usize,
    vars: Vec<String>,
    artificial_vars: Vec<String>,
    tableau: Vec<Equation>,
}
pub struct Equation {
    basic_var: String,
    vec: Vec<Rational32>,
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
            matrix: vec![vec![Rational32::new(0, 1); 3]; 10],
            input_strings: vec![vec!["0".to_string(); 3]; 10],
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
                    if ui.button("Add Constraint").clicked() {
                        self.n += 1;
                    };
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
            egui::ScrollArea::new([false, true]).show(ui, |ui| {
                // The central panel the region left after adding TopPanel's and SidePanel's
                if self.mode == Mode::Edit {
                    ui.heading("Enter your matrix:");

                    egui::Grid::new("some_unique_id").show(ui, |ui| {
                        for (row_index, row) in self.matrix.iter_mut().enumerate() {
                            ui.horizontal(|ui| {
                                for (col_index, rational) in row.iter_mut().enumerate() {
                                    let temp = &mut self.input_strings[row_index][col_index];

                                    // Add the text edit and get its response
                                    let response =
                                        ui.add(TextEdit::singleline(temp).desired_width(50.0));

                                    // Check if this widget lost focus
                                    if response.lost_focus() {
                                        match Ratio::from_str(temp) {
                                            Ok(parsed_num) => {
                                                *rational = parsed_num;
                                            }
                                            Err(_e) => {
                                                *rational = Rational32::new(0, 1);
                                            }
                                        }
                                    }
                                }
                            });
                            ui.end_row();
                        }
                    });
                } else if self.mode == Mode::Pivot {
                    ui.heading("Starting Pivot Operations");

                    egui::Grid::new("some_unique_id").show(ui, |ui| {
                        for (row_index, row) in self.matrix.clone().iter_mut().enumerate() {
                            ui.horizontal(|ui| {
                                for (col_index, _rational) in row.iter_mut().enumerate() {
                                    if col_index == 0 {
                                        ui.separator();
                                        ui.separator();
                                    }

                                    ui.label(format!("{}", self.matrix[row_index][col_index]));
                                    if col_index < self.m - 2 {
                                        ui.separator();
                                    } else if col_index == self.m - 2 {
                                        ui.separator();
                                        ui.separator();
                                    }
                                }
                            });
                            ui.end_row();
                        }
                    });
                }

                //footer
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    powered_by_egui_and_eframe(ui);
                    egui::warn_if_debug_build(ui);
                });
            });
        });

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
    }
}
