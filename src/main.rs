use eframe::*;
use egui::CentralPanel;

struct SSM {}

impl eframe::App for SSM {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello, world!");
        });
    }
}

fn main() -> eframe::Result<(), eframe::Error> {
    run_native(
    "Source Spray Manager",
    NativeOptions::default(),
    Box::new(|_cc: &CreationContext<'_>|{
        Box::new(SSM {})
    }))
}