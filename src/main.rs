use eframe::egui;

mod app;
mod screens;

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
}

#[derive(Default, PartialEq)]
enum Screen {
    #[default]
    MainMenu,
    XmlToJson,
    UuidGenerator,
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
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_screen {
                Screen::MainMenu => self.show_main_menu(ui),
                Screen::XmlToJson => self.show_xml_to_json(ui),
                Screen::UuidGenerator => self.show_uuid_generator(ui),
            }
        });
    }
}

impl UtilitiesApp {
    fn show_main_menu(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            
            ui.heading("🛠️ Rust Utilities");
            ui.add_space(20.0);
            
            ui.horizontal(|ui| {
                ui.add_space(50.0);
                ui.vertical(|ui| {
                    if ui.add_sized([200.0, 80.0], egui::Button::new("🔄 XML → JSON\nКонвертация XML в JSON")).clicked() {
                        self.current_screen = Screen::XmlToJson;
                    }
                    
                    ui.add_space(20.0);
                    
                    if ui.add_sized([200.0, 80.0], egui::Button::new("🆔 UUID Generator\nГенерация уникальных ID")).clicked() {
                        self.current_screen = Screen::UuidGenerator;
                    }
                });
                
                ui.add_space(50.0);
                
                ui.vertical(|ui| {
                    ui.add_sized([200.0, 80.0], egui::Button::new("📝 Text Editor\n(Coming Soon)").wrap(false));

                    ui.add_sized([200.0, 80.0], egui::Button::new("🔐 Password Gen\n(Coming Soon)").wrap(false));
                    
                    ui.add_space(20.0);
                    
                    ui.add_sized([200.0, 80.0], egui::Button::new("📊 File Analyzer\n(Coming Soon)").wrap(false));
                });
            });
        });
    }
    
    fn show_xml_to_json(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            if ui.button("⬅️ Назад").clicked() {
                self.current_screen = Screen::MainMenu;
            }
            
            ui.add_space(20.0);
            ui.heading("🔄 XML to JSON Converter");
            ui.separator();
            ui.add_space(20.0);
            
            ui.label("Здесь будет интерфейс для конвертации XML в JSON");
        });
    }
    
    fn show_uuid_generator(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            if ui.button("⬅️ Назад").clicked() {
                self.current_screen = Screen::MainMenu;
            }
            
            ui.add_space(20.0);
            ui.heading("🆔 UUID Generator");
            ui.separator();
            ui.add_space(20.0);
            
            ui.label("Здесь будет интерфейс для генерации UUID");
        });
    }
}