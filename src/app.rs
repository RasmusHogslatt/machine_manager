use crate::{
    adapter::*,
    adapter_placeholder::AdapterPlaceHolder,
    circular_insert::CircularInsert,
    collet::Collet,
    diamond_insert::DiamondInsert,
    drawable::*,
    drill::Drill,
    drill_chuck::DrillChuck,
    end_mill::EndMill,
    external_turning::ExternalTurning,
    grooving_parting::GroovingParting,
    holder_placeholder::HolderPlaceHolder,
    holders::holder::{Holder, HolderCategory},
    hydraulic::Hydraulic,
    internal_turning::InternalTurning,
    library::*,
    machine::*,
    magazine::*,
    mill::Mill,
    resources::*,
    shrink_fit::ShrinkFit,
    side_lock::SideLock,
    square_insert::SquareInsert,
    states::*,
    tapping::Tapping,
    threading::Threading,
    tool::{Tool, ToolCategory},
    tool_placeholder::ToolPlaceHolder,
    triangle_insert::TriangleInsert,
    trigon_insert::TrigonInsert,
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
                tool_selected: Tool::ToolPlaceHolder(ToolPlaceHolder {
                    name: "Empty tool".to_string(),
                    id: uuid::Uuid::new_v4(),
                    location_id: uuid::Uuid::new_v4(),
                    location_slot: 0,
                    category: ToolCategory::Empty,
                }),
                holder_selected: Holder::HolderPlaceHolder(HolderPlaceHolder {
                    name: "Empty holder".to_string(),
                    id: uuid::Uuid::new_v4(),
                    location_id: uuid::Uuid::new_v4(),
                    location_slot: 0,
                    category: HolderCategory::Empty,
                }),
                adapter_selected: Adapter::AdapterPlaceHolder(AdapterPlaceHolder {
                    name: "Empty adapter".to_string(),
                    id: uuid::Uuid::new_v4(),
                    location_id: uuid::Uuid::new_v4(),
                    location_slot: 1,
                    category: AdapterCategory::Empty,
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
                trigon_insert: crate::trigon_insert::TrigonInsert {
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
                placeholder_tool: ToolPlaceHolder {
                    name: "Empty tool".to_string(),
                    id: uuid::Uuid::new_v4(),
                    location_id: uuid::Uuid::new_v4(),
                    location_slot: 0,
                    category: ToolCategory::Empty,
                },
                collet_holder: Collet {
                    name: "collet holder".to_string(),
                    id: uuid::Uuid::new_v4(),
                    location_id: uuid::Uuid::new_v4(),
                    location_slot: 0,
                    category: HolderCategory::MillingHolder,
                    collet_type_name: "ET20".to_string(),
                    collet_size: 1.0,
                },
                end_mill_holder: EndMill {
                    name: "end mill holder".to_string(),
                    id: uuid::Uuid::new_v4(),
                    location_id: uuid::Uuid::new_v4(),
                    location_slot: 0,
                    category: HolderCategory::MillingHolder,
                },
                drill_chuck_holder: DrillChuck {
                    name: "drill chuck holder".to_string(),
                    id: uuid::Uuid::new_v4(),
                    location_id: uuid::Uuid::new_v4(),
                    location_slot: 0,
                    category: HolderCategory::DrillingHolder,
                },
                external_turning_holder: ExternalTurning {
                    name: "external turning holder".to_string(),
                    id: uuid::Uuid::new_v4(),
                    location_id: uuid::Uuid::new_v4(),
                    location_slot: 0,
                    category: HolderCategory::TurningHolder,
                },
                internal_turning_holder: InternalTurning {
                    name: "internal turning holder".to_string(),
                    id: uuid::Uuid::new_v4(),
                    location_id: uuid::Uuid::new_v4(),
                    location_slot: 0,
                    category: HolderCategory::TurningHolder,
                },
                grooving_parting_holder: GroovingParting {
                    name: "grooving parting holder".to_string(),
                    id: uuid::Uuid::new_v4(),
                    location_id: uuid::Uuid::new_v4(),
                    location_slot: 0,
                    category: HolderCategory::SpecialtyHolder,
                },
                threading_holder: Threading {
                    name: "threading holder".to_string(),
                    id: uuid::Uuid::new_v4(),
                    location_id: uuid::Uuid::new_v4(),
                    location_slot: 0,
                    category: HolderCategory::SpecialtyHolder,
                },
                tapping_holder: Tapping {
                    name: "tapping holder".to_string(),
                    id: uuid::Uuid::new_v4(),
                    location_id: uuid::Uuid::new_v4(),
                    location_slot: 0,
                    category: HolderCategory::SpecialtyHolder,
                },
                placeholder_holder: HolderPlaceHolder {
                    name: "Empty holder".to_string(),
                    id: uuid::Uuid::new_v4(),
                    location_id: uuid::Uuid::new_v4(),
                    location_slot: 0,
                    category: HolderCategory::Empty,
                },
                shrink_fit_adapter: ShrinkFit {
                    name: "shrink fit adapter".to_string(),
                    id: uuid::Uuid::new_v4(),
                    location_id: uuid::Uuid::new_v4(),
                    location_slot: 1,
                    category: AdapterCategory::Standard,
                },
                side_lock_adapter: SideLock {
                    name: "side lock adapter".to_string(),
                    id: uuid::Uuid::new_v4(),
                    location_id: uuid::Uuid::new_v4(),
                    location_slot: 1,
                    category: AdapterCategory::Standard,
                },
                hydraulic_adapter: Hydraulic {
                    name: "hydraulic adapter".to_string(),
                    id: uuid::Uuid::new_v4(),
                    location_id: uuid::Uuid::new_v4(),
                    location_slot: 1,
                    category: AdapterCategory::Standard,
                },
                placeholder_adapter: AdapterPlaceHolder {
                    name: "Empty adapter".to_string(),
                    id: uuid::Uuid::new_v4(),
                    location_id: uuid::Uuid::new_v4(),
                    location_slot: 1,
                    category: AdapterCategory::Empty,
                },
                magazine: Magazine {
                    name: "magazine".to_string(),
                    id: uuid::Uuid::new_v4(),
                    location_id: uuid::Uuid::new_v4(),
                    location_slot: 0,
                    contents: Vec::new(),
                    capacity: 10,
                },
                machine: Machine {
                    name: "machine".to_string(),
                    id: uuid::Uuid::new_v4(),
                    magazine_count: 1,
                    magazines: Vec::new(),
                },
                chosen_magazine_content: MagazineContent::default(),
                chosen_library_content: LibraryContent::default(),
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
        // egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        //     // The top panel is often a good place for a menu bar:
        //     egui::menu::bar(ui, |ui| {
        //         ui.menu_button("File", |ui| {
        //             if ui.button("Quit").clicked() {
        //                 _frame.close();
        //             }
        //         });
        //     });
        // });

        match popup_state {
            PopupState::AddMachine => add_machine(gui_resource, machines, popup_state, ctx),
            PopupState::AddTool => add_tool(gui_resource, library, popup_state, ctx),
            PopupState::AddHolder => add_holder(gui_resource, library, popup_state, ctx),
            PopupState::AddAdapter => add_adapter(gui_resource, library, popup_state, ctx),
            PopupState::None => {}
            PopupState::DisplayLibrary => display_library(ctx, gui_resource, library, popup_state),
        }

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Machine Manager");
            select_machine(machines, selected_machine, ui);
            select_magazine(machines, selected_machine, selected_magazine, ui);
            ui.separator();
            // Buttons to change state for adding stuff
            if ui.button("Show library").clicked() {
                self.popup_state = PopupState::DisplayLibrary;
            }
            ui.separator();
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
    let name = if !machines.is_empty() {
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
    if machines.len() < selected_machine.1 || machines.is_empty() {
        return;
    }
    let machine = &machines[selected_machine.1];

    if !machine.magazines.is_empty() {
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
    if machines.len() < selected_machine.1 || machines.is_empty() {
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

        if let Some(_current_magazine) = current_magazine {
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

pub fn display_library(
    ctx: &egui::Context,
    gui_resource: &mut GuiResource,
    library: &mut Library,
    popup_state: &mut PopupState,
) {
    if popup_state != &PopupState::DisplayLibrary {
        return;
    }
    egui::Window::new("Library").show(ctx, |ui| {
        ui.horizontal(|ui| {
            // use radio buttons to choose what to display: (tools, holders, adapters)
            ui.radio_value(
                &mut gui_resource.chosen_library_content,
                LibraryContent::ToolContent,
                "Tools",
            );
            ui.radio_value(
                &mut gui_resource.chosen_library_content,
                LibraryContent::HolderContent,
                "Holders",
            );
            ui.radio_value(
                &mut gui_resource.chosen_library_content,
                LibraryContent::AdapterContent,
                "Adapters",
            );
        });
        match gui_resource.chosen_library_content {
            LibraryContent::ToolContent => {
                // print tools
                for tool in &mut library.tools {
                    ui.horizontal(|ui| {
                        tool.draw_display(ui);
                        // show slot position in library tools vector

                        if ui.button("Delete").clicked() {
                            // Delete tool. Moving to library should be done from display_magazine
                        }
                    });
                }
            }
            LibraryContent::HolderContent => {
                // print holders
                for holder in &mut library.holders {
                    holder.draw_display(ui);
                }
            }
            LibraryContent::AdapterContent => {
                // print adapters
                for adapter in &mut library.adapters {
                    adapter.draw_display(ui);
                }
            }
        }
        if ui.button("Close").clicked() {
            *popup_state = PopupState::None;
        }
    });
}

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

pub fn add_holder(
    gui_resource: &mut GuiResource,
    library: &mut Library,
    popup_state: &mut PopupState,
    ctx: &egui::Context,
) {
    if popup_state != &PopupState::AddHolder {
        return;
    }
    egui::Window::new("Add holder").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.radio_value(
                &mut gui_resource.holder_category,
                HolderCategory::MillingHolder,
                "Milling",
            );
            ui.radio_value(
                &mut gui_resource.holder_category,
                HolderCategory::DrillingHolder,
                "Drilling",
            );
            ui.radio_value(
                &mut gui_resource.holder_category,
                HolderCategory::TurningHolder,
                "Turning",
            );
            ui.radio_value(
                &mut gui_resource.holder_category,
                HolderCategory::SpecialtyHolder,
                "Specialty",
            );
        });
        match gui_resource.holder_category {
            HolderCategory::Empty => {}
            HolderCategory::MillingHolder => {
                ui.horizontal(|ui| {
                    ui.radio_value(
                        &mut gui_resource.holder_selected,
                        Holder::Collet(Collet::default()),
                        "Collet Holder",
                    );
                    ui.radio_value(
                        &mut gui_resource.holder_selected,
                        Holder::EndMill(EndMill::default()),
                        "End Mill Holder",
                    );
                });
            }
            HolderCategory::DrillingHolder => {
                ui.horizontal(|ui| {
                    ui.radio_value(
                        &mut gui_resource.holder_selected,
                        Holder::DrillChuck(DrillChuck::default()),
                        "Drill Chuck Holder",
                    );
                });
            }
            HolderCategory::TurningHolder => {
                ui.horizontal(|ui| {
                    ui.radio_value(
                        &mut gui_resource.holder_selected,
                        Holder::ExternalTurning(ExternalTurning::default()),
                        "External Turning Holder",
                    );
                    ui.radio_value(
                        &mut gui_resource.holder_selected,
                        Holder::InternalTurning(InternalTurning::default()),
                        "Internal Turning Holder",
                    );
                });
            }
            HolderCategory::SpecialtyHolder => {
                ui.horizontal(|ui| {
                    ui.radio_value(
                        &mut gui_resource.holder_selected,
                        Holder::GroovingParting(GroovingParting::default()),
                        "Grooving/Parting Holder",
                    );
                    ui.radio_value(
                        &mut gui_resource.holder_selected,
                        Holder::Threading(Threading::default()),
                        "Threading Holder",
                    );
                    ui.radio_value(
                        &mut gui_resource.holder_selected,
                        Holder::Tapping(Tapping::default()),
                        "Tapping Holder",
                    );
                });
            }
        }
        match (&gui_resource.holder_selected, &gui_resource.holder_category) {
            (Holder::Collet(_), HolderCategory::MillingHolder) => {
                gui_resource
                    .collet_holder
                    .draw_adding_to_library(library, popup_state, ui);
            }

            (Holder::EndMill(_), HolderCategory::MillingHolder) => {
                gui_resource
                    .end_mill_holder
                    .draw_adding_to_library(library, popup_state, ui);
            }

            (Holder::DrillChuck(_), HolderCategory::DrillingHolder) => {
                gui_resource
                    .drill_chuck_holder
                    .draw_adding_to_library(library, popup_state, ui);
            }

            (Holder::ExternalTurning(_), HolderCategory::TurningHolder) => {
                gui_resource.external_turning_holder.draw_adding_to_library(
                    library,
                    popup_state,
                    ui,
                );
            }

            (Holder::InternalTurning(_), HolderCategory::TurningHolder) => {
                gui_resource.internal_turning_holder.draw_adding_to_library(
                    library,
                    popup_state,
                    ui,
                );
            }

            (Holder::GroovingParting(_), HolderCategory::SpecialtyHolder) => {
                gui_resource.grooving_parting_holder.draw_adding_to_library(
                    library,
                    popup_state,
                    ui,
                );
            }

            (Holder::Threading(_), HolderCategory::SpecialtyHolder) => {
                gui_resource
                    .threading_holder
                    .draw_adding_to_library(library, popup_state, ui);
            }

            (_, _) => {}
        }
    });
}

pub fn add_adapter(
    gui_resource: &mut GuiResource,
    library: &mut Library,
    popup_state: &mut PopupState,
    ctx: &egui::Context,
) {
    if popup_state != &PopupState::AddAdapter {
        return;
    }
    egui::Window::new("Add adapter").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.radio_value(
                &mut gui_resource.adapter_category,
                AdapterCategory::Standard,
                "Standard",
            );
        });
        match gui_resource.adapter_category {
            AdapterCategory::Empty => {}
            AdapterCategory::Standard => {
                ui.horizontal(|ui| {
                    ui.radio_value(
                        &mut gui_resource.adapter_selected,
                        Adapter::ShrinkFit(Default::default()),
                        "ShrinkFitAdapter",
                    );
                    ui.radio_value(
                        &mut gui_resource.adapter_selected,
                        Adapter::SideLock(Default::default()),
                        "SideLockAdapter",
                    );
                    ui.radio_value(
                        &mut gui_resource.adapter_selected,
                        Adapter::Hydraulic(Default::default()),
                        "HydraulicAdapter",
                    );
                });
            }
        }
        match (
            &gui_resource.adapter_selected,
            &gui_resource.adapter_category,
        ) {
            (Adapter::ShrinkFit(_), AdapterCategory::Standard) => {
                gui_resource
                    .shrink_fit_adapter
                    .draw_adding_to_library(library, popup_state, ui);
            }
            (Adapter::SideLock(_), AdapterCategory::Standard) => {
                gui_resource
                    .side_lock_adapter
                    .draw_adding_to_library(library, popup_state, ui);
            }
            (Adapter::Hydraulic(_), AdapterCategory::Standard) => {
                gui_resource
                    .hydraulic_adapter
                    .draw_adding_to_library(library, popup_state, ui);
            }
            (_, _) => {}
        }
    });
}
