use crate::{library::*, states::*};

pub trait Drawable {
    fn draw_display(&mut self, ui: &mut egui::Ui);
    fn draw_edit(&mut self, ui: &mut egui::Ui);
    fn draw_adding_to_library(
        &mut self,
        library: &mut Library,
        popup_state: &mut PopupState,
        ui: &mut egui::Ui,
    );
    // Returns first string as type of field, i.e. "Name: ". Second field is value, i.e. "MyName"
    fn get_pdf_string(&self) -> Vec<(String, String)>;
}
