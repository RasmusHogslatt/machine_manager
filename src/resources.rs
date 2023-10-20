use crate::{
    adapter::*, adapter_placeholder::AdapterPlaceHolder, circular_insert::CircularInsert,
    collet::Collet, diamond_insert::DiamondInsert, drill::Drill, drill_chuck::DrillChuck,
    end_mill::EndMill, external_turning::ExternalTurning, grooving_parting::GroovingParting,
    holder::*, holder_placeholder::HolderPlaceHolder, hydraulic::Hydraulic,
    internal_turning::InternalTurning, machine::*, magazine::*, mill::Mill, shrink_fit::ShrinkFit,
    side_lock::SideLock, square_insert::SquareInsert, tapping::Tapping, threading::Threading,
    tool::*, tool_placeholder::ToolPlaceHolder, triangle_insert::TriangleInsert,
    trigon_insert::TrigonInsert,
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Selections {
    pub selected_machine: Option<usize>, // Index to what machine is selected
    pub selected_magazine_slot: Option<usize>, // What slot is selected in magazine
    pub selected_library_slot: Option<usize>, // What slot is selected in library
    pub selected_magazine_content: MagazineContent, // Category being accessed
    pub selected_library_content: LibraryContent, // Category being accessed
}

// Single instances of all items possible
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct GuiResource {
    pub tool_category: ToolCategory,
    pub tool_selected: Tool,
    pub holder_selected: Holder,
    pub adapter_selected: Adapter,
    pub holder_category: HolderCategory,
    pub adapter_category: AdapterCategory,
    // tools
    pub drill: Drill,
    pub mill: Mill,
    pub triangle_insert: TriangleInsert,
    pub circular_insert: CircularInsert,
    pub square_insert: SquareInsert,
    pub trigon_insert: TrigonInsert,
    pub diamond_insert: DiamondInsert,
    pub placeholder_tool: ToolPlaceHolder,
    // holders
    pub collet_holder: Collet,
    pub end_mill_holder: EndMill,
    pub drill_chuck_holder: DrillChuck,
    pub external_turning_holder: ExternalTurning,
    pub internal_turning_holder: InternalTurning,
    pub grooving_parting_holder: GroovingParting,
    pub threading_holder: Threading,
    pub tapping_holder: Tapping,
    pub placeholder_holder: HolderPlaceHolder,
    // adapters
    pub shrink_fit_adapter: ShrinkFit,
    pub side_lock_adapter: SideLock,
    pub hydraulic_adapter: Hydraulic,
    pub placeholder_adapter: AdapterPlaceHolder,
    // machine
    pub machine: Machine,
    pub magazine: Magazine,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub enum MagazineContent {
    #[default]
    ToolContent,
    HolderContent,
    AdapterContent,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub enum LibraryContent {
    #[default]
    ToolContent,
    HolderContent,
    AdapterContent,
}
