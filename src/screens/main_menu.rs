use eframe::egui;
use super::Screen;

pub fn show(ui: &mut egui::Ui, current_screen: &mut Screen) {
    ui.vertical_centered(|ui| {
        ui.add_space(50.0);
        
        ui.heading("🛠️ Rust Utilities");
        ui.add_space(20.0);
        ui.label("Выберите утилиту:");
        ui.add_space(30.0);
        
        ui.horizontal(|ui| {
            ui.add_space(50.0);
            ui.vertical(|ui| {
                if ui.add_sized([200.0, 80.0], egui::Button::new("🔄 XML → JSON\nКонвертация XML в JSON")).clicked() {
                    *current_screen = Screen::XmlToJson;
                }
                
                ui.add_space(20.0);
                
                if ui.add_sized([200.0, 80.0], egui::Button::new("🆔 UUID Generator\nГенерация уникальных ID")).clicked() {
                    *current_screen = Screen::UuidGenerator;
                }
            });
            
            ui.add_space(50.0);
            
            ui.vertical(|ui| {
                ui.add_sized([200.0, 80.0], egui::Button::new("🔐 Password Gen\n(Coming Soon)").wrap(false));
                
                ui.add_space(20.0);
                
                ui.add_sized([200.0, 80.0], egui::Button::new("📊 File Analyzer\n(Coming Soon)").wrap(false));
            });
        });
    });
}