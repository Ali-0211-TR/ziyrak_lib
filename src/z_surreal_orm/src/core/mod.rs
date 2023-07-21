//! Модуль для функциональности обработки Vec, JSON, Map, String.
//! Оно очень важно для форматирование всех `z_surreal_orm::sql` вырожений
//! Для парсера строк и конвертации типов
//! 


pub(crate) mod generators;
pub(crate) mod separators;

pub use separators::json_to_map;
pub use separators::obj_to_string;
pub use separators::separate_fields;
pub use separators::separate_values;
pub use separators::separate_fields_and_values;
pub use separators::vec_to_string_without_scope;


#[cfg(test)]
mod tests{
    #[derive(Debug, Default, serde::Serialize)]
        struct TestPerson {
            name: String,
            phone: String,
            age: u32,
        }
       
   
}