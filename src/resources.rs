use crate::{adapter::*, holder::*, machine::*, magazine::*, tool::*};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct GuiResource {
    pub tool_category: ToolCategory,
    pub tool_selected: Tool,
    pub holder_category: HolderCategory,
    pub adapter_category: AdapterCategory,
    pub drill: Drill,
    pub mill: Mill,
    pub triangle_insert: TriangleInsert,
    pub circular_insert: CircularInsert,
    pub square_insert: SquareInsert,
    pub trigon_insert: TrigonInsert,
    pub diamond_insert: DiamondInsert,
    pub placeholder_tool: PlaceHolderTool,
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
