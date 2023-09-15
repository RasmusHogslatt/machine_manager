use crate::{drawable::*, library::*, resources::*, states::*};

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
    TriangleInsert(TriangleInsert),
    CircularInsert(CircularInsert),
    DiamondInsert(DiamondInsert),
    TrigonInsert(TrigonInsert),
    SquareInsert(SquareInsert),
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

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq)]
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

// Add tool to library's 'tools' vector
pub fn add_tool(
    gui_resource: &mut GuiResource,
    library: &mut Library,
    popup_state: &mut PopupState,
    ctx: &egui::Context,
) {
    if popup_state != &PopupState::AddTool {
        return;
    }
    egui::Window::new("Add Tool").show(ctx, |ui| {
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
                        Tool::TriangleInsert(TriangleInsert::default()),
                        "Triangle",
                    );
                    ui.radio_value(
                        &mut gui_resource.tool_selected,
                        Tool::CircularInsert(CircularInsert::default()),
                        "Circular",
                    );
                    ui.radio_value(
                        &mut gui_resource.tool_selected,
                        Tool::DiamondInsert(DiamondInsert::default()),
                        "Diamond",
                    );
                    ui.radio_value(
                        &mut gui_resource.tool_selected,
                        Tool::TrigonInsert(TrigonInsert::default()),
                        "Trigon",
                    );
                    ui.radio_value(
                        &mut gui_resource.tool_selected,
                        Tool::SquareInsert(SquareInsert::default()),
                        "Square",
                    );
                });
            }
            ToolCategory::Empty => {}
        }
        match (&gui_resource.tool_selected, &gui_resource.tool_category) {
            (Tool::Drill(_), ToolCategory::Rotating) => {
                gui_resource
                    .drill
                    .draw_adding_to_library(library, popup_state, ui);
            }
            (Tool::Mill(_), ToolCategory::Rotating) => {
                gui_resource
                    .mill
                    .draw_adding_to_library(library, popup_state, ui);
            }
            (Tool::TriangleInsert(_), ToolCategory::LatheInsert) => {
                gui_resource
                    .triangle_insert
                    .draw_adding_to_library(library, popup_state, ui);
            }
            (Tool::CircularInsert(_), ToolCategory::LatheInsert) => {
                gui_resource
                    .circular_insert
                    .draw_adding_to_library(library, popup_state, ui);
            }
            (Tool::DiamondInsert(_), ToolCategory::LatheInsert) => {
                gui_resource
                    .diamond_insert
                    .draw_adding_to_library(library, popup_state, ui);
            }
            (Tool::TrigonInsert(_), ToolCategory::LatheInsert) => {
                gui_resource
                    .trigon_insert
                    .draw_adding_to_library(library, popup_state, ui);
            }
            (Tool::SquareInsert(_), ToolCategory::LatheInsert) => {
                gui_resource
                    .square_insert
                    .draw_adding_to_library(library, popup_state, ui);
            }
            (_, _) => {}
        }
    });
}
