#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window in Windows on release

use eframe::egui;
use rand::{thread_rng, Rng};

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
    use_different_letter_cases: bool,
    use_numbers: bool,
    use_underlines: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            password: "".to_string(),
            password_length: 20,
            include_special_characters: true,
            use_different_letter_cases: true,
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
                egui::Checkbox::new(&mut self.use_different_letter_cases, "Use different letter cases")
            );
            ui.add(
                egui::Checkbox::new(&mut self.use_numbers, "Include numbers")
            );
            ui.add(
                egui::Checkbox::new(&mut self.use_underlines, "Use underlines")
            );

            if ui.add(egui::Button::new("Generate password")).clicked() {
                self.password = format!(
                    "Generated password {}",
                    generate_password(self)
                )
            }

            ui.add(
                egui::Label::new(&self.password)
            )
        });
    }
}

fn generate_password(passwd_parameters: &mut MyApp) -> String {
    let mut password = String::new();
    let mut rng = thread_rng();
    for _ in 0..passwd_parameters.password_length {
        let rand_num = rng.gen_range(33..=126);
        password.push(rand_num as u8 as char);
    }

    password
}