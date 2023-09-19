use crate::{Drawable, GuiResource, Library, PopupState};

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

pub fn add_adapter(
    gui_resource: &mut GuiResource,
    library: &mut Library,
    popup_state: &mut PopupState,
    ctx: &egui::Context,
) {
    if popup_state != &PopupState::AddAdapter {
        return;
    }
    egui::Window::new("Add adapter").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.radio_value(
                &mut gui_resource.adapter_category,
                AdapterCategory::Standard,
                "Standard",
            );
        });
        match gui_resource.adapter_category {
            AdapterCategory::Empty => {}
            AdapterCategory::Standard => {
                ui.horizontal(|ui| {
                    ui.radio_value(
                        &mut gui_resource.adapter_selected,
                        Adapter::ShrinkFitAdapter(Default::default()),
                        "ShrinkFitAdapter",
                    );
                    ui.radio_value(
                        &mut gui_resource.adapter_selected,
                        Adapter::SideLockAdapter(Default::default()),
                        "SideLockAdapter",
                    );
                    ui.radio_value(
                        &mut gui_resource.adapter_selected,
                        Adapter::HydraulicAdapter(Default::default()),
                        "HydraulicAdapter",
                    );
                });
            }
        }
        match (
            &gui_resource.adapter_selected,
            &gui_resource.adapter_category,
        ) {
            (Adapter::ShrinkFitAdapter(_), AdapterCategory::Standard) => {
                gui_resource
                    .shrink_fit_adapter
                    .draw_adding_to_library(library, popup_state, ui);
            }
            (Adapter::SideLockAdapter(_), AdapterCategory::Standard) => {
                gui_resource
                    .side_lock_adapter
                    .draw_adding_to_library(library, popup_state, ui);
            }
            (Adapter::HydraulicAdapter(_), AdapterCategory::Standard) => {
                gui_resource
                    .hydraulic_adapter
                    .draw_adding_to_library(library, popup_state, ui);
            }
            (_, _) => {}
        }
    });
}
