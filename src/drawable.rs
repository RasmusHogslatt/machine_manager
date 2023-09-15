use crate::{library::*, states::*, tool::*};

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

impl Drawable for Tool {
    fn draw_display(&mut self, ui: &mut egui::Ui) {
        match self {
            Tool::Drill(drill) => drill.draw_display(ui),
            Tool::Mill(mill) => mill.draw_display(ui),
            Tool::PlaceHolderTool(place_holder_tool) => place_holder_tool.draw_display(ui),
            Tool::TriangleInsert(triangle_insert) => triangle_insert.draw_display(ui),
            Tool::CircularInsert(circular_insert) => circular_insert.draw_display(ui),
            Tool::SquareInsert(square_insert) => square_insert.draw_display(ui),
            Tool::TrigonInsert(trigon_insert) => trigon_insert.draw_display(ui),
            Tool::DiamondInsert(diamond_insert) => diamond_insert.draw_display(ui),
        }
    }
    fn draw_edit(&mut self, ui: &mut egui::Ui) {
        match self {
            Tool::Drill(drill) => drill.draw_edit(ui),
            Tool::Mill(mill) => mill.draw_edit(ui),
            Tool::PlaceHolderTool(place_holder_tool) => place_holder_tool.draw_edit(ui),
            Tool::TriangleInsert(triangle_insert) => triangle_insert.draw_edit(ui),
            Tool::CircularInsert(circular_insert) => circular_insert.draw_edit(ui),
            Tool::SquareInsert(square_insert) => square_insert.draw_edit(ui),
            Tool::TrigonInsert(trigon_insert) => trigon_insert.draw_edit(ui),
            Tool::DiamondInsert(diamond_insert) => diamond_insert.draw_edit(ui),
        }
    }

    fn draw_adding_to_library(
        &mut self,
        library: &mut Library,
        popup_state: &mut PopupState,
        ui: &mut egui::Ui,
    ) {
        match self {
            Tool::Drill(drill) => drill.draw_adding_to_library(library, popup_state, ui),
            Tool::Mill(mill) => mill.draw_adding_to_library(library, popup_state, ui),
            Tool::PlaceHolderTool(place_holder_tool) => {
                place_holder_tool.draw_adding_to_library(library, popup_state, ui)
            }
            Tool::TriangleInsert(triangle_insert) => {
                triangle_insert.draw_adding_to_library(library, popup_state, ui)
            }
            Tool::CircularInsert(circular_insert) => {
                circular_insert.draw_adding_to_library(library, popup_state, ui)
            }
            Tool::SquareInsert(square_insert) => {
                square_insert.draw_adding_to_library(library, popup_state, ui)
            }
            Tool::TrigonInsert(trigon_insert) => {
                trigon_insert.draw_adding_to_library(library, popup_state, ui)
            }
            Tool::DiamondInsert(diamond_insert) => {
                diamond_insert.draw_adding_to_library(library, popup_state, ui)
            }
        }
    }
}

impl Drawable for Drill {
    fn draw_display(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Drill");
            ui.label(&self.name);
            ui.label(&self.diameter.to_string());
        });
    }
    fn draw_edit(&mut self, ui: &mut egui::Ui) {
        ui.heading("Drill");
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
                library.tools.push(Tool::Drill(self.clone()));
                *popup_state = PopupState::None;
            }
        });
    }
}

impl Drawable for Mill {
    fn draw_display(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Mill");
            ui.label(&self.name);
            ui.label(&self.diameter.to_string());
        });
    }
    fn draw_edit(&mut self, ui: &mut egui::Ui) {
        ui.heading("Mill");
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
                library.tools.push(Tool::Mill(self.clone()));
                *popup_state = PopupState::None;
            }
        });
    }
}

impl Drawable for PlaceHolderTool {
    fn draw_display(&mut self, _ui: &mut egui::Ui) {}
    fn draw_edit(&mut self, _ui: &mut egui::Ui) {}
    fn draw_adding_to_library(
        &mut self,
        _library: &mut Library,
        _popup_state: &mut PopupState,
        _ui: &mut egui::Ui,
    ) {
    }
}

impl Drawable for TriangleInsert {
    fn draw_display(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Triangle Insert");
            ui.label(&self.name);
            ui.label(&self.degree.to_string());
        });
    }
    fn draw_edit(&mut self, ui: &mut egui::Ui) {
        ui.heading("Triangle Insert");
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
                library.tools.push(Tool::TriangleInsert(self.clone()));
                *popup_state = PopupState::None;
            }
        });
    }
}

impl Drawable for CircularInsert {
    fn draw_display(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Circular Insert");
            ui.label(&self.name);
            ui.label(&self.degree.to_string());
        });
    }
    fn draw_edit(&mut self, ui: &mut egui::Ui) {
        ui.heading("Circular Insert");
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
                library.tools.push(Tool::CircularInsert(self.clone()));
                *popup_state = PopupState::None;
            }
        });
    }
}

impl Drawable for SquareInsert {
    fn draw_display(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Square Insert");
            ui.label(&self.name);
            ui.label(&self.degree.to_string());
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
                library.tools.push(Tool::SquareInsert(self.clone()));
                *popup_state = PopupState::None;
            }
        });
    }
}

impl Drawable for TrigonInsert {
    fn draw_display(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Trigon Insert");
            ui.label(&self.name);
            ui.label(&self.degree.to_string());
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
                library.tools.push(Tool::TrigonInsert(self.clone()));
                *popup_state = PopupState::None;
            }
        });
    }
}

impl Drawable for DiamondInsert {
    fn draw_display(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Diamond Insert");
            ui.label(&self.name);
            ui.label(&self.degree.to_string());
        });
    }
    fn draw_edit(&mut self, ui: &mut egui::Ui) {
        ui.heading("Diamond Insert");
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
                library.tools.push(Tool::DiamondInsert(self.clone()));
                *popup_state = PopupState::None;
            }
        });
    }
}
