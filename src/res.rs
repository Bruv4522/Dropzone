use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Res {
    pub message: String
}