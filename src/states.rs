pub use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum ApplicationState {
    Home,
    Library,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum PopupState {
    None,
    AddMachine,
    AddTool,
    AddHolder,
    AddAdapter,
    DisplayLibrary,
}
