use crate::{adapter::*, holder::*, tool::*};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Magazine {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid, // which machine?
    pub location_slot: usize,    // which magazine number?
    pub contents: Vec<(Tool, Holder, Adapter)>,
    pub capacity: usize,
}
