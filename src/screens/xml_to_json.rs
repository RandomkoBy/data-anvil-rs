use eframe::egui;
use super::Screen;

pub fn show(ui: &mut egui::Ui, current_screen: &mut Screen) {
    ui.vertical(|ui| {
        // Кнопка "Назад"
        if ui.button("⬅️ Назад").clicked() {
            *current_screen = Screen::MainMenu;
        }
        
        ui.add_space(20.0);
        ui.heading("🔄 XML to JSON Converter");
        ui.separator();
        ui.add_space(20.0);
        
        ui.label("Здесь будет интерфейс для конвертации XML в JSON");
        // Позже добавим полную функциональность
    });
}