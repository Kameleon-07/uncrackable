#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window in Windows on release

use eframe::egui;
use uncrackable;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Uncrackable",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    password: String,
    password_length: i32,
    include_special_characters: bool,
    use_uppercase: bool,
    use_numbers: bool,
    use_underlines: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            password: "".to_string(),
            password_length: 20,
            include_special_characters: true,
            use_uppercase: true,
            use_numbers: true,
            use_underlines: true,
        }
    }
}

impl eframe::App for MyApp {    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_visuals(egui::style::Visuals::dark());
            ui.heading("Uncrackable, your password generator!");
            ui.add(
                egui::Slider::new(
                    &mut self.password_length, 6..=55
                )
                .text("Password length")
            );
            ui.add(
                egui::Checkbox::new(&mut self.include_special_characters, "Include special characters")
            );
            ui.add(
                egui::Checkbox::new(&mut self.use_uppercase, "Use upper case letters")
            );
            ui.add(
                egui::Checkbox::new(&mut self.use_numbers, "Include numbers")
            );
            ui.add(
                egui::Checkbox::new(&mut self.use_underlines, "Use underlines")
            );

            let mut password_builder = uncrackable::PasswordBuilder::new()
                .set_length(self.password_length);

            if self.include_special_characters {
                password_builder = password_builder.include_special_characters();
            }

            if self.use_numbers {
                password_builder = password_builder.use_numbers();
            }

            if self.use_uppercase {
                password_builder = password_builder.use_uppercase();
            }

            if self.use_underlines {
                password_builder = password_builder.use_underlines();
            }


            if ui.add(egui::Button::new("Generate password")).clicked() {
                self.password = format!(
                    "Generated password: {}",
                    password_builder.build()
                )
            }

            ui.add(
                egui::Label::new(&self.password)
            )
        });
    }
}