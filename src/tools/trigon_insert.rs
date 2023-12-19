use crate::{tool::*, Drawable, Identifiable, Library, Locatable, PopupState};
use egui::RichText;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq)]
pub struct TrigonInsert {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: ToolCategory,
    pub degree: f32,
}

impl Identifiable for TrigonInsert {
    fn get_id(&self) -> Uuid {
        self.id
    }
}

impl Locatable for TrigonInsert {
    fn set_location_id(&mut self, location_id: Uuid) {
        self.location_id = location_id;
    }

    fn set_location_slot(&mut self, location_slot: usize) {
        self.location_slot = location_slot;
    }
}

impl Drawable for TrigonInsert {
    fn draw_display(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Trigon Insert").underline().strong());
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
        ui.heading("Trigon Insert");
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
                library.tools.push(Tool::TrigonInsert(self.clone()));
                *popup_state = PopupState::None;
            }
        });
    }
    fn get_pdf_string(&self) -> (String, usize) {
        let n_lines = 2;
        let n_lines = 2;
        let string = format!("Name: {}\nDegree: {}", self.name, self.degree);
        (string, n_lines)
    }
}
