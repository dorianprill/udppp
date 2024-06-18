use serde::{Serialize, Deserialize};

pub trait State {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Message {
    Data { sequence: usize, payload: Vec<u8> },
    Ack,
    Nack,
}

impl Message {
    pub fn is_valid(&self) -> bool {
        // Implement validation logic here, e.g., checksum verification
        true
    }
}
