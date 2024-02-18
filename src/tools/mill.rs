use crate::{tool::*, Drawable, Identifiable, Library, Locatable, PopupState};
use egui::RichText;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Mill {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: ToolCategory,
    pub diameter: f32,
}

impl Mill {
    pub fn get_category(&self) -> ToolCategory {
        self.category.clone()
    }
    pub fn get_location_slot(&self) -> usize {
        self.location_slot
    }
}

impl HasDiameter for Mill {
    fn get_diameter(&self) -> Option<f32> {
        Some(self.diameter)
    }
}

impl HasDegree for Mill {
    fn get_degree(&self) -> Option<f32> {
        None
    }
}

impl Identifiable for Mill {
    fn get_id(&self) -> Uuid {
        self.id
    }
}

impl Locatable for Mill {
    fn set_location_id(&mut self, location_id: Uuid) {
        self.location_id = location_id;
    }

    fn set_location_slot(&mut self, location_slot: usize) {
        self.location_slot = location_slot;
    }
}

impl Drawable for Mill {
    fn draw_display(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Mill").underline().strong());
            ui.horizontal(|ui| {
                ui.label("Name: ");
                ui.label(&self.name);
            });
            ui.horizontal(|ui| {
                ui.label("Diameter: ");
                ui.label(&self.diameter.to_string());
            });
        });
    }
    fn draw_edit(&mut self, ui: &mut egui::Ui) {
        ui.add(egui::TextEdit::singleline(&mut self.name).hint_text("Name"));
        ui.add(egui::Slider::new(&mut self.diameter, 0.0..=100.0).text("Diameter"));
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
                library.tools.push(Tool::Mill(self.clone()));
                *popup_state = PopupState::None;
            }
        });
    }
    fn get_pdf_string(&self) -> Vec<(String, String)> {
        let mut fields: Vec<(String, String)> = Vec::new();
        fields.push(("Name: ".to_string(), self.name.clone()));
        fields.push(("Diameter: ".to_string(), self.diameter.to_string()));
        fields
    }
}
