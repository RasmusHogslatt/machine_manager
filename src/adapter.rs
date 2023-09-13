#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub enum AdapterCategory {
    #[default]
    Empty,
    Standard,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum Adapter {
    StandardAdapter(StandardAdapter),
    PlaceHolderAdapter(PlaceHolderAdapter),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct StandardAdapter {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: AdapterCategory,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct PlaceHolderAdapter {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: AdapterCategory,
}
