pub mod main_menu;
pub mod xml_to_json;
pub mod uuid_generator;

#[derive(Default, PartialEq)]
pub enum Screen {
    #[default]
    MainMenu,
    XmlToJson,
    UuidGenerator,
    // Добавим больше утилит позже
}