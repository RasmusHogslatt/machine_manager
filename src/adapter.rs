#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub enum AdapterCategory {
    #[default]
    Empty,
    Standard,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum Adapter {
    ShrinkFitAdapter(ShrinkFitAdapter),
    SideLockAdapter(SideLockAdapter),
    HydraulicAdapter(HydraulicAdapter),
    PlaceHolderAdapter(PlaceHolderAdapter),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ShrinkFitAdapter {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: AdapterCategory,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq)]
pub struct SideLockAdapter {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: AdapterCategory,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq)]
pub struct HydraulicAdapter {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: AdapterCategory,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq)]
pub struct PlaceHolderAdapter {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: AdapterCategory,
}
