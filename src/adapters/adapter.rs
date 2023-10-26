use uuid::Uuid;

use crate::{
    adapter_placeholder::*, hydraulic::*, shrink_fit::*, side_lock::*, Drawable, Identifiable,
    IsPlaceholder, Library, Locatable, PopupState,
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub enum AdapterCategory {
    #[default]
    Empty,
    Standard,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Adapter {
    ShrinkFit(ShrinkFit),
    SideLock(SideLock),
    Hydraulic(Hydraulic),
    AdapterPlaceHolder(AdapterPlaceHolder),
}

impl Identifiable for Adapter {
    fn get_id(&self) -> Uuid {
        match self {
            Adapter::ShrinkFit(adapter) => adapter.get_id(),
            Adapter::SideLock(adapter) => adapter.get_id(),
            Adapter::Hydraulic(adapter) => adapter.get_id(),
            Adapter::AdapterPlaceHolder(adapter) => adapter.get_id(),
        }
    }
}

impl Locatable for Adapter {
    fn set_location_id(&mut self, location_id: Uuid) {
        match self {
            Adapter::ShrinkFit(adapter) => adapter.set_location_id(location_id),
            Adapter::SideLock(adapter) => adapter.set_location_id(location_id),
            Adapter::Hydraulic(adapter) => adapter.set_location_id(location_id),
            Adapter::AdapterPlaceHolder(adapter) => adapter.set_location_id(location_id),
        }
    }
    fn set_location_slot(&mut self, location_slot: usize) {
        match self {
            Adapter::ShrinkFit(adapter) => adapter.set_location_slot(location_slot),
            Adapter::SideLock(adapter) => adapter.set_location_slot(location_slot),
            Adapter::Hydraulic(adapter) => adapter.set_location_slot(location_slot),
            Adapter::AdapterPlaceHolder(adapter) => adapter.set_location_slot(location_slot),
        }
    }
}

impl Drawable for Adapter {
    fn draw_display(&mut self, ui: &mut egui::Ui) {
        match self {
            Adapter::ShrinkFit(adapter) => adapter.draw_display(ui),
            Adapter::SideLock(adapter) => adapter.draw_display(ui),
            Adapter::Hydraulic(adapter) => adapter.draw_display(ui),
            Adapter::AdapterPlaceHolder(adapter) => adapter.draw_display(ui),
        }
    }

    fn draw_edit(&mut self, ui: &mut egui::Ui) {
        match self {
            Adapter::ShrinkFit(adapter) => adapter.draw_edit(ui),
            Adapter::SideLock(adapter) => adapter.draw_edit(ui),
            Adapter::Hydraulic(adapter) => adapter.draw_edit(ui),
            Adapter::AdapterPlaceHolder(adapter) => adapter.draw_edit(ui),
        }
    }

    fn draw_adding_to_library(
        &mut self,
        library: &mut Library,
        popup_state: &mut PopupState,
        ui: &mut egui::Ui,
    ) {
        match self {
            Adapter::ShrinkFit(adapter) => adapter.draw_adding_to_library(library, popup_state, ui),
            Adapter::SideLock(adapter) => adapter.draw_adding_to_library(library, popup_state, ui),
            Adapter::Hydraulic(adapter) => adapter.draw_adding_to_library(library, popup_state, ui),
            Adapter::AdapterPlaceHolder(adapter) => {
                adapter.draw_adding_to_library(library, popup_state, ui)
            }
        }
    }
}

impl IsPlaceholder for Adapter {
    fn is_placeholder(&self) -> bool {
        match self {
            Adapter::AdapterPlaceHolder(adapter) => adapter.is_placeholder(),
            _ => false,
        }
    }
}
