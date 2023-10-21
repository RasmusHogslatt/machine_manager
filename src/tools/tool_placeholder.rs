use crate::{tool::*, Drawable, Identifiable, IsPlaceholder, Locatable, PopupState};
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct ToolPlaceHolder {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: ToolCategory,
}

impl Identifiable for ToolPlaceHolder {
    fn get_id(&self) -> Uuid {
        self.id
    }
}

impl Locatable for ToolPlaceHolder {
    fn set_location_id(&mut self, location_id: Uuid) {
        self.location_id = location_id;
    }

    fn set_location_slot(&mut self, location_slot: usize) {
        self.location_slot = location_slot;
    }
}

impl Drawable for ToolPlaceHolder {
    fn draw_display(&mut self, ui: &mut egui::Ui) {
        ui.label("EMPTY");
    }
    fn draw_edit(&mut self, _ui: &mut egui::Ui) {}
    fn draw_adding_to_library(
        &mut self,
        _library: &mut crate::Library,
        _popup_state: &mut PopupState,
        _ui: &mut egui::Ui,
    ) {
    }
}

impl IsPlaceholder for ToolPlaceHolder {
    fn is_placeholder(&self) -> bool {
        true
    }
}
