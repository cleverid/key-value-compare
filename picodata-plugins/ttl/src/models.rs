use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub id: String,
    pub region: String,
    pub ids: String,
    pub expire: String,
}
