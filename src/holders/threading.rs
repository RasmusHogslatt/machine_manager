use super::holder::*;
use crate::{Drawable, Identifiable, Library, Locatable, PopupState};
use egui::RichText;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct Threading {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: HolderCategory,
}

impl Identifiable for Threading {
    fn get_id(&self) -> Uuid {
        self.id
    }
}

impl Locatable for Threading {
    fn set_location_id(&mut self, location_id: Uuid) {
        self.location_id = location_id;
    }

    fn set_location_slot(&mut self, location_slot: usize) {
        self.location_slot = location_slot;
    }
}

impl Drawable for Threading {
    fn draw_display(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Drill Chuck").underline().strong());
            ui.horizontal(|ui| {
                ui.label("Name: ");
                ui.label(&self.name);
            });
        });
    }
    fn draw_edit(&mut self, ui: &mut egui::Ui) {
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
                library.holders.push(Holder::Threading(self.clone()));
                *popup_state = PopupState::None;
            }
        });
    }
    fn get_pdf_string(&self) -> Vec<(String, String)> {
        let mut fields: Vec<(String, String)> = Vec::new();
        fields.push(("Name: ".to_string(), self.name.clone()));
        fields
    }
}
