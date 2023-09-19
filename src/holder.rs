use crate::{Drawable, GuiResource, Library, PopupState};

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

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ColletHolder {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: HolderCategory,
    pub collet_type_name: String,
    pub collet_size: f32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq)]
pub struct EndMillHolder {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: HolderCategory,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq)]
pub struct DrillChuckHolder {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: HolderCategory,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ExternalTurningHolder {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: HolderCategory,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq)]
pub struct InternalTurningHolder {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: HolderCategory,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq)]
pub struct GroovingPartingHolder {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: HolderCategory,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ThreadingHolder {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: HolderCategory,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq)]
pub struct TappingHolder {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: HolderCategory,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq)]
pub struct PlaceHolderHolder {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: HolderCategory,
}

// Add holder to library's 'tools' vector
pub fn add_holder(
    gui_resource: &mut GuiResource,
    library: &mut Library,
    popup_state: &mut PopupState,
    ctx: &egui::Context,
) {
    if popup_state != &PopupState::AddHolder {
        return;
    }
    egui::Window::new("Add holder").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.radio_value(
                &mut gui_resource.holder_category,
                HolderCategory::MillingHolder,
                "Milling",
            );
            ui.radio_value(
                &mut gui_resource.holder_category,
                HolderCategory::DrillingHolder,
                "Drilling",
            );
            ui.radio_value(
                &mut gui_resource.holder_category,
                HolderCategory::TurningHolder,
                "Turning",
            );
            ui.radio_value(
                &mut gui_resource.holder_category,
                HolderCategory::SpecialtyHolder,
                "Specialty",
            );
        });
        match gui_resource.holder_category {
            HolderCategory::Empty => {}
            HolderCategory::MillingHolder => {
                ui.horizontal(|ui| {
                    ui.radio_value(
                        &mut gui_resource.holder_selected,
                        Holder::ColletHolder(ColletHolder::default()),
                        "Collet Holder",
                    );
                    ui.radio_value(
                        &mut gui_resource.holder_selected,
                        Holder::EndMillHolder(EndMillHolder::default()),
                        "End Mill Holder",
                    );
                });
            }
            HolderCategory::DrillingHolder => {
                ui.horizontal(|ui| {
                    ui.radio_value(
                        &mut gui_resource.holder_selected,
                        Holder::DrillChuckHolder(DrillChuckHolder::default()),
                        "Drill Chuck Holder",
                    );
                });
            }
            HolderCategory::TurningHolder => {
                ui.horizontal(|ui| {
                    ui.radio_value(
                        &mut gui_resource.holder_selected,
                        Holder::ExternalTurningHolder(ExternalTurningHolder::default()),
                        "External Turning Holder",
                    );
                    ui.radio_value(
                        &mut gui_resource.holder_selected,
                        Holder::InternalTurningHolder(InternalTurningHolder::default()),
                        "Internal Turning Holder",
                    );
                });
            }
            HolderCategory::SpecialtyHolder => {
                ui.horizontal(|ui| {
                    ui.radio_value(
                        &mut gui_resource.holder_selected,
                        Holder::GroovingPartingHolder(GroovingPartingHolder::default()),
                        "Grooving/Parting Holder",
                    );
                    ui.radio_value(
                        &mut gui_resource.holder_selected,
                        Holder::ThreadingHolder(ThreadingHolder::default()),
                        "Threading Holder",
                    );
                    ui.radio_value(
                        &mut gui_resource.holder_selected,
                        Holder::TappingHolder(TappingHolder::default()),
                        "Tapping Holder",
                    );
                });
            }
        }
        match (&gui_resource.holder_selected, &gui_resource.holder_category) {
            (Holder::ColletHolder(_), HolderCategory::MillingHolder) => {
                gui_resource
                    .collet_holder
                    .draw_adding_to_library(library, popup_state, ui);
            }

            (Holder::EndMillHolder(_), HolderCategory::MillingHolder) => {
                gui_resource
                    .end_mill_holder
                    .draw_adding_to_library(library, popup_state, ui);
            }

            (Holder::DrillChuckHolder(_), HolderCategory::DrillingHolder) => {
                gui_resource
                    .drill_chuck_holder
                    .draw_adding_to_library(library, popup_state, ui);
            }

            (Holder::ExternalTurningHolder(_), HolderCategory::TurningHolder) => {
                gui_resource.external_turning_holder.draw_adding_to_library(
                    library,
                    popup_state,
                    ui,
                );
            }

            (Holder::InternalTurningHolder(_), HolderCategory::TurningHolder) => {
                gui_resource.internal_turning_holder.draw_adding_to_library(
                    library,
                    popup_state,
                    ui,
                );
            }

            (Holder::GroovingPartingHolder(_), HolderCategory::SpecialtyHolder) => {
                gui_resource.grooving_parting_holder.draw_adding_to_library(
                    library,
                    popup_state,
                    ui,
                );
            }

            (Holder::ThreadingHolder(_), HolderCategory::SpecialtyHolder) => {
                gui_resource
                    .threading_holder
                    .draw_adding_to_library(library, popup_state, ui);
            }

            (_, _) => {}
        }
    });
}
