#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::fs;
use eframe::{egui::{self, TextEdit, menu}, epaint::{FontFamily, FontId}, emath::Align};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 640.0]),
        ..Default::default()
    };
    eframe::run_native(
        "rustidian",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyApp>::default()
        }),
    )
}

struct MyApp {
    body_text: String,
    title_text: String,
    old_name: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            body_text: "".to_owned(),
            title_text: "untitled".to_owned(),
            old_name: "untitled.md".to_owned(),
        }
    }
}

impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // start a rudimentary built-in top drop-down menu
        egui::TopBottomPanel::top("hi").show(ctx, |ui| {
            
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open file").clicked() {
                    }
                });
            });
         });

        egui::CentralPanel::default().show(ctx, |ui| {

            // define fonts unsed in central panel
            let body_font_id = FontId::new(12.0, FontFamily::default());
            let title_font_id = FontId::new(17.0, FontFamily::default());

            ui.vertical_centered(|ui| {

                // title text box
                let title_edit = TextEdit::singleline(&mut self.title_text)
                    .font(title_font_id)//;
                    .horizontal_align(Align::Center)
                    .desired_width(700.0)
                    .hint_text("title");
                let save_title = ui.add(title_edit);

                if save_title.changed() {
                    self.old_name = change_title(self.old_name.clone(), self.title_text.clone());
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
                    save_file(self.body_text.clone(), self.title_text.clone());
                }

            });
        });
    }
}

// a function that changes the name of the file being edited and returns the new name
pub fn change_title(old_title: String, new_title: String) -> String {

    let new_name = format!("{}.md", new_title);

    match fs::rename(old_title.clone(), new_name.clone()) {
        Ok(()) => println!("File renamed from {} to {} successfully", old_title, new_name),
        Err(e) => eprintln!("Error renaming file: {}", e),
    }
    new_name
}

// a function that saves all the text in the body text box to a file with the name from the title text box
pub fn save_file(text: String, title: String) {
    let file_path = format!("{}.md", title);
    match fs::write(file_path.clone(), text) {
        Ok(_) => println!("Text saved to {}", file_path),
        Err(e) => eprintln!("Error saving text to {}: {}", file_path, e),
    }
}

