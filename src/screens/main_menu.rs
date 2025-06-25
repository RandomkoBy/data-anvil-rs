use egui;
use super::Screen;
use crate::localization::{Language, Texts};

pub fn show(ui: &mut egui::Ui, current_screen: &mut Screen, current_language: &mut Language, texts: &Texts) {
    let available_rect = ui.available_rect_before_wrap();
    let window_width = available_rect.width();
    let window_height = available_rect.height();
    
    // Responsive button sizing
    let button_width = (window_width * 0.25).max(180.0).min(250.0);
    let button_height = (window_height * 0.12).max(60.0).min(100.0);
    let spacing = (window_width * 0.05).max(20.0).min(50.0);
    
    ui.vertical_centered(|ui| {
        // Language selector in top right
        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            ui.horizontal(|ui| {
                egui::ComboBox::from_id_source("language_selector")
                    .selected_text(match current_language {
                        Language::Russian => "Русский",
                        Language::English => "English",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(current_language, Language::Russian, "Русский");
                        ui.selectable_value(current_language, Language::English, "English");
                    });
            });
        });
        
        // Responsive vertical spacing
        ui.add_space(window_height * 0.08);
        
        // Title with responsive font size
        ui.heading(texts.app_title);
        ui.add_space(window_height * 0.03);
        ui.label(texts.select_utility);
        ui.add_space(window_height * 0.05);
        
        // Responsive layout - switch to vertical on narrow screens
        if window_width > 600.0 {
            // Wide screen - horizontal layout
            ui.horizontal(|ui| {
                ui.add_space(spacing);
                
                ui.vertical(|ui| {
                    if ui.add_sized([button_width, button_height], 
                        egui::Button::new(format!("{} \n{}", texts.xml_to_json_title, texts.xml_to_json_desc))
                    ).clicked() {
                        *current_screen = Screen::XmlToJson;
                    }
                    
                    ui.add_space(spacing * 0.4);
                    
                    if ui.add_sized([button_width, button_height], 
                        egui::Button::new(format!("{} \n{}", texts.uuid_generator_title, texts.uuid_generator_desc))
                    ).clicked() {
                        *current_screen = Screen::UuidGenerator;
                    }
                });
                
                ui.add_space(spacing);
                
                ui.vertical(|ui| {
                    ui.add_sized([button_width, button_height], 
                        egui::Button::new(format!("{} \n{}", texts.password_gen_title, texts.password_gen_desc))
                            .wrap(false)
                    );
                    
                    ui.add_space(spacing * 0.4);
                    
                    ui.add_sized([button_width, button_height], 
                        egui::Button::new(format!("{} \n{}", texts.file_analyzer_title, texts.file_analyzer_desc))
                            .wrap(false)
                    );
                });
                
                ui.add_space(spacing);
            });
        } else {
            // Narrow screen - vertical layout
            ui.vertical_centered(|ui| {
                let narrow_button_width = (window_width * 0.8).max(200.0).min(300.0);
                
                if ui.add_sized([narrow_button_width, button_height], 
                    egui::Button::new(format!("{} \n{}", texts.xml_to_json_title, texts.xml_to_json_desc))
                ).clicked() {
                    *current_screen = Screen::XmlToJson;
                }
                
                ui.add_space(spacing * 0.3);
                
                if ui.add_sized([narrow_button_width, button_height], 
                    egui::Button::new(format!("{} \n{}", texts.uuid_generator_title, texts.uuid_generator_desc))
                ).clicked() {
                    *current_screen = Screen::UuidGenerator;
                }
                
                ui.add_space(spacing * 0.3);
                
                ui.add_sized([narrow_button_width, button_height], 
                    egui::Button::new(format!("{} \n{}", texts.password_gen_title, texts.password_gen_desc))
                        .wrap(false)
                );
                
                ui.add_space(spacing * 0.3);
                
                ui.add_sized([narrow_button_width, button_height], 
                    egui::Button::new(format!("{} \n{}", texts.file_analyzer_title, texts.file_analyzer_desc))
                        .wrap(false)
                );
            });
        }
    });
}