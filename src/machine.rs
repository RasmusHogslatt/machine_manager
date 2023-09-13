use crate::{adapter::*, holder::*, magazine::*, resources::*, states::*, tool::*};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Machine {
    pub name: String,
    pub id: uuid::Uuid,
    pub magazine_count: usize,
    pub magazines: Vec<Magazine>,
}

impl PartialEq for Machine {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub fn add_machine(
    gui_resource: &mut GuiResource,
    machines: &mut Vec<Machine>,
    popup_state: &mut PopupState,
    ctx: &egui::Context,
) {
    egui::Window::new("Add Machine").show(ctx, |ui| {
        // UI elements to collect machine properties
        ui.label("Machine name:");
        ui.text_edit_singleline(&mut gui_resource.machine.name);

        ui.label("Magazine count:");
        ui.add(egui::DragValue::new(&mut gui_resource.machine.magazine_count).speed(1.0));

        // Generate machine ID. This is added to items in magazine
        gui_resource.machine.id = uuid::Uuid::new_v4();

        // Check if there should be a magazine
        if gui_resource.machine.magazine_count > 0 {
            // Choose magazine capacity
            ui.horizontal(|ui| {
                ui.label("Magazine capacity:");
                ui.add(egui::DragValue::new(&mut gui_resource.magazine.capacity).speed(1.0));
            });
            let mut placeholder_tools: Vec<Tool> = Vec::new();
            let mut placeholder_holders: Vec<Holder> = Vec::new();
            let mut placeholder_adapters: Vec<Adapter> = Vec::new();
            for i in 0..gui_resource.magazine.capacity {
                placeholder_tools.push(Tool::PlaceHolderTool(PlaceHolderTool {
                    name: format!("Empty tool {}", i).to_string(),
                    id: uuid::Uuid::new_v4(),
                    location_id: gui_resource.machine.id,
                    location_slot: i,
                    category: ToolCategory::Empty,
                }));
                placeholder_holders.push(Holder::PlaceHolderHolder(PlaceHolderHolder {
                    name: format!("Empty holder {}", i).to_string(),
                    id: uuid::Uuid::new_v4(),
                    location_id: gui_resource.machine.id,
                    location_slot: i,
                    category: HolderCategory::Empty,
                }));
                placeholder_adapters.push(Adapter::PlaceHolderAdapter(PlaceHolderAdapter {
                    name: format!("Empty adapter {}", i).to_string(),
                    id: uuid::Uuid::new_v4(),
                    location_id: gui_resource.machine.id,
                    location_slot: i,
                    category: AdapterCategory::Empty,
                }));
            }
            gui_resource.magazine.tools = placeholder_tools;
            gui_resource.magazine.holders = placeholder_holders;
            gui_resource.magazine.adapters = placeholder_adapters;
        }
        ui.horizontal(|ui| {
            if ui.button("Cancel").clicked() {
                *popup_state = PopupState::None;
            }
            if ui.button("Save").clicked() {
                for i in 0..gui_resource.machine.magazine_count {
                    gui_resource.magazine.location_id = gui_resource.machine.id;
                    gui_resource.magazine.location_slot = i;
                    gui_resource.magazine.id = uuid::Uuid::new_v4();
                    gui_resource.magazine.name = format!("Magazine {}", i).to_string();
                    gui_resource
                        .machine
                        .magazines
                        .push(gui_resource.magazine.clone());
                }
                machines.push(gui_resource.machine.clone());
                gui_resource.machine = Machine {
                    name: "Machine".to_string(),
                    id: uuid::Uuid::new_v4(),
                    magazine_count: 1,
                    magazines: Vec::new(),
                };
                *popup_state = PopupState::None;
            }
        });
    });
}
