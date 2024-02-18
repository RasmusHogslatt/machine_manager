use std::cmp::Ordering;
use uuid::Uuid;

use crate::{
    adapter::*, circular_insert::*, diamond_insert::*, drill::*, holder::*, mill::*,
    square_insert::*, tool_placeholder::*, triangle_insert::*, trigon_insert::*, Drawable,
    Identifiable, IsPlaceholder, Library, Locatable, PopupState,
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub enum ToolCategory {
    #[default]
    All,
    Rotating,
    LatheInsert,
    Empty,
}

impl std::fmt::Display for ToolCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolCategory::All => write!(f, "All"),
            ToolCategory::Rotating => write!(f, "Rotating"),
            ToolCategory::LatheInsert => write!(f, "Lathe Insert"),
            ToolCategory::Empty => write!(f, "Empty"),
        }
    }
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

pub trait HasDiameter {
    fn get_diameter(&self) -> Option<f32>;
}

pub trait HasDegree {
    fn get_degree(&self) -> Option<f32>;
}

impl Tool {
    pub fn get_category(&self) -> ToolCategory {
        match self {
            Tool::ToolPlaceHolder(tool) => tool.get_category(),
            Tool::Drill(tool) => tool.get_category(),
            Tool::Mill(tool) => tool.get_category(),
            Tool::TriangleInsert(tool) => tool.get_category(),
            Tool::CircularInsert(tool) => tool.get_category(),
            Tool::DiamondInsert(tool) => tool.get_category(),
            Tool::TrigonInsert(tool) => tool.get_category(),
            Tool::SquareInsert(tool) => tool.get_category(),
        }
    }

    pub fn get_location_slot(&self) -> usize {
        match self {
            Tool::ToolPlaceHolder(tool) => tool.get_location_slot(),
            Tool::Drill(tool) => tool.get_location_slot(),
            Tool::Mill(tool) => tool.get_location_slot(),
            Tool::TriangleInsert(tool) => tool.get_location_slot(),
            Tool::CircularInsert(tool) => tool.get_location_slot(),
            Tool::DiamondInsert(tool) => tool.get_location_slot(),
            Tool::TrigonInsert(tool) => tool.get_location_slot(),
            Tool::SquareInsert(tool) => tool.get_location_slot(),
        }
    }
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

    fn get_pdf_string(&self) -> Vec<(String, String)> {
        match self {
            Tool::Drill(drill) => drill.get_pdf_string(),
            Tool::Mill(mill) => mill.get_pdf_string(),
            Tool::ToolPlaceHolder(place_holder_tool) => place_holder_tool.get_pdf_string(),
            Tool::TriangleInsert(triangle_insert) => triangle_insert.get_pdf_string(),
            Tool::CircularInsert(circular_insert) => circular_insert.get_pdf_string(),
            Tool::SquareInsert(square_insert) => square_insert.get_pdf_string(),
            Tool::TrigonInsert(trigon_insert) => trigon_insert.get_pdf_string(),
            Tool::DiamondInsert(diamond_insert) => diamond_insert.get_pdf_string(),
        }
    }
}

impl IsPlaceholder for Tool {
    fn is_placeholder(&self) -> bool {
        match self {
            Tool::ToolPlaceHolder(place_holder_tool) => place_holder_tool.is_placeholder(),
            _ => false,
        }
    }
}

impl HasDiameter for Tool {
    fn get_diameter(&self) -> Option<f32> {
        match self {
            Tool::Drill(drill) => drill.get_diameter(),
            Tool::Mill(mill) => mill.get_diameter(),
            Tool::ToolPlaceHolder(place_holder_tool) => place_holder_tool.get_diameter(),
            Tool::TriangleInsert(triangle_insert) => triangle_insert.get_diameter(),
            Tool::CircularInsert(circular_insert) => circular_insert.get_diameter(),
            Tool::SquareInsert(square_insert) => square_insert.get_diameter(),
            Tool::TrigonInsert(trigon_insert) => trigon_insert.get_diameter(),
            Tool::DiamondInsert(diamond_insert) => diamond_insert.get_diameter(),
        }
    }
}

impl HasDegree for Tool {
    fn get_degree(&self) -> Option<f32> {
        match self {
            Tool::Drill(drill) => drill.get_degree(),
            Tool::Mill(mill) => mill.get_degree(),
            Tool::ToolPlaceHolder(place_holder_tool) => place_holder_tool.get_degree(),
            Tool::TriangleInsert(triangle_insert) => triangle_insert.get_degree(),
            Tool::CircularInsert(circular_insert) => circular_insert.get_degree(),
            Tool::SquareInsert(square_insert) => square_insert.get_degree(),
            Tool::TrigonInsert(trigon_insert) => trigon_insert.get_degree(),
            Tool::DiamondInsert(diamond_insert) => diamond_insert.get_degree(),
        }
    }
}

pub fn sort_tools_by_diameter(tools: &mut Vec<(Tool, Holder, Adapter)>) {
    tools.sort_by(|(a, _, _), (b, _, _)| {
        a.get_diameter()
            .unwrap_or(std::f32::NAN)
            .partial_cmp(&b.get_diameter().unwrap_or(std::f32::NAN))
            .unwrap_or(Ordering::Equal)
    });
}

pub fn sort_tools_by_degree(tools: &mut Vec<(Tool, Holder, Adapter)>) {
    tools.sort_by(|(a, _, _), (b, _, _)| {
        a.get_degree()
            .unwrap_or(std::f32::NAN)
            .partial_cmp(&b.get_degree().unwrap_or(std::f32::NAN))
            .unwrap_or(Ordering::Equal)
    });
}
