use crate::{library::*, resources::*, states::*};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub enum ToolCategory {
    #[default]
    Rotating,
    LatheInsert,
    Empty,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum Tool {
    PlaceHolderTool(PlaceHolderTool),
    Drill(Drill),
    Mill(Mill),
    TriangleInsert,
    CircularInsert,
    DiamondInsert,
    TrigonInsert,
    SquareInsert,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Drill {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: ToolCategory,
    pub diameter: f32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Mill {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: ToolCategory,
    pub diameter: f32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq)]
pub struct PlaceHolderTool {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: ToolCategory,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq)]
pub struct TriangleInsert {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: ToolCategory,
    pub degree: f32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq)]
pub struct CircularInsert {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: ToolCategory,
    pub degree: f32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq)]
pub struct DiamondInsert {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: ToolCategory,
    pub degree: f32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq)]
pub struct TrigonInsert {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: ToolCategory,
    pub degree: f32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq)]
pub struct SquareInsert {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid,
    pub location_slot: usize,
    pub category: ToolCategory,
    pub degree: f32,
}

// Add tool to library
pub fn add_tool(
    gui_resource: &mut GuiResource,
    library: &mut Library,
    popup_state: &mut PopupState,
    ctx: &egui::Context,
) {
    egui::Window::new("Add Tool").show(ctx, |ui| {
        // Choose tool category with radio buttons (rotating, latheinsert, empty)
        ui.horizontal(|ui| {
            ui.radio_value(
                &mut gui_resource.tool_category,
                ToolCategory::LatheInsert,
                "Lathe Insert",
            );
            ui.radio_value(
                &mut gui_resource.tool_category,
                ToolCategory::Rotating,
                "Rotating",
            );
            ui.radio_value(
                &mut gui_resource.tool_category,
                ToolCategory::Empty,
                "Empty",
            );
        });
        match gui_resource.tool_category {
            ToolCategory::Rotating => {
                // if rotating, choose drill or mill as tool_selected
                ui.horizontal(|ui| {
                    ui.radio_value(
                        &mut gui_resource.tool_selected,
                        Tool::Drill(Drill::default()),
                        "Drill",
                    );
                    ui.radio_value(
                        &mut gui_resource.tool_selected,
                        Tool::Mill(Mill::default()),
                        "Mill",
                    );
                });
            }
            ToolCategory::LatheInsert => {
                // if latheinsert, choose insert as tool_selected
                ui.horizontal(|ui| {
                    ui.radio_value(
                        &mut gui_resource.tool_selected,
                        Tool::TriangleInsert,
                        "Triangle",
                    );
                    ui.radio_value(
                        &mut gui_resource.tool_selected,
                        Tool::CircularInsert,
                        "Circular",
                    );
                    ui.radio_value(
                        &mut gui_resource.tool_selected,
                        Tool::DiamondInsert,
                        "Diamond",
                    );
                    ui.radio_value(
                        &mut gui_resource.tool_selected,
                        Tool::TrigonInsert,
                        "Trigon",
                    );
                    ui.radio_value(
                        &mut gui_resource.tool_selected,
                        Tool::SquareInsert,
                        "Square",
                    );
                });
            }
            ToolCategory::Empty => {}
        }
        match (&gui_resource.tool_selected, &gui_resource.tool_category) {
            (Tool::Drill(_), ToolCategory::Rotating) => {
                ui.add(egui::TextEdit::singleline(&mut gui_resource.drill.name).hint_text("Name"));
                ui.add(
                    egui::Slider::new(&mut gui_resource.drill.diameter, 0.0..=100.0)
                        .text("Diameter"),
                );
            }
            (Tool::Mill(_), ToolCategory::Rotating) => {
                ui.add(egui::TextEdit::singleline(&mut gui_resource.mill.name).hint_text("Name"));
                ui.add(
                    egui::Slider::new(&mut gui_resource.mill.diameter, 0.0..=100.0)
                        .text("Diameter"),
                );
            }
            (Tool::TriangleInsert, ToolCategory::LatheInsert) => {
                ui.add(
                    egui::TextEdit::singleline(&mut gui_resource.triangle_insert.name)
                        .hint_text("Name"),
                );
                ui.add(
                    egui::Slider::new(&mut gui_resource.triangle_insert.degree, 0.0..=100.0)
                        .text("Degree"),
                );
            }
            (Tool::CircularInsert, ToolCategory::LatheInsert) => {
                ui.add(
                    egui::TextEdit::singleline(&mut gui_resource.circular_insert.name)
                        .hint_text("Name"),
                );
                ui.add(
                    egui::Slider::new(&mut gui_resource.circular_insert.degree, 0.0..=100.0)
                        .text("Degree"),
                );
            }
            (Tool::DiamondInsert, ToolCategory::LatheInsert) => {
                ui.add(
                    egui::TextEdit::singleline(&mut gui_resource.diamond_insert.name)
                        .hint_text("Name"),
                );
                ui.add(
                    egui::Slider::new(&mut gui_resource.diamond_insert.degree, 0.0..=100.0)
                        .text("Degree"),
                );
            }
            (Tool::TrigonInsert, ToolCategory::LatheInsert) => {
                ui.add(
                    egui::TextEdit::singleline(&mut gui_resource.trigon_insert.name)
                        .hint_text("Name"),
                );
                ui.add(
                    egui::Slider::new(&mut gui_resource.trigon_insert.degree, 0.0..=100.0)
                        .text("Degree"),
                );
            }
            (Tool::SquareInsert, ToolCategory::LatheInsert) => {
                ui.add(
                    egui::TextEdit::singleline(&mut gui_resource.square_insert.name)
                        .hint_text("Name"),
                );
                ui.add(
                    egui::Slider::new(&mut gui_resource.square_insert.degree, 0.0..=100.0)
                        .text("Degree"),
                );
            }
            (_, _) => {}
        }

        // Generate machine ID. This is added to items in magazine
        gui_resource.machine.id = uuid::Uuid::new_v4();

        ui.horizontal(|ui| {
            if ui.button("Cancel").clicked() {
                *popup_state = PopupState::None;
            }
            if ui.button("Save").clicked() {
                //push cloned tool to library
                *popup_state = PopupState::None;
            }
        });
    });
}
