use uuid::Uuid;

use crate::adapter::*;
use crate::holder::*;
use crate::tool::*;

pub fn remove_tool_from_vec(tools: &mut Vec<Tool>, tool_id: Uuid) -> Option<Tool> {
    let index = tools.iter().position(|tool| tool.get_id() == tool_id);
    index.map(|index| tools.remove(index))
}

pub fn add_tool_to_vec(tools: &mut Vec<Tool>, mut tool: Tool, slot: usize, location_id: Uuid) {
    tool.set_location_id(location_id);
    tool.set_location_slot(slot);
    tools[slot] = tool;
}

// pub fn remove_holder_from_vec(holders: &mut Vec<Holder>, holder_id: u32) -> Option<Holder> {
//     let index = holders.iter().position(|holder| holder.id == holder_id)?;
//     Some(holders.remove(index))
// }

// pub fn remove_adapter_from_vec(adapters: &mut Vec<Adapter>, adapter_id: u32) -> Option<Adapter> {
//     let index = adapters
//         .iter()
//         .position(|adapter| adapter.id == adapter_id)?;
//     Some(adapters.remove(index))
// }

pub trait Identifiable {
    fn get_id(&self) -> Uuid;
}

pub trait Locatable {
    fn set_location_id(&mut self, location_id: Uuid);
    fn set_location_slot(&mut self, location_slot: usize);
}
