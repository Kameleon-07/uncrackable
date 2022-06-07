#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window in Windows on release

use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Uncrackable",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    password_length: i32,
    include_special_characters: bool,
    use_different_letter_cases: bool,
    use_numbers: bool,
    use_underlines: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
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
        });
    }
}