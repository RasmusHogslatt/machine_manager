use crate::{adapter::*, holder::*, tool::*};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Library {
    pub name: String,
    pub id: uuid::Uuid,
    pub tools: Vec<Tool>,
    pub holders: Vec<Holder>,
    pub adapters: Vec<Adapter>,
}

impl Default for Library {
    fn default() -> Self {
        Library {
            name: "Library".to_string(),
            id: uuid::Uuid::new_v4(),
            tools: Vec::new(),
            holders: Vec::new(),
            adapters: Vec::new(),
        }
    }
}
