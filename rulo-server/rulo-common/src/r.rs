use serde::Serialize;

#[derive(Serialize)]
pub struct R<T> {
    code: i32,
    data: T,
    message: String,
}

impl R<T> {
    pub fn ok(data: T) -> R<T> {
        R {
            code: 200,
            data: data,
            message: "success".to_string(),
        }
    }
}
