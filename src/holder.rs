#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub enum HolderCategory {
    #[default]
    Empty,
    ColletHolder,
    EndMillHolder,
    DrillChuckHolder,
    ExternalTurningHolder,
    InternalTurningHolder,
    GroovingPartingHolder,
    ThreadingHolder,
    TappingHolder,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum Holder {
    ColletHolder(ColletHolder),
    EndMillHolder(EndMillHolder),
    DrillChuckHolder(DrillChuckHolder),
    ExternalTurningHolder(ExternalTurningHolder),
    InternalTurningHolder(InternalTurningHolder),
    GroovingPartingHolder(GroovingPartingHolder),
    ThreadingHolder(ThreadingHolder),
    TappingHolder(TappingHolder),
    PlaceHolderHolder(PlaceHolderHolder),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct ColletHolder {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: HolderCategory,
    pub collet_type_name: String,
    pub collet_size: f32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct EndMillHolder {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: HolderCategory,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct DrillChuckHolder {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: HolderCategory,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct ExternalTurningHolder {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: HolderCategory,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct InternalTurningHolder {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: HolderCategory,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct GroovingPartingHolder {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: HolderCategory,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct ThreadingHolder {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: HolderCategory,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct TappingHolder {
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
