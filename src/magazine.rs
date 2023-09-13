use crate::{adapter::*, holder::*, tool::*};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Magazine {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid, // which machine?
    pub location_slot: usize,    // which magazine number?
    pub tools: Vec<Tool>,
    pub holders: Vec<Holder>,
    pub adapters: Vec<Adapter>,
    pub capacity: usize,
}
