use serde::Serialize;

#[derive(Serialize)]
pub struct HelloMessage {
    pub message: String
}
