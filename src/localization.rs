#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    Russian,
    English,
}

impl Default for Language {
    fn default() -> Self {
        Language::Russian
    }
}

#[derive(Clone)]
pub struct Texts {
    // Main menu
    pub app_title: &'static str,
    pub select_utility: &'static str,
    pub xml_to_json_title: &'static str,
    pub xml_to_json_desc: &'static str,
    pub uuid_generator_title: &'static str,
    pub uuid_generator_desc: &'static str,
    pub password_gen_title: &'static str,
    pub password_gen_desc: &'static str,
    pub file_analyzer_title: &'static str,
    pub file_analyzer_desc: &'static str,
    pub coming_soon: &'static str,
    
    // Common
    pub back_button: &'static str,
    pub language: &'static str,
    
    // XML to JSON screen
    pub xml_to_json_heading: &'static str,
    pub xml_to_json_placeholder: &'static str,
    
    // UUID Generator screen
    pub uuid_generator_heading: &'static str,
    pub uuid_generator_placeholder: &'static str,
    pub uuid_version: &'static str,
    pub generate_button: &'static str,
    pub copy_button: &'static str,
    pub generated_uuid: &'static str,
    pub copied_message: &'static str,
    pub uuid_v1_desc: &'static str,
    pub uuid_v4_desc: &'static str,
}

const RUSSIAN_TEXTS: Texts = Texts {
    app_title: "Rust Utilities",
    select_utility: "Выберите утилиту:",
    xml_to_json_title: "XML → JSON",
    xml_to_json_desc: "Конвертация XML в JSON",
    uuid_generator_title: "UUID Generator",
    uuid_generator_desc: "Генерация уникальных ID",
    password_gen_title: "Генератор паролей",
    password_gen_desc: "(Скоро)",
    file_analyzer_title: "Анализатор файлов",
    file_analyzer_desc: "(Скоро)",
    coming_soon: "(Скоро)",
    back_button: "Назад",
    language: "Язык",
    xml_to_json_heading: "XML to JSON Converter",
    xml_to_json_placeholder: "Здесь будет интерфейс для конвертации XML в JSON",
    uuid_generator_heading: "🆔 Генератор UUID",
    uuid_generator_placeholder: "Выберите версию UUID и нажмите 'Генерировать'",
    uuid_version: "Версия UUID:",
    generate_button: "🎲 Генерировать",
    copy_button: "📋 Копировать",
    generated_uuid: "Сгенерированный UUID:",
    copied_message: "✅ Скопировано в буфер обмена!",
    uuid_v1_desc: "V1 (на основе времени и MAC-адреса)",
    uuid_v4_desc: "V4 (случайный)",
};

const ENGLISH_TEXTS: Texts = Texts {
    app_title: "🛠️ Rust Utilities",
    select_utility: "Select a utility:",
    xml_to_json_title: "XML → JSON",
    xml_to_json_desc: "Convert XML to JSON",
    uuid_generator_title: "UUID Generator",
    uuid_generator_desc: "Generate unique IDs",
    password_gen_title: "Password Generator",
    password_gen_desc: "(Coming Soon)",
    file_analyzer_title: "File Analyzer",
    file_analyzer_desc: "(Coming Soon)",
    coming_soon: "(Coming Soon)",
    back_button: "Back",
    language: "Language",
    xml_to_json_heading: "XML to JSON Converter",
    xml_to_json_placeholder: "Here will be the interface for XML to JSON conversion",
    uuid_generator_heading: "🆔 UUID Generator",
    uuid_generator_placeholder: "Select UUID version and click 'Generate'",
    uuid_version: "UUID Version:",
    generate_button: "🎲 Generate",
    copy_button: "📋 Copy",
    generated_uuid: "Generated UUID:",
    copied_message: "✅ Copied to clipboard!",
    uuid_v1_desc: "V1 (time-based with MAC address)",
    uuid_v4_desc: "V4 (random)",
};

pub fn get_texts(language: Language) -> Texts {
    match language {
        Language::Russian => RUSSIAN_TEXTS,
        Language::English => ENGLISH_TEXTS,
    }
}