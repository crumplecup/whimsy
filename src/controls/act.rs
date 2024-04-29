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

    pub fn from_str(key: &str) -> Option<Self> {
        match key {
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
}

impl Stringly for AppAct {
    fn from_str(input: &str) -> Option<Self> {
        Self::from_str(input)
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

    pub fn from_str(key: &str) -> Option<Self> {
        match key {
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
}

impl Stringly for EguiAct {
    fn from_str(input: &str) -> Option<Self> {
        Self::from_str(input)
    }
}

pub trait Stringly
where
    Self: Sized,
{
    fn from_str(input: &str) -> Option<Self>;
}
