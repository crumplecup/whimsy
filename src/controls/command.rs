use crate::prelude::{Act, AppAct, EguiAct, Stringly};
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, space0};
use nom::combinator::opt;
use nom::sequence::delimited;
use nom::IResult;
use polite::{FauxPas, Polite};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::EnumIter;
use toml::{Table, Value};
use tracing::{info, trace, warn};
use winit::keyboard::ModifiersState;

#[derive(
    Debug, Default, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize,
)]
pub struct Modifiers {
    pub shift_key: bool,
    pub control_key: bool,
    pub alt_key: bool,
    pub super_key: bool,
}

impl Modifiers {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn is_none(&self) -> bool {
        self.shift_key && self.control_key && self.alt_key && self.super_key
    }

    pub fn and(&mut self, other: &Modifiers) {
        if other.shift_key {
            self.shift_key = true;
        }
        if other.control_key {
            self.control_key = true;
        }
        if other.alt_key {
            self.alt_key = true;
        }
        if other.super_key {
            self.super_key = true;
        }
    }
}

impl From<&ModifiersState> for Modifiers {
    fn from(mods: &ModifiersState) -> Self {
        let mut result = Self::new();
        if mods.shift_key() {
            result.shift_key = true;
        }
        if mods.control_key() {
            result.control_key = true;
        }
        if mods.alt_key() {
            result.alt_key = true;
        }
        if mods.super_key() {
            result.super_key = true;
        }
        result
    }
}

impl From<&Option<ModifiersState>> for Modifiers {
    fn from(mods: &Option<ModifiersState>) -> Self {
        let mut m = Modifiers::new();
        if let Some(n) = mods {
            m = n.into();
        }
        m
    }
}

impl From<&Command> for Modifiers {
    fn from(cmd: &Command) -> Self {
        Self::from(cmd.mods)
    }
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Command {
    pub key: String,
    pub mods: Modifiers,
}

impl Command {
    pub fn new(key: &str, mods: &ModifiersState) -> Self {
        Self {
            key: key.to_owned(),
            mods: mods.into(),
        }
    }

    pub fn with_modifier(key: &str, mods: &Modifiers) -> Self {
        Self {
            key: key.to_owned(),
            mods: mods.to_owned(),
        }
    }

    pub fn into_mods(input: &str) -> Option<ModifiersState> {
        match input {
            "cr" => Some(ModifiersState::CONTROL),
            "control" => Some(ModifiersState::CONTROL),
            "alt" => Some(ModifiersState::ALT),
            "shift" => Some(ModifiersState::SHIFT),
            "super" => Some(ModifiersState::SUPER),
            _ => None,
        }
    }

    pub fn is_modifier(input: &str) -> IResult<&str, bool> {
        let (input, _) = Self::separator(input)?;
        let (_, mods) = opt(Self::parse_mod)(input)?;
        if let Some(_) = mods {
            Ok((input, true))
        } else {
            Ok((input, false))
        }
    }

    pub fn word(input: &str) -> IResult<&str, &str> {
        let (rem, _) = space0(input)?;
        alphanumeric1(rem)
    }

    pub fn separator(input: &str) -> IResult<&str, bool> {
        let mut separated = false;
        let (rem, _) = space0(input)?;
        let (rem, sep) = opt(tag("+"))(rem)?;
        if let Some(s) = sep {
            separated = true;
        }
        let (rem, _) = space0(rem)?;
        Ok((rem, separated))
    }

    pub fn parse_mod(input: &str) -> IResult<&str, Option<ModifiersState>> {
        let (input, _) = Self::separator(input)?;
        let (rem, bracketed) = delimited(tag("<"), alphanumeric1, tag(">"))(input)?;
        let (rem, _) = Self::separator(rem)?;
        let bracketed = Self::into_mods(bracketed);
        Ok((rem, bracketed))
    }

    pub fn parse_mods(input: &str) -> IResult<&str, Modifiers> {
        let mut modifiers = Modifiers::new();
        let mut i = input;
        let (_, mut more) = Self::is_modifier(i)?;
        while more {
            let (rem, m) = Self::parse_mod(i)?;
            modifiers.and(&Modifiers::from(&m));
            let (_, check) = Self::is_modifier(rem)?;
            more = check;
            i = rem;
        }
        Ok((i, modifiers))
    }

    pub fn parse_str(input: &str) -> IResult<&str, Option<Self>> {
        let (rem, mods) = Self::parse_mods(input)?;
        let (rem, key) = Self::word(rem)?;
        let command = Command::with_modifier(key, &mods);
        Ok((rem, Some(command)))
    }

    pub fn parse_cmd(input: &str) -> Polite<Self> {
        let (rem, opt) = Self::parse_str(input)?;
        if let Some(mut cmd) = opt {
            if cmd.key == cmd.key.to_uppercase() {
                cmd.mods.shift_key = true;
            }
            Ok(cmd)
        } else {
            Err(FauxPas::Nom(rem.to_string()))
        }
    }

    pub fn act(&self, trigger: &Command) -> bool {
        self == trigger
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum CommandOptions {
    Commands(Vec<Command>),
    Acts(Vec<Act>),
}

impl CommandOptions {
    pub fn with_act<T: Into<Act>>(&mut self, act: T) {
        match self {
            Self::Commands(_) => warn!("Not an Acts variant!"),
            Self::Acts(acts) => acts.push(act.into()),
        }
    }

    pub fn with_command(&mut self, command: Command) {
        match self {
            Self::Commands(commands) => commands.push(command),
            Self::Acts(_) => warn!("Not a Commands variant!"),
        }
    }
}

impl<T: Into<Act>> From<T> for CommandOptions {
    fn from(act: T) -> Self {
        let mut acts = Vec::new();
        acts.push(act.into());
        Self::Acts(acts)
    }
}

impl<T: Into<Act> + Clone> From<&[T]> for CommandOptions {
    fn from(acts: &[T]) -> Self {
        let a = acts.iter().map(|v| v.clone().into()).collect::<Vec<Act>>();
        Self::Acts(a)
    }
}

impl<T: Into<Act> + Clone> From<Vec<T>> for CommandOptions {
    fn from(acts: Vec<T>) -> Self {
        Self::from(&acts[..])
    }
}

impl From<Command> for CommandOptions {
    fn from(command: Command) -> Self {
        let mut commands = Vec::new();
        commands.push(command);
        Self::Commands(commands)
    }
}

impl From<Vec<Command>> for CommandOptions {
    fn from(commands: Vec<Command>) -> Self {
        Self::Commands(commands)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandMode {
    Normal(Choices),
}

impl CommandMode {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn choices(&self) -> &Choices {
        match self {
            Self::Normal(choices) => choices,
        }
    }
}

impl Default for CommandMode {
    fn default() -> Self {
        let choices = Choices::default();
        Self::Normal(choices)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Choices(HashMap<Command, CommandOptions>);

impl Choices {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_toml<T: Into<Act> + Clone + Stringly>(value: &Value) -> Polite<Self> {
        info!("{:#?}", value);
        match value {
            Value::Table(t) => {
                let mut choices = HashMap::new();
                let command_queue = t.keys().map(|k| k.clone()).collect::<Vec<String>>();
                for key in command_queue {
                    info!("Reading {}", &key);
                    if let Value::String(s) = &value[&key] {
                        let s = s.to_owned();
                        let command = Command::parse_cmd(&s)?;
                        info!("Command result: {:#?}", &command);
                        let act = T::from_str(&key);
                        if let Some(a) = act {
                            let opts = CommandOptions::from(vec![a]);
                            choices.insert(command, opts);
                        }
                    }
                }
                Ok(Self(choices))
            }
            v => {
                info!("Command not recognized: {}", v);
                Err(FauxPas::Unknown)
            }
        }
    }

    pub fn with_config() -> Polite<Self> {
        let config = include_bytes!("../../config.toml");
        trace!("Config read: {} u8.", config.len());
        let stringly = String::from_utf8_lossy(config);
        let config = stringly.parse::<Table>().unwrap();
        trace!("Config read: {}", config);
        // let command_queue = vec!["right", "left", "up", "down"];
        let egui = &config["egui"];
        let winit = &config["winit"];
        let mut choices = Self::from_toml::<EguiAct>(&egui)?;
        let winit = Self::from_toml::<AppAct>(&winit)?;
        choices.value_mut().extend(winit.value().clone());
        trace!("Choices: {:#?}", choices);
        Ok(choices)
    }

    pub fn value(&self) -> &HashMap<Command, CommandOptions> {
        match self {
            Self(data) => data,
        }
    }

    pub fn value_mut(&mut self) -> &mut HashMap<Command, CommandOptions> {
        match self {
            Self(data) => data,
        }
    }
}

impl Default for Choices {
    fn default() -> Self {
        Choices::with_config().unwrap()
    }
}
