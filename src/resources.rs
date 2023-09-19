use crate::{adapter::*, holder::*, machine::*, magazine::*, tool::*};

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
    pub placeholder_tool: PlaceHolderTool,
    // holders
    pub collet_holder: ColletHolder,
    pub end_mill_holder: EndMillHolder,
    pub drill_chuck_holder: DrillChuckHolder,
    pub external_turning_holder: ExternalTurningHolder,
    pub internal_turning_holder: InternalTurningHolder,
    pub grooving_parting_holder: GroovingPartingHolder,
    pub threading_holder: ThreadingHolder,
    pub tapping_holder: TappingHolder,
    pub placeholder_holder: PlaceHolderHolder,
    // adapters
    pub shrink_fit_adapter: ShrinkFitAdapter,
    pub side_lock_adapter: SideLockAdapter,
    pub hydraulic_adapter: HydraulicAdapter,
    pub placeholder_adapter: PlaceHolderAdapter,
    // machine
    pub machine: Machine,
    pub magazine: Magazine,
    pub chosen_magazine_content: MagazineContent,
    pub chosen_library_content: LibraryContent,
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
