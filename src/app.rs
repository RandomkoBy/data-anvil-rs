use egui;
use crate::screens::{Screen, main_menu, xml_to_json, uuid_generator};
use crate::localization::{Language, get_texts};

#[derive(Default)]
pub struct UtilitiesApp {
    current_screen: Screen,
    current_language: Language,
}

impl UtilitiesApp {
    pub fn setup_style(&mut self, cc: &eframe::CreationContext) {
        let ctx = &cc.egui_ctx;
        let mut style = (*ctx.style()).clone();
        
        style.visuals.dark_mode = true;
        style.visuals.override_text_color = Some(egui::Color32::WHITE);
        style.visuals.window_fill = egui::Color32::from_rgb(30, 30, 35);
        style.visuals.panel_fill = egui::Color32::from_rgb(25, 25, 30);
        
        style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(60, 60, 70);
        style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(70, 70, 80);
        style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(80, 80, 90);
        
        style.visuals.widgets.inactive.rounding = egui::Rounding::same(8.0);
        style.visuals.widgets.hovered.rounding = egui::Rounding::same(8.0);
        style.visuals.widgets.active.rounding = egui::Rounding::same(8.0);
        
        ctx.set_style(style);
    }
}

impl eframe::App for UtilitiesApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let texts = get_texts(self.current_language);
        
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_screen {
                Screen::MainMenu => main_menu::show(ui, &mut self.current_screen, &mut self.current_language, &texts),
                Screen::XmlToJson => xml_to_json::show(ui, &mut self.current_screen, &texts),
                Screen::UuidGenerator => uuid_generator::show(ui, &mut self.current_screen, &texts),
            }
        });
    }
}