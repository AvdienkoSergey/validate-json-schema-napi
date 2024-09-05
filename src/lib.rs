use napi_derive::napi;
use napi::Error as JsError;
use std::borrow::Cow;
use serde::Deserialize;
use validator::{Validate, ValidationErrors, ValidationErrorsKind};
use once_cell::sync::Lazy;
use regex::Regex;

static RE_ONE_CHARS_0_OR_1_OR_2: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[012]$").unwrap()
});

static RE_START_WITH_1_9: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[1-9]$").unwrap()
});

#[derive(Debug, Validate, Deserialize)]
struct InputData {
    #[validate(
        length(min = 1, max = 1, message="Длина строки должна быть 1 символ"),
        regex(path=*RE_ONE_CHARS_0_OR_1_OR_2, message="Значение может быть только 0, 1 или 2"),
    )]
    #[serde(rename = "type")]
    input_type: Option<String>,

    #[validate(
        length(min = 1, max = 2, message="Длина строки не может быть больше 2 символов"),
        regex(path=*RE_START_WITH_1_9, message="Значение может быть от 1 до 99")
    )]
    location_id: Option<String>,

    #[validate(length(min = 1, message="Длина строки должна быть не меньше 1 символа"))]
    category_id: Option<String>,

    #[validate(length(min = 1, message="Длина строки должна быть не меньше 1 символа"))]
    c_keyword: Option<String>,

    #[validate(length(min = 1, message="Длина строки должна быть не меньше 1 символа"))]
    b_keyword: Option<String>,
}

fn validate_json(json_data: &str) -> Result<(), ValidationErrors> {
    let input_data: InputData = serde_json::from_str(json_data).expect("Invalid JSON data");
    input_data.validate()
}

// Функция для вывода ошибок в кастомизированном формате
fn create_custom_error(errors: ValidationErrors) -> Vec<String> {
    let mut error_messages = Vec::new();
    for (field, error_kind) in errors.into_errors() {
        let field_name = if field == "input_type" { "type" } else { &field };
        match error_kind {
            ValidationErrorsKind::Field(errors) => {
                for error in errors {
                  let message = format!(
                    "Ошибка в поле: {} -> {}",
                    field_name,
                    error.message.unwrap_or_else(|| Cow::from("Неизвестная ошибка".to_string()))
                  );
                  error_messages.push(message);
                }
            }
            _ => {}
        }
    };
    error_messages
}

#[napi]
pub fn validate(json_data: String) -> Result<String, JsError> {
    match validate_json(&json_data) {
        Ok(_) => Ok("JSON is valid!".to_string()),
        Err(e) => Err(JsError::from_reason(format!("{:?}", create_custom_error(e)))),
    }
}

#[napi]
pub fn hello() -> String {
    "Hello from Rust!".to_string()
}