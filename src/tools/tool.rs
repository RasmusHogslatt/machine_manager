use uuid::Uuid;

use crate::{
    circular_insert::*, diamond_insert::*, drill::*, mill::*, square_insert::*,
    tool_placeholder::*, triangle_insert::*, trigon_insert::*, Drawable, Identifiable, Library,
    Locatable, PopupState,
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub enum ToolCategory {
    #[default]
    Rotating,
    LatheInsert,
    Empty,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum Tool {
    ToolPlaceHolder(ToolPlaceHolder),
    Drill(Drill),
    Mill(Mill),
    TriangleInsert(TriangleInsert),
    CircularInsert(CircularInsert),
    DiamondInsert(DiamondInsert),
    TrigonInsert(TrigonInsert),
    SquareInsert(SquareInsert),
}

impl Identifiable for Tool {
    fn get_id(&self) -> Uuid {
        match self {
            Tool::ToolPlaceHolder(tool) => tool.get_id(),
            Tool::Drill(tool) => tool.get_id(),
            Tool::Mill(tool) => tool.get_id(),
            Tool::TriangleInsert(tool) => tool.get_id(),
            Tool::CircularInsert(tool) => tool.get_id(),
            Tool::DiamondInsert(tool) => tool.get_id(),
            Tool::TrigonInsert(tool) => tool.get_id(),
            Tool::SquareInsert(tool) => tool.get_id(),
        }
    }
}
impl Locatable for Tool {
    fn set_location_id(&mut self, location_id: Uuid) {
        match self {
            Tool::ToolPlaceHolder(tool) => tool.set_location_id(location_id),
            Tool::Drill(tool) => tool.set_location_id(location_id),
            Tool::Mill(tool) => tool.set_location_id(location_id),
            Tool::TriangleInsert(tool) => tool.set_location_id(location_id),
            Tool::CircularInsert(tool) => tool.set_location_id(location_id),
            Tool::DiamondInsert(tool) => tool.set_location_id(location_id),
            Tool::TrigonInsert(tool) => tool.set_location_id(location_id),
            Tool::SquareInsert(tool) => tool.set_location_id(location_id),
        }
    }

    fn set_location_slot(&mut self, location_slot: usize) {
        match self {
            Tool::ToolPlaceHolder(place_holder_tool) => {
                place_holder_tool.location_slot = location_slot
            }
            Tool::Drill(tool) => tool.set_location_slot(location_slot),
            Tool::Mill(tool) => tool.set_location_slot(location_slot),
            Tool::TriangleInsert(tool) => tool.set_location_slot(location_slot),
            Tool::CircularInsert(tool) => tool.set_location_slot(location_slot),
            Tool::DiamondInsert(tool) => tool.set_location_slot(location_slot),
            Tool::TrigonInsert(tool) => tool.set_location_slot(location_slot),
            Tool::SquareInsert(tool) => tool.set_location_slot(location_slot),
        }
    }
}

impl Drawable for Tool {
    fn draw_display(&mut self, ui: &mut egui::Ui) {
        match self {
            Tool::Drill(drill) => drill.draw_display(ui),
            Tool::Mill(mill) => mill.draw_display(ui),
            Tool::ToolPlaceHolder(place_holder_tool) => place_holder_tool.draw_display(ui),
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
            Tool::ToolPlaceHolder(place_holder_tool) => place_holder_tool.draw_edit(ui),
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
            Tool::ToolPlaceHolder(place_holder_tool) => {
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
