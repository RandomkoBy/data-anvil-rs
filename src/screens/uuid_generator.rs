use egui;
use super::Screen;
use crate::localization::Texts;
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq)]
enum UuidVersion {
    V1,
    V4,
}

impl Default for UuidVersion {
    fn default() -> Self {
        UuidVersion::V4
    }
}

#[derive(Default)]
pub struct UuidGeneratorState {
    selected_version: UuidVersion,
    generated_uuid: String,
    show_copied_message: bool,
    copied_message_timer: f32,
}

static mut UUID_STATE: Option<UuidGeneratorState> = None;

fn get_state() -> &'static mut UuidGeneratorState {
    unsafe {
        if UUID_STATE.is_none() {
            UUID_STATE = Some(UuidGeneratorState::default());
        }
        UUID_STATE.as_mut().unwrap()
    }
}

pub fn show(ui: &mut egui::Ui, current_screen: &mut Screen, texts: &Texts) {
    let state = get_state();
    
    ui.vertical(|ui| {
        // Back button
        if ui.button(texts.back_button).clicked() {
            *current_screen = Screen::MainMenu;
        }
        
        ui.add_space(20.0);
        ui.heading(texts.uuid_generator_heading);
        ui.separator();
        ui.add_space(30.0);
        
        // UUID Version selector
        ui.horizontal(|ui| {
            ui.label(texts.uuid_version);
            ui.add_space(10.0);
            
            egui::ComboBox::from_id_source("uuid_version_selector")
                .selected_text(match state.selected_version {
                    UuidVersion::V1 => texts.uuid_v1_desc,
                    UuidVersion::V4 => texts.uuid_v4_desc,
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut state.selected_version, UuidVersion::V1, texts.uuid_v1_desc);
                    ui.selectable_value(&mut state.selected_version, UuidVersion::V4, texts.uuid_v4_desc);
                });
        });
        
        ui.add_space(20.0);
        
        // Generate button
        if ui.add_sized([150.0, 40.0], egui::Button::new(texts.generate_button)).clicked() {
            state.generated_uuid = match state.selected_version {
                UuidVersion::V1 => {
                    // For V1, we'll use a simple time-based approach
                    // Note: Real V1 UUID requires MAC address, this is a simplified version
                    let timestamp = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_nanos();
                    
                    // Create a pseudo-V1 UUID using timestamp
                    format!(
                        "{:08x}-{:04x}-1{:03x}-{:04x}-{:012x}",
                        (timestamp & 0xffffffff) as u32,
                        ((timestamp >> 32) & 0xffff) as u16,
                        ((timestamp >> 48) & 0xfff) as u16,
                        (timestamp & 0xffff) as u16,
                        (timestamp >> 16) & 0xffffffffffff
                    )
                },
                UuidVersion::V4 => Uuid::new_v4().to_string(),
            };
            state.show_copied_message = false;
        }
        
        ui.add_space(20.0);
        
        // Generated UUID display
        if !state.generated_uuid.is_empty() {
            ui.label(texts.generated_uuid);
            ui.add_space(5.0);
            
            ui.horizontal(|ui| {
                // UUID text field (read-only)
                let mut uuid_text = state.generated_uuid.clone();
                ui.add_sized(
                    [400.0, 25.0],
                    egui::TextEdit::singleline(&mut uuid_text)
                        .desired_width(400.0)
                        .interactive(false)
                );
                
                // Copy button
                if ui.button(texts.copy_button).clicked() {
                    ui.output_mut(|o| o.copied_text = state.generated_uuid.clone());
                    state.show_copied_message = true;
                    state.copied_message_timer = 3.0; // Show message for 3 seconds
                }
            });
            
            ui.add_space(10.0);
            
            // Copied message
            if state.show_copied_message {
                ui.colored_label(egui::Color32::GREEN, texts.copied_message);
                
                // Timer countdown
                state.copied_message_timer -= ui.input(|i| i.unstable_dt);
                if state.copied_message_timer <= 0.0 {
                    state.show_copied_message = false;
                }
                
                // Request repaint to update timer
                ui.ctx().request_repaint();
            }
        } else {
            ui.label(texts.uuid_generator_placeholder);
        }
        
        ui.add_space(20.0);
        
        // Info section
        ui.separator();
        ui.add_space(10.0);
        
        match state.selected_version {
            UuidVersion::V1 => {
                ui.label("ℹ️ UUID v1 генерируется на основе текущего времени.");
                ui.label("   Подходит для случаев, когда нужна временная упорядоченность.");
            },
            UuidVersion::V4 => {
                ui.label("ℹ️ UUID v4 генерируется случайным образом.");
                ui.label("   Наиболее распространенный тип UUID.");
            },
        }
    });
}