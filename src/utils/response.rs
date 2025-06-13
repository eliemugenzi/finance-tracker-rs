use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse<T: Serialize> {
    pub status: u16,
    pub data: Option<T>,
    pub message: String,
}
