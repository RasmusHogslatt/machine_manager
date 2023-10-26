use crate::{adapter::*, Drawable, Identifiable, IsPlaceholder, Library, Locatable, PopupState};
use uuid::Uuid;

use super::adapter::Adapter;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct AdapterPlaceHolder {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: AdapterCategory,
}

impl Identifiable for AdapterPlaceHolder {
    fn get_id(&self) -> Uuid {
        self.id
    }
}

impl Locatable for AdapterPlaceHolder {
    fn set_location_id(&mut self, location_id: Uuid) {
        self.location_id = location_id;
    }

    fn set_location_slot(&mut self, location_slot: usize) {
        self.location_slot = location_slot;
    }
}

impl Drawable for AdapterPlaceHolder {
    fn draw_display(&mut self, ui: &mut egui::Ui) {
        ui.label("EMPTY");
    }
    fn draw_edit(&mut self, ui: &mut egui::Ui) {
        ui.label("Place Holder");
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
                library
                    .adapters
                    .push(Adapter::AdapterPlaceHolder(self.clone()));
                *popup_state = PopupState::None;
            }
        });
    }
}

impl IsPlaceholder for AdapterPlaceHolder {
    fn is_placeholder(&self) -> bool {
        true
    }
}
