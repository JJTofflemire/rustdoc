#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{
    env, fmt::format, fs, path::{Path, PathBuf}, process::ExitStatus
};

use eframe::{
    egui::{self, menu, ScrollArea, TextEdit},
    emath::Align,
    epaint::{FontFamily, FontId},
};
mod file_management;
use file_management::{change_title, display_explorer, open_dir, open_file, save_file};

struct MyApp {
    body_text: String,
    title_text: String,
    old_name: String,
    working_dir: PathBuf,
    config_dir: PathBuf,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            body_text: "".to_owned(),
            title_text: "untitled".to_owned(),
            old_name: "untitled.md".to_owned(),
            working_dir: env::current_dir().expect("Failed to get current directory"),
            config_dir: directories::BaseDirs::new()
                .unwrap()
                .config_dir()
                .to_path_buf(),
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 640.0]),
        ..Default::default()
    };

    eframe::run_native(
        "rustdoc",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            let mut app = MyApp::default();
            app.startup();
            Box::new(app)
        }),
    )
}

impl MyApp {
    fn startup(&mut self) {
        let rust_config_dir = format!("{}/rustdoc", self.config_dir.display());
        if !Path::new(rust_config_dir.as_str()).exists() {
            match fs::create_dir(rust_config_dir) {
                Ok(_) => (),
                Err(e) => eprintln!("{}", e),
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // start a rudimentary built-in top drop-down menu
        egui::TopBottomPanel::top("hi").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("file", |ui| {
                    if ui.button("add file").clicked() {
                        match open_file(self.working_dir.clone()) {
                            Some(file_data_output_real) => {
                                self.body_text = file_data_output_real.new_body_text;
                                self.title_text = file_data_output_real.new_title_text_short;
                                self.old_name = file_data_output_real.new_old_title_text_short;
                            }
                            None => (),
                        }
                    }
                    ui.collapsing("open new directory", |ui| {
                        ui.collapsing("previous directories", |ui| {
                            // read a previous folders json and output as buttons
                        });
                        if ui.button("add new directory").clicked() {
                            match open_dir() {
                                Some(new_working_dir) => self.working_dir = new_working_dir,
                                None => (),
                            }
                        }
                    });
                });
            });
        });

        egui::SidePanel::left("ls panel").show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| match display_explorer(&self.working_dir, ui) {
                Some(file_data_output) => {
                    self.body_text = file_data_output.new_body_text;
                    self.title_text = file_data_output.new_title_text_short;
                    self.old_name = file_data_output.new_old_title_text_short;
                    self.working_dir = file_data_output.new_working_dir;
                }
                None => {}
            });
        });

        // central panel with title and body text boxes
        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                // define fonts unsed in central panel
                let body_font_id = FontId::new(12.0, FontFamily::default());
                let title_font_id = FontId::new(17.0, FontFamily::default());

                ui.vertical_centered(|ui| {
                    // title text box
                    let title_edit = TextEdit::singleline(&mut self.title_text)
                        .font(title_font_id) //;
                        .horizontal_align(Align::Center)
                        .desired_width(700.0)
                        .hint_text("title");
                    let save_title = ui.add(title_edit);

                    if save_title.changed() {
                        self.old_name =
                            change_title(self.old_name.clone(), self.title_text.clone());
                    }

                    // body text box
                    let text_edit = TextEdit::multiline(&mut self.body_text)
                        .font(body_font_id)
                        .lock_focus(true)
                        .desired_rows(1)
                        .desired_width(700.0)
                        .hint_text("body text");
                    //.frame(false);
                    let save_body = ui.add(text_edit);

                    if save_body.changed() {
                        save_file(
                            self.body_text.clone(),
                            self.title_text.clone(),
                            self.working_dir.clone(),
                        );
                    }
                });
            });
        });

        // attempting to implement drag and drop file support, but there is little documentation on this
        //        for event in ui.input(|i| i.raw.dropped_files.clone()) {
        //            dropped_file = Some(event);
    }
}
