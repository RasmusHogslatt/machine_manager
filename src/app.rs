use crate::{
    adapter::*, holder::*, library::*, machine::*, magazine::*, resources::*, states::*, tool::*,
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ManagingApp {
    // Example stuff:
    library: Library,
    gui_resource: GuiResource,
    machines: Vec<Machine>,
    app_state: ApplicationState,
    popup_state: PopupState,
    selected_machine: (uuid::Uuid, usize),
    selected_magazine: (uuid::Uuid, usize),
    // this how you opt-out of serialization of a member
    //#[serde(skip)]
}

impl Default for ManagingApp {
    fn default() -> Self {
        Self {
            gui_resource: GuiResource {
                tool_category: ToolCategory::Empty,
                tool_selected: Tool::PlaceHolderTool(PlaceHolderTool {
                    name: "Empty tool".to_string(),
                    id: uuid::Uuid::new_v4(),
                    location_id: uuid::Uuid::new_v4(),
                    location_slot: 0,
                    category: ToolCategory::Empty,
                }),
                holder_category: HolderCategory::Empty,
                adapter_category: AdapterCategory::Empty,
                drill: Drill {
                    name: "drill".to_string(),
                    id: uuid::Uuid::new_v4(),
                    location_id: uuid::Uuid::new_v4(),
                    location_slot: 0,
                    category: ToolCategory::Rotating,
                    diameter: 0.0,
                },
                mill: Mill {
                    name: "mill".to_string(),
                    id: uuid::Uuid::new_v4(),
                    location_id: uuid::Uuid::new_v4(),
                    location_slot: 0,
                    category: ToolCategory::Rotating,
                    diameter: 0.0,
                },
                triangle_insert: TriangleInsert {
                    name: "triangle insert".to_string(),
                    id: uuid::Uuid::new_v4(),
                    location_id: uuid::Uuid::new_v4(),
                    location_slot: 0,
                    category: ToolCategory::LatheInsert,
                    degree: 35.0,
                },
                circular_insert: CircularInsert {
                    name: "circular insert".to_string(),
                    id: uuid::Uuid::new_v4(),
                    location_id: uuid::Uuid::new_v4(),
                    location_slot: 0,
                    category: ToolCategory::LatheInsert,
                    degree: 35.0,
                },
                square_insert: SquareInsert {
                    name: "square insert".to_string(),
                    id: uuid::Uuid::new_v4(),
                    location_id: uuid::Uuid::new_v4(),
                    location_slot: 0,
                    category: ToolCategory::LatheInsert,
                    degree: 35.0,
                },
                trigon_insert: TrigonInsert {
                    name: "trigon insert".to_string(),
                    id: uuid::Uuid::new_v4(),
                    location_id: uuid::Uuid::new_v4(),
                    location_slot: 0,
                    category: ToolCategory::LatheInsert,
                    degree: 35.0,
                },
                diamond_insert: DiamondInsert {
                    name: "diamond insert".to_string(),
                    id: uuid::Uuid::new_v4(),
                    location_id: uuid::Uuid::new_v4(),
                    location_slot: 0,
                    category: ToolCategory::LatheInsert,
                    degree: 35.0,
                },
                placeholder_tool: PlaceHolderTool {
                    name: "Empty tool".to_string(),
                    id: uuid::Uuid::new_v4(),
                    location_id: uuid::Uuid::new_v4(),
                    location_slot: 0,
                    category: ToolCategory::Empty,
                },
                magazine: Magazine {
                    name: "magazine".to_string(),
                    id: uuid::Uuid::new_v4(),
                    location_id: uuid::Uuid::new_v4(),
                    location_slot: 0,
                    tools: Vec::new(),
                    holders: Vec::new(),
                    adapters: Vec::new(),
                    capacity: 10,
                },
                machine: Machine {
                    name: "machine".to_string(),
                    id: uuid::Uuid::new_v4(),
                    magazine_count: 1,
                    magazines: Vec::new(),
                },
                chosen_magazine_content: MagazineContent::default(),
            },
            library: Library::default(),
            machines: Vec::new(),
            app_state: ApplicationState::Home,
            popup_state: PopupState::None,
            selected_machine: (uuid::Uuid::new_v4(), 0),
            selected_magazine: (uuid::Uuid::new_v4(), 0),
        }
    }
}

impl ManagingApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for ManagingApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            library,
            gui_resource,
            machines,
            app_state: _,
            popup_state,
            selected_machine,
            selected_magazine,
        } = self;

        // #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
                select_machine(machines, selected_machine, ui);
                select_magazine(machines, selected_machine, selected_magazine, ui);
            });
        });

        match popup_state {
            PopupState::AddMachine => add_machine(gui_resource, machines, popup_state, ctx),
            PopupState::AddTool => add_tool(gui_resource, library, popup_state, ctx),
            PopupState::AddHolder => {}
            PopupState::AddAdapter => {}
            PopupState::None => {}
        }
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            // Buttons to change state for adding stuff
            if ui.button("Add machine").clicked() {
                self.popup_state = PopupState::AddMachine;
            }
            if ui.button("Add tool").clicked() {
                self.popup_state = PopupState::AddTool;
            }
            if ui.button("Add holder").clicked() {
                self.popup_state = PopupState::AddHolder;
            }
            if ui.button("Add adapter").clicked() {
                self.popup_state = PopupState::AddAdapter;
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            display_machine_ui(
                machines,
                selected_machine,
                selected_magazine,
                ui,
                gui_resource,
            );
        });
    }
}

pub fn select_machine(
    machines: &mut Vec<Machine>,
    selected_machine: &mut (uuid::Uuid, usize),
    ui: &mut egui::Ui,
) {
    // ComboBox to select machine and set selected_machine

    let name = if machines.len() > 0 {
        &machines[selected_machine.1].name
    } else {
        "No machines"
    };
    egui::ComboBox::from_label("Select machine")
        .selected_text(name)
        .show_ui(ui, |ui| {
            for (i, m) in machines.iter().enumerate() {
                if ui
                    .selectable_label(selected_machine.0 == m.id, m.name.clone())
                    .clicked()
                {
                    selected_machine.1 = i;
                    selected_machine.0 = machines[i].id;
                }
            }
        });
}

pub fn select_magazine(
    machines: &mut Vec<Machine>,
    selected_machine: &mut (uuid::Uuid, usize),
    selected_magazine: &mut (uuid::Uuid, usize),
    ui: &mut egui::Ui,
) {
    if machines.len() < selected_machine.1 || machines.len() == 0 {
        return;
    }
    let machine = &machines[selected_machine.1];

    if machine.magazines.len() > 0 {
        let name = &machine.magazines[selected_magazine.1].name;
        egui::ComboBox::from_label("Select magazine")
            .selected_text(name)
            .show_ui(ui, |ui| {
                for (i, m) in machine.magazines.iter().enumerate() {
                    if ui
                        .selectable_label(selected_magazine.0 == m.id, m.name.clone())
                        .clicked()
                    {
                        selected_magazine.0 = machine.magazines[i].id;
                        selected_magazine.1 = i;
                    }
                }
            });
    }
}

pub fn display_machine_ui(
    machines: &mut Vec<Machine>,
    selected_machine: &mut (uuid::Uuid, usize),
    selected_magazine: &mut (uuid::Uuid, usize),
    ui: &mut egui::Ui,
    gui_resource: &mut GuiResource,
) {
    if machines.len() < selected_machine.1 || machines.len() == 0 {
        return;
    } else {
        let current_machine = &machines[selected_machine.1];
        let current_magazine = current_machine.magazines.get(selected_magazine.1);

        ui.heading(format!("Machine: {}", current_machine.name));
        ui.horizontal(|ui| {
            ui.label(format!(
                "Magazine count: {}",
                current_machine.magazine_count
            ));
            ui.label(format!(
                "with capacity: {}",
                current_machine.magazines.len()
            ));
        });

        if let Some(current_magazine) = current_magazine {
            // Choose what magazine content to display: tools, holders or adapters with radio buttons
            ui.horizontal(|ui| {
                ui.radio_value(
                    &mut gui_resource.chosen_magazine_content,
                    MagazineContent::ToolContent,
                    "Tools",
                );
                ui.radio_value(
                    &mut gui_resource.chosen_magazine_content,
                    MagazineContent::HolderContent,
                    "Holders",
                );
                ui.radio_value(
                    &mut gui_resource.chosen_magazine_content,
                    MagazineContent::AdapterContent,
                    "Adapters",
                );
            });
            // call function to print magazine vectors based on chosen_magazine_content
        }
    }
}
