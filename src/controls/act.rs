use crate::prelude::Command;
use polite::Polite;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    EnumIter,
    Deserialize,
    Serialize,
)]
pub enum Act {
    App(AppAct),
    Egui(EguiAct),
    Named(NamedAct),
    #[default]
    Be,
}

impl Act {
    pub fn new() -> Self {
        Self::default()
    }

    fn from_vec<T: Into<Act> + Clone>(act: &[T]) -> Vec<Self> {
        act.iter().map(|a| a.clone().into()).collect::<Vec<Act>>()
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

#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    EnumIter,
    Deserialize,
    Serialize,
)]
pub enum AppAct {
    Help,
    Menu,
    Decorations,
    Fullscreen,
    Maximize,
    Minimize,
    #[default]
    Be,
}

impl AppAct {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Stringly for AppAct {
    fn from_str(input: &str) -> Option<Self> {
        match input {
            "help" => Some(Self::Help),
            "menu" => Some(Self::Menu),
            "decorations" => Some(Self::Decorations),
            "fullscreen" => Some(Self::Fullscreen),
            "maximize" => Some(Self::Maximize),
            "minimize" => Some(Self::Minimize),
            "be" => Some(Self::Be),
            _ => None,
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Self::Help => "Help",
            Self::Menu => "Menu",
            Self::Decorations => "Decorations",
            Self::Fullscreen => "Fullscreen",
            Self::Maximize => "Maximize",
            Self::Minimize => "Minimize",
            Self::Be => "Be",
        }
    }
}

#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    EnumIter,
    Deserialize,
    Serialize,
)]
pub enum EguiAct {
    Right,
    Left,
    Up,
    Down,
    Next,
    Previous,
    NextWindow,
    PreviousWindow,
    #[default]
    Be,
}

impl EguiAct {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Stringly for EguiAct {
    fn from_str(input: &str) -> Option<Self> {
        match input {
            "right" => Some(Self::Right),
            "left" => Some(Self::Left),
            "up" => Some(Self::Up),
            "down" => Some(Self::Down),
            "next" => Some(Self::Next),
            "previous" => Some(Self::Previous),
            "next_window" => Some(Self::NextWindow),
            "previous_window" => Some(Self::PreviousWindow),
            "be" => Some(Self::Be),
            _ => None,
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Self::Right => "Right",
            Self::Left => "Left",
            Self::Up => "Up",
            Self::Down => "Down",
            Self::Next => "Next",
            Self::Previous => "Previous",
            Self::NextWindow => "Next Window",
            Self::PreviousWindow => "Previous Window",
            Self::Be => "Be",
        }
    }
}

#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
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
}

impl Stringly for NamedAct {
    fn from_str(input: &str) -> Option<Self> {
        match input {
            "enter" => Some(Self::Enter),
            "escape" => Some(Self::Escape),
            "arrow_left" => Some(Self::ArrowLeft),
            "arrow_right" => Some(Self::ArrowRight),
            "arrow_up" => Some(Self::ArrowUp),
            "arrow_down" => Some(Self::ArrowDown),
            "be" => Some(Self::Be),
            _ => None,
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Self::Enter => "Enter",
            Self::Escape => "Escape",
            Self::ArrowLeft => "Arrow Left",
            Self::ArrowRight => "Arrow Right",
            Self::ArrowUp => "Arrow Up",
            Self::ArrowDown => "Arrow Down",
            Self::Be => "Be",
        }
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


pub trait Stringly
where
    Self: Sized,
{
    fn from_str(input: &str) -> Option<Self>;
    fn to_str(&self) -> &str;
}
