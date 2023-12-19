use uuid::Uuid;

use crate::{
    collet::*, drill_chuck::*, end_mill::*, external_turning::*, grooving_parting::*,
    holder_placeholder::*, internal_turning::*, tapping::*, threading::*, Drawable, Identifiable,
    IsPlaceholder, Library, Locatable, PopupState,
};
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub enum HolderCategory {
    #[default]
    Empty,
    MillingHolder,
    DrillingHolder,
    TurningHolder,
    SpecialtyHolder,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum Holder {
    Collet(Collet),
    EndMill(EndMill),
    DrillChuck(DrillChuck),
    ExternalTurning(ExternalTurning),
    InternalTurning(InternalTurning),
    GroovingParting(GroovingParting),
    Threading(Threading),
    Tapping(Tapping),
    HolderPlaceHolder(HolderPlaceHolder),
}

impl Identifiable for Holder {
    fn get_id(&self) -> uuid::Uuid {
        match self {
            Holder::Collet(holder) => holder.get_id(),
            Holder::EndMill(holder) => holder.get_id(),
            Holder::DrillChuck(holder) => holder.get_id(),
            Holder::ExternalTurning(holder) => holder.get_id(),
            Holder::InternalTurning(holder) => holder.get_id(),
            Holder::GroovingParting(holder) => holder.get_id(),
            Holder::Threading(holder) => holder.get_id(),
            Holder::Tapping(holder) => holder.get_id(),
            Holder::HolderPlaceHolder(holder) => holder.get_id(),
        }
    }
}

impl Locatable for Holder {
    fn set_location_id(&mut self, location_id: Uuid) {
        match self {
            Holder::Collet(holder) => holder.set_location_id(location_id),
            Holder::EndMill(holder) => holder.set_location_id(location_id),
            Holder::DrillChuck(holder) => holder.set_location_id(location_id),
            Holder::ExternalTurning(holder) => holder.set_location_id(location_id),
            Holder::InternalTurning(holder) => holder.set_location_id(location_id),
            Holder::GroovingParting(holder) => holder.set_location_id(location_id),
            Holder::Threading(holder) => holder.set_location_id(location_id),
            Holder::Tapping(holder) => holder.set_location_id(location_id),
            Holder::HolderPlaceHolder(holder) => holder.set_location_id(location_id),
        }
    }
    fn set_location_slot(&mut self, location_slot: usize) {
        match self {
            Holder::Collet(holder) => holder.set_location_slot(location_slot),
            Holder::EndMill(holder) => holder.set_location_slot(location_slot),
            Holder::DrillChuck(holder) => holder.set_location_slot(location_slot),
            Holder::ExternalTurning(holder) => holder.set_location_slot(location_slot),
            Holder::InternalTurning(holder) => holder.set_location_slot(location_slot),
            Holder::GroovingParting(holder) => holder.set_location_slot(location_slot),
            Holder::Threading(holder) => holder.set_location_slot(location_slot),
            Holder::Tapping(holder) => holder.set_location_slot(location_slot),
            Holder::HolderPlaceHolder(holder) => holder.set_location_slot(location_slot),
        }
    }
}
impl Drawable for Holder {
    fn draw_display(&mut self, ui: &mut egui::Ui) {
        match self {
            Holder::Collet(collet_holder) => collet_holder.draw_display(ui),
            Holder::EndMill(end_mill_holder) => end_mill_holder.draw_display(ui),
            Holder::DrillChuck(drill_chuck_holder) => drill_chuck_holder.draw_display(ui),
            Holder::ExternalTurning(external_turning_holder) => {
                external_turning_holder.draw_display(ui)
            }
            Holder::InternalTurning(internal_turning_holder) => {
                internal_turning_holder.draw_display(ui)
            }
            Holder::GroovingParting(grooving_parting_holder) => {
                grooving_parting_holder.draw_display(ui)
            }
            Holder::Threading(threading_holder) => threading_holder.draw_display(ui),
            Holder::Tapping(tapping_holder) => tapping_holder.draw_display(ui),
            Holder::HolderPlaceHolder(place_holder_holder) => place_holder_holder.draw_display(ui),
        }
    }
    fn draw_edit(&mut self, ui: &mut egui::Ui) {
        match self {
            Holder::Collet(collet_holder) => collet_holder.draw_edit(ui),
            Holder::EndMill(end_mill_holder) => end_mill_holder.draw_edit(ui),
            Holder::DrillChuck(drill_chuck_holder) => drill_chuck_holder.draw_edit(ui),
            Holder::ExternalTurning(external_turning_holder) => {
                external_turning_holder.draw_edit(ui)
            }
            Holder::InternalTurning(internal_turning_holder) => {
                internal_turning_holder.draw_edit(ui)
            }
            Holder::GroovingParting(grooving_parting_holder) => {
                grooving_parting_holder.draw_edit(ui)
            }
            Holder::Threading(threading_holder) => threading_holder.draw_edit(ui),
            Holder::Tapping(tapping_holder) => tapping_holder.draw_edit(ui),
            Holder::HolderPlaceHolder(place_holder_holder) => place_holder_holder.draw_edit(ui),
        }
    }

    fn draw_adding_to_library(
        &mut self,
        library: &mut Library,
        popup_state: &mut PopupState,
        ui: &mut egui::Ui,
    ) {
        match self {
            Holder::Collet(collet_holder) => {
                collet_holder.draw_adding_to_library(library, popup_state, ui)
            }
            Holder::EndMill(end_mill_holder) => {
                end_mill_holder.draw_adding_to_library(library, popup_state, ui)
            }
            Holder::DrillChuck(drill_chuck_holder) => {
                drill_chuck_holder.draw_adding_to_library(library, popup_state, ui)
            }
            Holder::ExternalTurning(external_turning_holder) => {
                external_turning_holder.draw_adding_to_library(library, popup_state, ui)
            }
            Holder::InternalTurning(internal_turning_holder) => {
                internal_turning_holder.draw_adding_to_library(library, popup_state, ui)
            }
            Holder::GroovingParting(grooving_parting_holder) => {
                grooving_parting_holder.draw_adding_to_library(library, popup_state, ui)
            }
            Holder::Threading(threading_holder) => {
                threading_holder.draw_adding_to_library(library, popup_state, ui)
            }
            Holder::Tapping(tapping_holder) => {
                tapping_holder.draw_adding_to_library(library, popup_state, ui)
            }
            Holder::HolderPlaceHolder(place_holder_holder) => {
                place_holder_holder.draw_adding_to_library(library, popup_state, ui)
            }
        }
    }
    fn get_pdf_string(&self) -> (String, usize) {
        match self {
            Holder::Collet(collet_holder) => collet_holder.get_pdf_string(),
            Holder::EndMill(end_mill_holder) => end_mill_holder.get_pdf_string(),
            Holder::DrillChuck(drill_chuck_holder) => drill_chuck_holder.get_pdf_string(),
            Holder::ExternalTurning(external_turning_holder) => {
                external_turning_holder.get_pdf_string()
            }
            Holder::InternalTurning(internal_turning_holder) => {
                internal_turning_holder.get_pdf_string()
            }
            Holder::GroovingParting(grooving_parting_holder) => {
                grooving_parting_holder.get_pdf_string()
            }
            Holder::Threading(threading_holder) => threading_holder.get_pdf_string(),
            Holder::Tapping(tapping_holder) => tapping_holder.get_pdf_string(),
            Holder::HolderPlaceHolder(place_holder_holder) => place_holder_holder.get_pdf_string(),
        }
    }
}

impl IsPlaceholder for Holder {
    fn is_placeholder(&self) -> bool {
        match self {
            Holder::HolderPlaceHolder(place_holder_holder) => place_holder_holder.is_placeholder(),
            _ => false,
        }
    }
}
