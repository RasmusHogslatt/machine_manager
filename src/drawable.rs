use egui::*;

use crate::{adapter::*, library::*, states::*};

pub trait Drawable {
    fn draw_display(&mut self, ui: &mut egui::Ui);
    fn draw_edit(&mut self, ui: &mut egui::Ui);
    fn draw_adding_to_library(
        &mut self,
        library: &mut Library,
        popup_state: &mut PopupState,
        ui: &mut egui::Ui,
    );
}

// impl Drawable for Holder {
//     fn draw_display(&mut self, ui: &mut egui::Ui) {
//         match self {
//             Holder::Collet(collet_holder) => collet_holder.draw_display(ui),
//             Holder::EndMill(end_mill_holder) => end_mill_holder.draw_display(ui),
//             Holder::DrillChuck(drill_chuck_holder) => drill_chuck_holder.draw_display(ui),
//             Holder::ExternalTurning(external_turning_holder) => {
//                 external_turning_holder.draw_display(ui)
//             }
//             Holder::InternalTurning(internal_turning_holder) => {
//                 internal_turning_holder.draw_display(ui)
//             }
//             Holder::GroovingParting(grooving_parting_holder) => {
//                 grooving_parting_holder.draw_display(ui)
//             }
//             Holder::Threading(threading_holder) => threading_holder.draw_display(ui),
//             Holder::Tapping(tapping_holder) => tapping_holder.draw_display(ui),
//             Holder::HolderPlaceHolder(place_holder_holder) => place_holder_holder.draw_display(ui),
//         }
//     }
//     fn draw_edit(&mut self, ui: &mut egui::Ui) {
//         match self {
//             Holder::Collet(collet_holder) => collet_holder.draw_edit(ui),
//             Holder::EndMill(end_mill_holder) => end_mill_holder.draw_edit(ui),
//             Holder::DrillChuck(drill_chuck_holder) => drill_chuck_holder.draw_edit(ui),
//             Holder::ExternalTurning(external_turning_holder) => {
//                 external_turning_holder.draw_edit(ui)
//             }
//             Holder::InternalTurning(internal_turning_holder) => {
//                 internal_turning_holder.draw_edit(ui)
//             }
//             Holder::GroovingParting(grooving_parting_holder) => {
//                 grooving_parting_holder.draw_edit(ui)
//             }
//             Holder::Threading(threading_holder) => threading_holder.draw_edit(ui),
//             Holder::Tapping(tapping_holder) => tapping_holder.draw_edit(ui),
//             Holder::HolderPlaceHolder(place_holder_holder) => place_holder_holder.draw_edit(ui),
//         }
//     }

//     fn draw_adding_to_library(
//         &mut self,
//         library: &mut Library,
//         popup_state: &mut PopupState,
//         ui: &mut egui::Ui,
//     ) {
//         match self {
//             Holder::Collet(collet_holder) => {
//                 collet_holder.draw_adding_to_library(library, popup_state, ui)
//             }
//             Holder::EndMill(end_mill_holder) => {
//                 end_mill_holder.draw_adding_to_library(library, popup_state, ui)
//             }
//             Holder::DrillChuck(drill_chuck_holder) => {
//                 drill_chuck_holder.draw_adding_to_library(library, popup_state, ui)
//             }
//             Holder::ExternalTurning(external_turning_holder) => {
//                 external_turning_holder.draw_adding_to_library(library, popup_state, ui)
//             }
//             Holder::InternalTurning(internal_turning_holder) => {
//                 internal_turning_holder.draw_adding_to_library(library, popup_state, ui)
//             }
//             Holder::GroovingParting(grooving_parting_holder) => {
//                 grooving_parting_holder.draw_adding_to_library(library, popup_state, ui)
//             }
//             Holder::Threading(threading_holder) => {
//                 threading_holder.draw_adding_to_library(library, popup_state, ui)
//             }
//             Holder::Tapping(tapping_holder) => {
//                 tapping_holder.draw_adding_to_library(library, popup_state, ui)
//             }
//             Holder::HolderPlaceHolder(place_holder_holder) => {
//                 place_holder_holder.draw_adding_to_library(library, popup_state, ui)
//             }
//         }
//     }
// }
// impl Drawable for ColletHolder {
//     fn draw_display(&mut self, ui: &mut egui::Ui) {
//         ui.horizontal(|ui| {
//             ui.label(RichText::new("Collet holder").underline().strong());
//             ui.horizontal(|ui| {
//                 ui.label("Name: ");
//                 ui.label(&self.name);
//             });
//             ui.horizontal(|ui| {
//                 ui.label("Type: ");
//                 ui.label(&self.collet_type_name);
//             });
//             ui.horizontal(|ui| {
//                 ui.label("Size: ");
//                 ui.label(format!("{:.2}", self.collet_size));
//             });
//         });
//     }

//     fn draw_edit(&mut self, ui: &mut egui::Ui) {
//         ui.add(egui::TextEdit::singleline(&mut self.name).hint_text("Name"));
//         ui.add(egui::TextEdit::singleline(&mut self.collet_type_name).hint_text("Type"));
//         ui.add(egui::Slider::new(&mut self.collet_size, 0.0..=100.0).text("Size"));
//     }

//     fn draw_adding_to_library(
//         &mut self,
//         library: &mut Library,
//         popup_state: &mut PopupState,
//         ui: &mut egui::Ui,
//     ) {
//         self.draw_edit(ui);

//         ui.horizontal(|ui| {
//             if ui.button("Cancel").clicked() {
//                 *popup_state = PopupState::None;
//             }
//             if ui.button("Save").clicked() {
//                 self.location_id = library.id;
//                 library.holders.push(Holder::ColletHolder(self.clone()));
//                 *popup_state = PopupState::None;
//             }
//         });
//     }
// }

// impl Drawable for EndMillHolder {
//     fn draw_display(&mut self, ui: &mut egui::Ui) {
//         ui.horizontal(|ui| {
//             ui.label(RichText::new("End Mill Holder").underline().strong());
//             ui.horizontal(|ui| {
//                 ui.label("Name: ");
//                 ui.label(&self.name);
//             });
//         });
//     }

//     fn draw_edit(&mut self, ui: &mut egui::Ui) {
//         ui.add(egui::TextEdit::singleline(&mut self.name).hint_text("Name"));
//     }

//     fn draw_adding_to_library(
//         &mut self,
//         library: &mut Library,
//         popup_state: &mut PopupState,
//         ui: &mut egui::Ui,
//     ) {
//         self.draw_edit(ui);

//         ui.horizontal(|ui| {
//             if ui.button("Cancel").clicked() {
//                 *popup_state = PopupState::None;
//             }
//             if ui.button("Save").clicked() {
//                 self.location_id = library.id;
//                 library.holders.push(Holder::EndMillHolder(self.clone()));
//                 *popup_state = PopupState::None;
//             }
//         });
//     }
// }

// impl Drawable for DrillChuckHolder {
//     fn draw_display(&mut self, ui: &mut egui::Ui) {
//         ui.horizontal(|ui| {
//             ui.label(RichText::new("Drill Chuck Holder").underline().strong());
//             ui.horizontal(|ui| {
//                 ui.label("Name: ");
//                 ui.label(&self.name);
//             });
//         });
//     }

//     fn draw_edit(&mut self, ui: &mut egui::Ui) {
//         ui.add(egui::TextEdit::singleline(&mut self.name).hint_text("Name"));
//     }

//     fn draw_adding_to_library(
//         &mut self,
//         library: &mut Library,
//         popup_state: &mut PopupState,
//         ui: &mut egui::Ui,
//     ) {
//         self.draw_edit(ui);
//         ui.horizontal(|ui| {
//             if ui.button("Cancel").clicked() {
//                 *popup_state = PopupState::None;
//             }
//             if ui.button("Save").clicked() {
//                 self.location_id = library.id;
//                 library.holders.push(Holder::DrillChuckHolder(self.clone()));
//                 *popup_state = PopupState::None;
//             }
//         });
//     }
// }

// impl Drawable for ExternalTurningHolder {
//     fn draw_display(&mut self, ui: &mut egui::Ui) {
//         ui.horizontal(|ui| {
//             ui.label(
//                 RichText::new("External Turning Holder")
//                     .underline()
//                     .strong(),
//             );
//             ui.horizontal(|ui| {
//                 ui.label("Name: ");
//                 ui.label(&self.name);
//             });
//         });
//     }

//     fn draw_edit(&mut self, ui: &mut egui::Ui) {
//         ui.add(egui::TextEdit::singleline(&mut self.name).hint_text("Name"));
//     }

//     fn draw_adding_to_library(
//         &mut self,
//         library: &mut Library,
//         popup_state: &mut PopupState,
//         ui: &mut egui::Ui,
//     ) {
//         self.draw_edit(ui);
//         ui.horizontal(|ui| {
//             if ui.button("Cancel").clicked() {
//                 *popup_state = PopupState::None;
//             }
//             if ui.button("Save").clicked() {
//                 self.location_id = library.id;
//                 library
//                     .holders
//                     .push(Holder::ExternalTurningHolder(self.clone()));
//                 *popup_state = PopupState::None;
//             }
//         });
//     }
// }

// impl Drawable for InternalTurningHolder {
//     fn draw_display(&mut self, ui: &mut egui::Ui) {
//         ui.horizontal(|ui| {
//             ui.label(
//                 RichText::new("Internal Turning Holder")
//                     .underline()
//                     .strong(),
//             );
//             ui.horizontal(|ui| {
//                 ui.label("Name: ");
//                 ui.label(&self.name);
//             });
//         });
//     }

//     fn draw_edit(&mut self, ui: &mut egui::Ui) {
//         ui.add(egui::TextEdit::singleline(&mut self.name).hint_text("Name"));
//     }

//     fn draw_adding_to_library(
//         &mut self,
//         library: &mut Library,
//         popup_state: &mut PopupState,
//         ui: &mut egui::Ui,
//     ) {
//         self.draw_edit(ui);
//         ui.horizontal(|ui| {
//             if ui.button("Cancel").clicked() {
//                 *popup_state = PopupState::None;
//             }
//             if ui.button("Save").clicked() {
//                 self.location_id = library.id;
//                 library
//                     .holders
//                     .push(Holder::InternalTurningHolder(self.clone()));
//                 *popup_state = PopupState::None;
//             }
//         });
//     }
// }

// impl Drawable for GroovingPartingHolder {
//     fn draw_display(&mut self, ui: &mut egui::Ui) {
//         ui.horizontal(|ui| {
//             ui.label(
//                 RichText::new("Grooving Parting Holder")
//                     .underline()
//                     .strong(),
//             );
//             ui.horizontal(|ui| {
//                 ui.label("Name: ");
//                 ui.label(&self.name);
//             });
//         });
//     }

//     fn draw_edit(&mut self, ui: &mut egui::Ui) {
//         ui.add(egui::TextEdit::singleline(&mut self.name).hint_text("Name"));
//     }

//     fn draw_adding_to_library(
//         &mut self,
//         library: &mut Library,
//         popup_state: &mut PopupState,
//         ui: &mut egui::Ui,
//     ) {
//         self.draw_edit(ui);
//         ui.horizontal(|ui| {
//             if ui.button("Cancel").clicked() {
//                 *popup_state = PopupState::None;
//             }
//             if ui.button("Save").clicked() {
//                 self.location_id = library.id;
//                 library
//                     .holders
//                     .push(Holder::GroovingPartingHolder(self.clone()));
//                 *popup_state = PopupState::None;
//             }
//         });
//     }
// }

// impl Drawable for ThreadingHolder {
//     fn draw_display(&mut self, ui: &mut egui::Ui) {
//         ui.horizontal(|ui| {
//             ui.label(RichText::new("Threading Holder").underline().strong());
//             ui.horizontal(|ui| {
//                 ui.label("Name: ");
//                 ui.label(&self.name);
//             });
//         });
//     }

//     fn draw_edit(&mut self, ui: &mut egui::Ui) {
//         ui.add(egui::TextEdit::singleline(&mut self.name).hint_text("Name"));
//     }

//     fn draw_adding_to_library(
//         &mut self,
//         library: &mut Library,
//         popup_state: &mut PopupState,
//         ui: &mut egui::Ui,
//     ) {
//         self.draw_edit(ui);
//         ui.horizontal(|ui| {
//             if ui.button("Cancel").clicked() {
//                 *popup_state = PopupState::None;
//             }
//             if ui.button("Save").clicked() {
//                 self.location_id = library.id;
//                 library.holders.push(Holder::ThreadingHolder(self.clone()));
//                 *popup_state = PopupState::None;
//             }
//         });
//     }
// }
// impl Drawable for TappingHolder {
//     fn draw_display(&mut self, ui: &mut egui::Ui) {
//         ui.horizontal(|ui| {
//             ui.label(RichText::new("Tapping Holder").underline().strong());
//             ui.horizontal(|ui| {
//                 ui.label("Name: ");
//                 ui.label(&self.name);
//             });
//         });
//     }

//     fn draw_edit(&mut self, ui: &mut egui::Ui) {
//         ui.label("Tapping Holder");
//         ui.add(egui::TextEdit::singleline(&mut self.name).hint_text("Name"));
//     }

//     fn draw_adding_to_library(
//         &mut self,
//         library: &mut Library,
//         popup_state: &mut PopupState,
//         ui: &mut egui::Ui,
//     ) {
//         self.draw_edit(ui);
//         ui.horizontal(|ui| {
//             if ui.button("Cancel").clicked() {
//                 *popup_state = PopupState::None;
//             }
//             if ui.button("Save").clicked() {
//                 self.location_id = library.id;
//                 library.holders.push(Holder::TappingHolder(self.clone()));
//                 *popup_state = PopupState::None;
//             }
//         });
//     }
// }

// impl Drawable for PlaceHolderHolder {
//     fn draw_display(&mut self, _ui: &mut egui::Ui) {}
//     fn draw_edit(&mut self, _ui: &mut egui::Ui) {}
//     fn draw_adding_to_library(
//         &mut self,
//         _library: &mut Library,
//         _popup_state: &mut PopupState,
//         _ui: &mut egui::Ui,
//     ) {
//     }
// }

// impl Drawable for Adapter {
//     fn draw_display(&mut self, ui: &mut egui::Ui) {
//         match self {
//             Adapter::ShrinkFitAdapter(shrink_fit_adapter) => shrink_fit_adapter.draw_display(ui),
//             Adapter::SideLockAdapter(side_lock_adapter) => side_lock_adapter.draw_display(ui),
//             Adapter::HydraulicAdapter(hydraulic_adapter) => hydraulic_adapter.draw_display(ui),
//             Adapter::PlaceHolderAdapter(_) => {}
//         }
//     }
//     fn draw_edit(&mut self, ui: &mut egui::Ui) {
//         match self {
//             Adapter::ShrinkFitAdapter(shrink_fit_adapter) => shrink_fit_adapter.draw_edit(ui),
//             Adapter::SideLockAdapter(side_lock_adapter) => side_lock_adapter.draw_edit(ui),
//             Adapter::HydraulicAdapter(hydraulic_adapter) => hydraulic_adapter.draw_edit(ui),
//             Adapter::PlaceHolderAdapter(_) => {}
//         }
//     }
//     fn draw_adding_to_library(
//         &mut self,
//         library: &mut Library,
//         popup_state: &mut PopupState,
//         ui: &mut Ui,
//     ) {
//         match self {
//             Adapter::ShrinkFitAdapter(shrink_fit_adapter) => {
//                 shrink_fit_adapter.draw_adding_to_library(library, popup_state, ui)
//             }
//             Adapter::SideLockAdapter(side_lock_adapter) => {
//                 side_lock_adapter.draw_adding_to_library(library, popup_state, ui)
//             }
//             Adapter::HydraulicAdapter(hydraulic_adapter) => {
//                 hydraulic_adapter.draw_adding_to_library(library, popup_state, ui)
//             }
//             Adapter::PlaceHolderAdapter(_) => {}
//         }
//     }
// }

// impl Drawable for ShrinkFitAdapter {
//     fn draw_display(&mut self, ui: &mut egui::Ui) {
//         ui.horizontal(|ui| {
//             ui.label(RichText::new("Shrink Fit Adapter").underline().strong());
//             ui.horizontal(|ui| {
//                 ui.label("Name: ");
//                 ui.label(&self.name);
//             });
//         });
//     }
//     fn draw_edit(&mut self, ui: &mut egui::Ui) {
//         ui.heading("Shrink Fit Adapter");
//         ui.add(egui::TextEdit::singleline(&mut self.name).hint_text("Name"));
//     }
//     fn draw_adding_to_library(
//         &mut self,
//         library: &mut Library,
//         popup_state: &mut PopupState,
//         ui: &mut Ui,
//     ) {
//         self.draw_edit(ui);

//         ui.horizontal(|ui| {
//             if ui.button("Cancel").clicked() {
//                 *popup_state = PopupState::None;
//                 return;
//             }
//             if ui.button("Save").clicked() {
//                 self.location_id = library.id;
//                 library
//                     .adapters
//                     .push(Adapter::ShrinkFitAdapter(self.clone()));
//                 *popup_state = PopupState::None;
//             }
//         });
//     }
// }

// impl Drawable for SideLockAdapter {
//     fn draw_display(&mut self, ui: &mut egui::Ui) {
//         ui.horizontal(|ui| {
//             ui.label(RichText::new("Side Lock Adapter").underline().strong());
//             ui.horizontal(|ui| {
//                 ui.label("Name: ");
//                 ui.label(&self.name);
//             });
//         });
//     }
//     fn draw_edit(&mut self, ui: &mut egui::Ui) {
//         ui.heading("Side Lock Adapter");
//         ui.add(egui::TextEdit::singleline(&mut self.name).hint_text("Name"));
//     }
//     fn draw_adding_to_library(
//         &mut self,
//         library: &mut Library,
//         popup_state: &mut PopupState,
//         ui: &mut Ui,
//     ) {
//         self.draw_edit(ui);

//         ui.horizontal(|ui| {
//             if ui.button("Cancel").clicked() {
//                 *popup_state = PopupState::None;
//                 return;
//             }
//             if ui.button("Save").clicked() {
//                 self.location_id = library.id;
//                 library
//                     .adapters
//                     .push(Adapter::SideLockAdapter(self.clone()));
//                 *popup_state = PopupState::None;
//             }
//         });
//     }
// }

// impl Drawable for HydraulicAdapter {
//     fn draw_display(&mut self, ui: &mut egui::Ui) {
//         ui.horizontal(|ui| {
//             ui.label(RichText::new("Hydraulic Adapter").underline().strong());
//             ui.horizontal(|ui| {
//                 ui.label("Name: ");
//                 ui.label(&self.name);
//             });
//         });
//     }
//     fn draw_edit(&mut self, ui: &mut egui::Ui) {
//         ui.heading("Hydraulic Adapter");
//         ui.add(egui::TextEdit::singleline(&mut self.name).hint_text("Name"));
//     }
//     fn draw_adding_to_library(
//         &mut self,
//         library: &mut Library,
//         popup_state: &mut PopupState,
//         ui: &mut Ui,
//     ) {
//         self.draw_edit(ui);

//         ui.horizontal(|ui| {
//             if ui.button("Cancel").clicked() {
//                 *popup_state = PopupState::None;
//                 return;
//             }
//             if ui.button("Save").clicked() {
//                 self.location_id = library.id;
//                 library
//                     .adapters
//                     .push(Adapter::HydraulicAdapter(self.clone()));
//                 *popup_state = PopupState::None;
//             }
//         });
//     }
// }
