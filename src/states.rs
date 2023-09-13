pub use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ApplicationState {
    Home,
    Library,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum PopupState {
    None,
    AddMachine,
    AddTool,
    AddHolder,
    AddAdapter,
}
