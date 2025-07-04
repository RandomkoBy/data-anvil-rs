use egui;
use super::Screen;
use crate::localization::Texts;

pub fn show(ui: &mut egui::Ui, current_screen: &mut Screen, texts: &Texts) {
    ui.vertical(|ui| {
        if ui.button(texts.back_button).clicked() {
            *current_screen = Screen::MainMenu;
        }
        
        ui.add_space(20.0);
        ui.heading(texts.xml_to_json_heading);
        ui.separator();
        ui.add_space(20.0);
        
        ui.label(texts.xml_to_json_placeholder);
    });
}