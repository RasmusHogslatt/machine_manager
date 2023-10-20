use uuid::Uuid;

pub trait Identifiable {
    fn get_id(&self) -> Uuid;
}

pub trait Locatable {
    fn set_location_id(&mut self, location_id: Uuid);
    fn set_location_slot(&mut self, location_slot: usize);
}

pub trait IsPlaceholder {
    fn is_placeholder(&self) -> bool;
}
