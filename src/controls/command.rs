use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, space0};
use nom::combinator::opt;
use nom::IResult;
use nom::sequence::delimited;
use polite::{Polite, FauxPas};
use std::collections::HashMap;
use std::io::prelude::*;
use std::path::Path;
use toml::Table;
use tracing::{info, trace, warn};
use winit::keyboard::ModifiersState;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Modifiers {
    Control,
    Alt,
    Shift,
    Space,
    Tab,
    Super,
}

impl Modifiers {
    pub fn from_str(input: &str) -> Option<Self> {
        match input {
            "cr" => Some(Self::Control),
            "control" => Some(Self::Control),
            "alt" => Some(Self::Alt),
            "shift" => Some(Self::Shift),
            "space" => Some(Self::Space),
            "tab" => Some(Self::Tab),
            "super" => Some(Self::Super),
            _ => None,
        }
    }
}


#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Command {
    key: String,
    mods: Option<ModifiersState>,
}

impl Command {
    pub fn new(key: &str, mods: &Option<ModifiersState>) -> Self {
        let key = key.to_owned();
        let mods = mods.to_owned();
        Self { key, mods }
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

    pub fn parse_str(input: &str) -> IResult<&str, Option<Self>> {
        let (rem, mods) = opt(Self::parse_mods)(input)?;
        let (rem, key) = Self::word(rem)?;
        if let Some(val) = mods {
            let command = Command::new(key, &val);
            Ok((rem, Some(command)))
        } else {
            let command = Command::new(key, &None);
            Ok((rem, Some(command)))
        }
    }

    pub fn word(input: &str) -> IResult<&str, &str> {
        let (rem, _) = space0(input)?;
        alphanumeric1(rem)
    }

    pub fn parse_mods(input: &str) -> IResult<&str, Option<ModifiersState>> {
        let (rem, bracketed) = delimited(tag("<"), alphanumeric1, tag(">"))(input)?;
        trace!("Bracketed: {}", bracketed);
        let bracketed = Self::into_mods(bracketed);
        trace!("Remaining: {}", rem);
        let (rem, _) = space0(rem)?;
        let (rem, _) = tag("+")(rem)?;
        let (rem, _) = space0(rem)?;
        Ok((rem, bracketed))
    }

    pub fn from_str(value: &str) -> Option<Command> {
        if value.len() > 1 {
            warn!("Modifiers not supported.");
            None
        } else {
            match value {
                "" => {
                    warn!("Empty command detected.");
                    None
                },
                key => {
                    Some(Self {
                        key: key.to_owned(),
                        mods: None,
                    })
                },
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Act {
    Right,
    Left,
    Up,
    Down,
    Next,
    Previous,
}

impl Act {
    pub fn from_str(key: &str) -> Option<Self> {
        match key {
            "right" => Some(Self::Right),
            "left" => Some(Self::Left),
            "up" => Some(Self::Up),
            "down" => Some(Self::Down),
            "next" => Some(Self::Next),
            "previous" => Some(Self::Previous),
            _ => None
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum CommandOptions {
    Commands(Vec<Command>),
    Acts(Vec<Act>),
}

impl CommandOptions {
    pub fn with_act(&mut self, act: Act) {
        match self {
            Self::Commands(_) => warn!("Not an Acts variant!"),
            Self::Acts(acts) => acts.push(act),
        }
    }

    pub fn with_command(&mut self, command: Command) {
        match self {
            Self::Commands(commands) => commands.push(command),
            Self::Acts(_) => warn!("Not a Commands variant!"),
        }
    }
}

impl From<Act> for CommandOptions {
    fn from(act: Act) -> Self {
        let mut acts = Vec::new();
        acts.push(act);
        Self::Acts(acts)
    }
}

impl From<Vec<Act>> for CommandOptions {
    fn from(acts: Vec<Act>) -> Self {
        Self::Acts(acts)
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

    pub fn with_config() -> Polite<Self> {
        let mut choices = HashMap::new();
        let config = include_bytes!("../../config.toml");
        trace!("Config read: {} u8.", config.len());
        let stringly = String::from_utf8_lossy(config);
        let config = stringly.parse::<Table>().unwrap();
        trace!("Config read: {}", config);
        // let command_queue = vec!["right", "left", "up", "down"];
        let commands = &config["commands"];
        match commands {
            toml::Value::Table(tab) => {
                let command_queue = tab.keys().map(|k| k.clone()).collect::<Vec<String>>();
                for key in command_queue {
                    info!("Reading {}", &key);
                    if let toml::Value::String(s) = &commands[&key] {
                        let s = s.to_owned();
                        let command = match Command::parse_str(&s) {
                            Ok((_, c)) => Ok(c),
                            Err(_) => Err(FauxPas::Unknown),
                            // Err(e) => Err(FauxPas::from(e)),
                        };
                        let command = command?;
                        let act = Act::from_str(&key);
                        if let Some(a) = act {
                            let mut vec = Vec::new();
                            vec.push(a);
                            let opts = CommandOptions::Acts(vec);
                            if let Some(c) = command {
                                choices.insert(c, opts);
                            }
                        }
                    }
                }
            },
            _ => info!("Commands not a toml table."),
        }
        trace!("Choices: {:#?}", choices);
        Ok(Self(choices))
    }

    pub fn value(&self) -> &HashMap<Command, CommandOptions> {
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
