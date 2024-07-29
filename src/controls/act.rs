//! The `act` module encapsulates the event handling model for the application by classifying
//! application functions as variants of the `Act` enum.
use serde::{Deserialize, Serialize};
use std::{cmp, fmt};
use strum_macros::EnumIter;

/// The `Act` enum delineates the types of application functions that are accessible to the user.
/// The `command` module maps keyboard inputs to specific variants of the `Act` enum as an action
/// handling model.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, EnumIter, Deserialize, Serialize)]
pub enum Act {
    /// Event handlers for the `winit` library.
    App(AppAct),
    /// Event handlers for the `egui` library.
    Egui(EguiAct),
    /// Event handlers for named keys.
    Named(NamedAct),
    /// Event handlers for the `egui_dock` library.
    Dock(Dock),
    /// A no-op action.
    #[default]
    Be,
}

impl Act {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn idx(&self) -> usize {
        match self {
            Self::App(act) => act.idx(),
            Self::Egui(act) => act.idx() + 100,
            Self::Named(act) => act.idx() + 200,
            Self::Dock(act) => act.idx() + 300,
            Self::Be => 999,
        }
    }
}

impl PartialOrd for Act {
    fn partial_cmp(&self, other: &Act) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Act {
    fn cmp(&self, other: &Act) -> cmp::Ordering {
        let self_id = self.idx();
        let other_id = other.idx();
        self_id.cmp(&other_id)
    }
}

impl fmt::Display for Act {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::App(act) => write!(f, "{}", act),
            Self::Egui(act) => write!(f, "{}", act),
            Self::Named(act) => write!(f, "{}", act),
            Self::Dock(act) => write!(f, "{}", act),
            Self::Be => write!(f, "Be"),
        }
    }
}

impl std::str::FromStr for Act {
    type Err = polite::FauxPas;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(act) = AppAct::from_str(s) {
            Ok(Self::App(act))
        } else if let Ok(act) = EguiAct::from_str(s) {
            Ok(Self::Egui(act))
        } else if let Ok(act) = NamedAct::from_str(s) {
            Ok(Self::Named(act))
        } else if let Ok(act) = Dock::from_str(s) {
            Ok(Self::Dock(act))
        } else if &s.to_lowercase() == "be" {
            Ok(Self::Be)
        } else {
            Err(polite::FauxPas::Unknown)
        }
    }
}

impl From<AppAct> for Act {
    fn from(act: AppAct) -> Self {
        match act {
            AppAct::Be => Self::Be,
            other => Self::App(other),
        }
    }
}

impl From<&AppAct> for Act {
    fn from(act: &AppAct) -> Self {
        match act {
            AppAct::Be => Self::Be,
            other => Self::App(*other),
        }
    }
}

impl From<EguiAct> for Act {
    fn from(act: EguiAct) -> Self {
        match act {
            EguiAct::Be => Self::Be,
            other => Self::Egui(other),
        }
    }
}

impl From<&EguiAct> for Act {
    fn from(act: &EguiAct) -> Self {
        match act {
            EguiAct::Be => Self::Be,
            other => Self::Egui(*other),
        }
    }
}

impl From<NamedAct> for Act {
    fn from(act: NamedAct) -> Self {
        match act {
            NamedAct::Be => Self::Be,
            other => Self::Named(other),
        }
    }
}

impl From<&NamedAct> for Act {
    fn from(act: &NamedAct) -> Self {
        match act {
            NamedAct::Be => Self::Be,
            other => Self::Named(*other),
        }
    }
}

impl From<Dock> for Act {
    fn from(act: Dock) -> Self {
        match act {
            Dock::Be => Self::Be,
            other => Self::Dock(other),
        }
    }
}

impl From<&Dock> for Act {
    fn from(act: &Dock) -> Self {
        match act {
            Dock::Be => Self::Be,
            other => Self::Dock(*other),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, EnumIter, Deserialize, Serialize)]
pub enum AppAct {
    Help,
    Menu,
    Decorations,
    Fullscreen,
    Maximize,
    Minimize,
    ActiveTab,
    #[default]
    Be,
}

impl AppAct {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn idx(&self) -> usize {
        match self {
            Self::Help => 0,
            Self::Menu => 1,
            Self::Decorations => 2,
            Self::Fullscreen => 3,
            Self::Maximize => 4,
            Self::Minimize => 5,
            Self::ActiveTab => 6,
            Self::Be => 7,
        }
    }
}

impl PartialOrd for AppAct {
    fn partial_cmp(&self, other: &AppAct) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AppAct {
    fn cmp(&self, other: &AppAct) -> cmp::Ordering {
        let self_id = self.idx();
        let other_id = other.idx();
        self_id.cmp(&other_id)
    }
}

impl fmt::Display for AppAct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Help => write!(f, "Help"),
            Self::Menu => write!(f, "Menu"),
            Self::Decorations => write!(f, "Decorations"),
            Self::Fullscreen => write!(f, "Fullscreen"),
            Self::Maximize => write!(f, "Maximize"),
            Self::Minimize => write!(f, "Minimize"),
            Self::ActiveTab => write!(f, "Active Tab"),
            Self::Be => write!(f, "Be"),
        }
    }
}

// impl std::string::ToString for AppAct {
//     fn to_string(&self) -> String {
//         let str = match self {
//             Self::Help => "Help",
//             Self::Menu => "Menu",
//             Self::Decorations => "Decorations",
//             Self::Fullscreen => "Fullscreen",
//             Self::Maximize => "Maximize",
//             Self::Minimize => "Minimize",
//             Self::ActiveTab => "Active Tab",
//             Self::Be => "Be",
//         };
//         str.to_string()
//     }
// }

impl std::str::FromStr for AppAct {
    type Err = polite::FauxPas;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "help" => Ok(Self::Help),
            "menu" => Ok(Self::Menu),
            "decorations" => Ok(Self::Decorations),
            "fullscreen" => Ok(Self::Fullscreen),
            "maximize" => Ok(Self::Maximize),
            "minimize" => Ok(Self::Minimize),
            "active_tab" => Ok(Self::ActiveTab),
            "be" => Ok(Self::Be),
            _ => Err(polite::FauxPas::Unknown),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, EnumIter, Deserialize, Serialize)]
pub enum EguiAct {
    Right,
    Left,
    Up,
    Down,
    Next,
    Previous,
    NextWindow,
    PreviousWindow,
    NextRow,
    PreviousRow,
    FocusedLeaf,
    #[default]
    Be,
}

impl EguiAct {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn idx(&self) -> usize {
        match self {
            Self::Right => 0,
            Self::Left => 1,
            Self::Up => 2,
            Self::Down => 3,
            Self::Next => 4,
            Self::Previous => 5,
            Self::NextWindow => 6,
            Self::PreviousWindow => 7,
            Self::NextRow => 8,
            Self::PreviousRow => 9,
            Self::FocusedLeaf => 10,
            Self::Be => 11,
        }
    }
}

impl PartialOrd for EguiAct {
    fn partial_cmp(&self, other: &EguiAct) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EguiAct {
    fn cmp(&self, other: &EguiAct) -> cmp::Ordering {
        let self_id = self.idx();
        let other_id = other.idx();
        self_id.cmp(&other_id)
    }
}

impl fmt::Display for EguiAct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Right => write!(f, "Right"),
            Self::Left => write!(f, "Left"),
            Self::Up => write!(f, "Up"),
            Self::Down => write!(f, "Down"),
            Self::Next => write!(f, "Next"),
            Self::Previous => write!(f, "Previous"),
            Self::NextWindow => write!(f, "Next Window"),
            Self::PreviousWindow => write!(f, "Previous Window"),
            Self::NextRow => write!(f, "Next Row"),
            Self::PreviousRow => write!(f, "Previous Row"),
            Self::FocusedLeaf => write!(f, "Focused Leaf"),
            Self::Be => write!(f, "Be"),
        }
    }
}

impl std::str::FromStr for EguiAct {
    type Err = polite::FauxPas;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "right" => Ok(Self::Right),
            "left" => Ok(Self::Left),
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            "next" => Ok(Self::Next),
            "previous" => Ok(Self::Previous),
            "next_window" => Ok(Self::NextWindow),
            "previous_window" => Ok(Self::PreviousWindow),
            "next_row" => Ok(Self::NextRow),
            "previous_row" => Ok(Self::PreviousRow),
            "focused_leaf" => Ok(Self::FocusedLeaf),
            "be" => Ok(Self::Be),
            _ => Err(polite::FauxPas::Unknown),
        }
    }
}

#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    // PartialOrd,
    Eq,
    // Ord,
    Hash,
    EnumIter,
    Deserialize,
    Serialize,
)]
pub enum NamedAct {
    Enter,
    Escape,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    #[default]
    Be,
}

impl NamedAct {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cmd(&self) -> String {
        let value = match self {
            Self::Enter => "enter",
            Self::Escape => "escape",
            Self::ArrowUp => "arrow_up",
            Self::ArrowDown => "arrow_down",
            Self::ArrowLeft => "arrow_left",
            Self::ArrowRight => "arrow_right",
            Self::Be => "be",
        };
        value.to_owned()
    }

    pub fn idx(&self) -> usize {
        match self {
            Self::Enter => 0,
            Self::Escape => 1,
            Self::ArrowUp => 2,
            Self::ArrowDown => 3,
            Self::ArrowLeft => 4,
            Self::ArrowRight => 5,
            Self::Be => 6,
        }
    }
}

impl PartialOrd for NamedAct {
    fn partial_cmp(&self, other: &NamedAct) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NamedAct {
    fn cmp(&self, other: &NamedAct) -> cmp::Ordering {
        let self_id = self.idx();
        let other_id = other.idx();
        self_id.cmp(&other_id)
    }
}

impl From<&winit::keyboard::NamedKey> for NamedAct {
    fn from(named: &winit::keyboard::NamedKey) -> Self {
        match named {
            winit::keyboard::NamedKey::Enter => Self::Enter,
            winit::keyboard::NamedKey::Escape => Self::Escape,
            winit::keyboard::NamedKey::ArrowLeft => Self::ArrowLeft,
            winit::keyboard::NamedKey::ArrowRight => Self::ArrowRight,
            winit::keyboard::NamedKey::ArrowUp => Self::ArrowUp,
            winit::keyboard::NamedKey::ArrowDown => Self::ArrowDown,
            _ => Self::Be,
        }
    }
}

impl From<&winit::keyboard::Key> for NamedAct {
    fn from(named: &winit::keyboard::Key) -> Self {
        match named {
            winit::keyboard::Key::Named(k) => Self::from(k),
            _ => Self::Be,
        }
    }
}

impl fmt::Display for NamedAct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Enter => write!(f, "Enter"),
            Self::Escape => write!(f, "Escape"),
            Self::ArrowLeft => write!(f, "Arrow Left"),
            Self::ArrowRight => write!(f, "Arrow Right"),
            Self::ArrowUp => write!(f, "Arrow Up"),
            Self::ArrowDown => write!(f, "Arrow Down"),
            Self::Be => write!(f, "Be"),
        }
    }
}

impl std::str::FromStr for NamedAct {
    type Err = polite::FauxPas;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "enter" => Ok(Self::Enter),
            "escape" => Ok(Self::Escape),
            "arrow_left" => Ok(Self::ArrowLeft),
            "arrow_right" => Ok(Self::ArrowRight),
            "arrow_up" => Ok(Self::ArrowUp),
            "arrow_down" => Ok(Self::ArrowDown),
            "be" => Ok(Self::Be),
            _ => Err(polite::FauxPas::Unknown),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, EnumIter, Deserialize, Serialize)]
pub enum Dock {
    CurrentTab,
    NextTab,
    PreviousTab,
    NextNode,
    PreviousNode,
    NextSurface,
    PreviousSurface,
    InspectRecords,
    #[default]
    Be,
}

impl Dock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cmd(&self) -> String {
        let value = match self {
            Self::CurrentTab => "select_current",
            Self::NextTab => "next_tab",
            Self::PreviousTab => "previous_tab",
            Self::NextNode => "next_node",
            Self::PreviousNode => "previous_node",
            Self::NextSurface => "next_surface",
            Self::PreviousSurface => "previous_surface",
            Self::InspectRecords => "inspect_records",
            Self::Be => "be",
        };
        value.to_owned()
    }

    pub fn idx(&self) -> usize {
        match self {
            Self::CurrentTab => 0,
            Self::NextTab => 1,
            Self::PreviousTab => 2,
            Self::NextNode => 3,
            Self::PreviousNode => 4,
            Self::NextSurface => 5,
            Self::PreviousSurface => 6,
            Self::InspectRecords => 7,
            Self::Be => 8,
        }
    }
}

impl PartialOrd for Dock {
    fn partial_cmp(&self, other: &Dock) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Dock {
    fn cmp(&self, other: &Dock) -> cmp::Ordering {
        let self_id = self.idx();
        let other_id = other.idx();
        self_id.cmp(&other_id)
    }
}

impl fmt::Display for Dock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CurrentTab => write!(f, "Select Current"),
            Self::NextTab => write!(f, "Next Tab"),
            Self::PreviousTab => write!(f, "Previous Tab"),
            Self::NextNode => write!(f, "Next Node"),
            Self::PreviousNode => write!(f, "Previous Node"),
            Self::NextSurface => write!(f, "Next Surface"),
            Self::PreviousSurface => write!(f, "Previous Surface"),
            Self::InspectRecords => write!(f, "Inspect Records"),
            Self::Be => write!(f, "Be"),
        }
    }
}

impl std::str::FromStr for Dock {
    type Err = polite::FauxPas;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "select_current" => Ok(Self::CurrentTab),
            "next_tab" => Ok(Self::NextTab),
            "previous_tab" => Ok(Self::PreviousTab),
            "next_node" => Ok(Self::NextNode),
            "previous_node" => Ok(Self::PreviousNode),
            "next_surface" => Ok(Self::NextSurface),
            "previous_surface" => Ok(Self::PreviousSurface),
            "inspect_records" => Ok(Self::InspectRecords),
            "be" => Ok(Self::Be),
            _ => Err(polite::FauxPas::Unknown),
        }
    }
}
