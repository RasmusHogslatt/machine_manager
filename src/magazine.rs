use std::cmp::Ordering;

use crate::{adapter::*, holder::*, tool::*};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Magazine {
    pub name: String,
    pub id: uuid::Uuid,
    pub location_id: uuid::Uuid, // which machine?
    pub location_slot: usize,    // which magazine number?
    pub contents: Vec<(Option<Tool>, Option<Holder>, Option<Adapter>)>,
    pub capacity: usize,
}

fn get_sorted_by_tool_diameter(
    contents: &mut Vec<(Option<Tool>, Option<Holder>, Option<Adapter>)>,
) -> Vec<&mut (Option<Tool>, Option<Holder>, Option<Adapter>)> {
    let mut filtered = contents
        .iter_mut()
        .filter(|(tool, holder, adapter)| {
            tool.map(|t| t.get_category() == ToolCategory::Rotating)
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();

    filtered.sort_by(|(tool_a, ..), (tool_b, ..)| {
        tool_a
            .unwrap()
            .get_diameter()
            .unwrap()
            .partial_cmp(&tool_b.unwrap().get_diameter().unwrap())
            .unwrap_or(Ordering::Equal)
    });

    filtered
}
