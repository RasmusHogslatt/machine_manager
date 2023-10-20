use crate::{adapter::*, Drawable, Identifiable, Library, Locatable, PopupState};
use egui::RichText;
use uuid::Uuid;

use super::adapter::Adapter;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct ShrinkFit {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: AdapterCategory,
}

impl Identifiable for ShrinkFit {
    fn get_id(&self) -> Uuid {
        self.id
    }
}

impl Locatable for ShrinkFit {
    fn set_location_id(&mut self, location_id: Uuid) {
        self.location_id = location_id;
    }

    fn set_location_slot(&mut self, location_slot: usize) {
        self.location_slot = location_slot;
    }
}

impl Drawable for ShrinkFit {
    fn draw_display(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Shrink Fit").underline().strong());
            ui.horizontal(|ui| {
                ui.label("Name: ");
                ui.label(&self.name);
            });
        });
    }
    fn draw_edit(&mut self, ui: &mut egui::Ui) {
        ui.label("Shrink Fit");
        ui.add(egui::TextEdit::singleline(&mut self.name).hint_text("Name"));
    }
    fn draw_adding_to_library(
        &mut self,
        library: &mut Library,
        popup_state: &mut PopupState,
        ui: &mut egui::Ui,
    ) {
        self.draw_edit(ui);

        ui.horizontal(|ui| {
            if ui.button("Cancel").clicked() {
                *popup_state = PopupState::None;
            }
            if ui.button("Save").clicked() {
                self.location_id = library.id;
                self.location_slot = library.tools.len();
                library.adapters.push(Adapter::ShrinkFit(self.clone()));
                *popup_state = PopupState::None;
            }
        });
    }
}
