#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub enum HolderCategory {
    #[default]
    Empty,
    Standard,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum Holder {
    StandardHolder(StandardHolder),
    PlaceHolderHolder(PlaceHolderHolder),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct StandardHolder {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: HolderCategory,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct PlaceHolderHolder {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: HolderCategory,
}
