use crate::{tool::*, Drawable, Identifiable, Library, Locatable, PopupState};
use egui::RichText;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq)]
pub struct SquareInsert {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: ToolCategory,
    pub degree: f32,
}

impl Identifiable for SquareInsert {
    fn get_id(&self) -> Uuid {
        self.id
    }
}

impl Locatable for SquareInsert {
    fn set_location_id(&mut self, location_id: Uuid) {
        self.location_id = location_id;
    }

    fn set_location_slot(&mut self, location_slot: usize) {
        self.location_slot = location_slot;
    }
}

impl Drawable for SquareInsert {
    fn draw_display(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Square Insert").underline().strong());
            ui.horizontal(|ui| {
                ui.label("Name: ");
                ui.label(&self.name);
            });
            ui.horizontal(|ui| {
                ui.label("Degree: ");
                ui.label(&self.degree.to_string());
            });
        });
    }
    fn draw_edit(&mut self, ui: &mut egui::Ui) {
        ui.heading("Square Insert");
        ui.add(egui::TextEdit::singleline(&mut self.name).hint_text("Name"));
        ui.add(egui::Slider::new(&mut self.degree, 0.0..=100.0).text("Degree"));
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
                library.tools.push(Tool::SquareInsert(self.clone()));
                *popup_state = PopupState::None;
            }
        });
    }
}
