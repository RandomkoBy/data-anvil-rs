use egui;

mod app;
mod screens;
mod localization;

use crate::localization::{Language, get_texts};
use crate::screens::Screen;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([600.0, 400.0])
            .with_title("Rust Utilities App"),
        ..Default::default()
    };

    eframe::run_native(
        "Rust Utilities",
        options,
        Box::new(|cc| {
            let mut app = app::UtilitiesApp::default();
            app.setup_style(cc);
            Box::new(app)
        }),
    )
}

#[derive(Default)]
struct UtilitiesApp {
    current_screen: Screen,
    current_language: Language,
}

impl UtilitiesApp {
    fn setup_style(&mut self, cc: &eframe::CreationContext) {
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
                Screen::MainMenu => self.show_main_menu(ui, &texts),
                Screen::XmlToJson => self.show_xml_to_json(ui, &texts),
                Screen::UuidGenerator => self.show_uuid_generator(ui, &texts),
            }
        });
    }
}

impl UtilitiesApp {
    fn show_main_menu(&mut self, ui: &mut egui::Ui, texts: &crate::localization::Texts) {
        ui.vertical_centered(|ui| {
            // Language selector in top right
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                ui.horizontal(|ui| {
                    egui::ComboBox::from_id_source("language_selector")
                        .selected_text(match self.current_language {
                            Language::Russian => "Русский",
                            Language::English => "English",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.current_language, Language::Russian, "Русский");
                            ui.selectable_value(&mut self.current_language, Language::English, "English");
                        });
                });
            });
            
            ui.add_space(50.0);
            
            ui.heading(texts.app_title);
            ui.add_space(20.0);
            ui.label(texts.select_utility);
            ui.add_space(30.0);
            
            ui.horizontal(|ui| {
                ui.add_space(50.0);
                ui.vertical(|ui| {
                    if ui.add_sized([200.0, 80.0], egui::Button::new(format!("{} \n{}", texts.xml_to_json_title, texts.xml_to_json_desc))).clicked() {
                        self.current_screen = Screen::XmlToJson;
                    }
                    
                    ui.add_space(20.0);
                    
                    if ui.add_sized([200.0, 80.0], egui::Button::new(format!("{} \n{}", texts.uuid_generator_title, texts.uuid_generator_desc))).clicked() {
                        self.current_screen = Screen::UuidGenerator;
                    }
                });
                
                ui.add_space(50.0);
                
                ui.vertical(|ui| {
                    ui.add_sized([200.0, 80.0], egui::Button::new(format!("{} \n{}", texts.password_gen_title, texts.password_gen_desc)).wrap(false));
                    
                    ui.add_space(20.0);
                    
                    ui.add_sized([200.0, 80.0], egui::Button::new(format!("{} \n{}", texts.file_analyzer_title, texts.file_analyzer_desc)).wrap(false));
                });
            });
        });
    }
    
    fn show_xml_to_json(&mut self, ui: &mut egui::Ui, texts: &crate::localization::Texts) {
        ui.vertical(|ui| {
            if ui.button(texts.back_button).clicked() {
                self.current_screen = Screen::MainMenu;
            }
            
            ui.add_space(20.0);
            ui.heading(texts.xml_to_json_heading);
            ui.separator();
            ui.add_space(20.0);
            
            ui.label(texts.xml_to_json_placeholder);
        });
    }
    
    fn show_uuid_generator(&mut self, ui: &mut egui::Ui, texts: &crate::localization::Texts) {
        ui.vertical(|ui| {
            if ui.button(texts.back_button).clicked() {
                self.current_screen = Screen::MainMenu;
            }
            
            ui.add_space(20.0);
            ui.heading(texts.uuid_generator_heading);
            ui.separator();
            ui.add_space(20.0);
            
            ui.label(texts.uuid_generator_placeholder);
        });
    }
}