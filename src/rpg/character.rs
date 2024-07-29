use crate::table;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Character {
    attributes: Attributes,
    encumbrance: Encumbrance,
    stats: Stats,
}

impl Character {
    pub fn new(attributes: Attributes) -> Self {
        let stats = Stats::from(&attributes);
        let encumbrance = Encumbrance::from(&stats);
        Self {
            attributes,
            stats,
            encumbrance,
        }
    }

    pub fn attributes(&self) -> &Attributes {
        &self.attributes
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
    Display,
    EnumIter,
    Serialize,
    Deserialize,
)]
pub enum AttributeType {
    #[default]
    Strength,
    Dexterity,
    Intelligence,
    Health,
    HitPoints,
    Willpower,
    Perception,
    Fatigue,
}

#[derive(
    Debug, Default, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize,
)]
pub struct Attributes {
    st: usize,
    dx: usize,
    iq: usize,
    ht: usize,
    hp: usize,
    will: usize,
    per: usize,
    fp: usize,
}

impl Attributes {
    pub fn from_vec(vec: Vec<usize>) -> Self {
        Self {
            st: vec[0],
            dx: vec[1],
            iq: vec[2],
            ht: vec[3],
            hp: vec[4],
            will: vec[5],
            per: vec[6],
            fp: vec[7],
        }
    }

    pub fn name(&self, attribute: &AttributeType) -> String {
        match *attribute {
            AttributeType::Strength => "Strength".to_string(),
            AttributeType::Dexterity => "Dexterity".to_string(),
            AttributeType::Intelligence => "Intelligence".to_string(),
            AttributeType::Health => "Health".to_string(),
            AttributeType::HitPoints => "Hit Points".to_string(),
            AttributeType::Willpower => "Willpower".to_string(),
            AttributeType::Perception => "Perception".to_string(),
            AttributeType::Fatigue => "Fatigue".to_string(),
        }
    }

    pub fn value(&self, attribute: &AttributeType) -> usize {
        match *attribute {
            AttributeType::Strength => self.st,
            AttributeType::Dexterity => self.dx,
            AttributeType::Intelligence => self.iq,
            AttributeType::Health => self.ht,
            AttributeType::HitPoints => self.hp,
            AttributeType::Willpower => self.will,
            AttributeType::Perception => self.per,
            AttributeType::Fatigue => self.fp,
        }
    }

    /// Generates the display value for a given column.
    pub fn column(&self, attribute: &AttributeType, column: &DisplayColumns) -> String {
        match *column {
            DisplayColumns::Name => self.name(attribute),
            DisplayColumns::Value => self.value(attribute).to_string(),
        }
    }

    pub fn columns(&self, attribute: &AttributeType) -> Vec<String> {
        DisplayColumns::iter()
            .map(|c| self.column(attribute, &c))
            .collect::<Vec<String>>()
    }

    pub fn iter_columns(&self) -> ColumnIterator {
        ColumnIterator::from(self)
    }
}

impl table::Tabular<DisplayField> for Attributes {
    fn headers() -> Vec<String> {
        DisplayColumns::iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
    }

    fn rows(&self) -> Vec<DisplayField> {
        self.iter_columns().collect::<Vec<DisplayField>>()
    }

    fn sort_by_col(&mut self, column_index: usize, reverse: bool) {}
}

impl table::Filtration<Attributes, String> for Attributes {
    fn filter(self, filter: &String) -> Self {
        self
    }
}

/// Describes a single attribute for display in a table.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct DisplayField {
    name: String,
    value: String,
    id: uuid::Uuid,
}

impl DisplayField {
    /// Creates a new struct from the provided parts.
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
            id: uuid::Uuid::new_v4(),
        }
    }
}

impl table::Columnar for DisplayField {
    fn names() -> Vec<String> {
        DisplayColumns::iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
    }

    fn values(&self) -> Vec<String> {
        vec![self.name.clone(), self.value.clone()]
    }

    fn id(&self) -> &uuid::Uuid {
        &self.id
    }
}

pub struct ColumnIterator {
    values: Attributes,
    type_of: AttributeTypeIter,
}

impl From<&Attributes> for ColumnIterator {
    fn from(value: &Attributes) -> Self {
        Self {
            values: *value,
            type_of: AttributeType::iter(),
        }
    }
}

impl Iterator for ColumnIterator {
    type Item = DisplayField;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(x) = self.type_of.next() {
            let columns = self.values.columns(&x);
            let item = DisplayField::new(&columns[0], &columns[1]);
            Some(item)
        } else {
            None
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
    Display,
    EnumIter,
    Serialize,
    Deserialize,
)]
pub enum DisplayColumns {
    Name,
    #[default]
    Value,
}

impl DisplayColumns {
    pub fn names() -> Vec<String> {
        let mut values = Vec::new();
        for column in Self::iter() {
            values.push(format!("{column}"));
        }
        values
    }
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Stats {
    /// The maximum weight you can lift over your head with one hand one second.
    /// (ST * ST)/5
    /// BS-15
    basic_lift: usize,
    basic_move: usize,
    /// Basic speed is (HT + DX)/4
    /// Dodge is basic speed plus 3, dropping fractions [BS - 17]
    basic_speed: f64,
}

impl Stats {
    pub fn name(&self, stat: &StatType) -> String {
        match *stat {
            StatType::BasicLift => "Basic Lift".to_string(),
            StatType::BasicMove => "Basic Move".to_string(),
            StatType::BasicSpeed => "Basic Speed".to_string(),
        }
    }

    pub fn value(&self, stat: &StatType) -> String {
        match *stat {
            StatType::BasicLift => self.basic_lift.to_string(),
            StatType::BasicMove => self.basic_move.to_string(),
            StatType::BasicSpeed => {
                let speed = self.basic_speed * 100.0;
                let speed = speed.round() / 100.0;
                speed.to_string()
            }
        }
    }

    /// Generates the display value for a given column.
    pub fn column(&self, stat: &StatType, column: &DisplayColumns) -> String {
        match *column {
            DisplayColumns::Name => self.name(stat),
            DisplayColumns::Value => self.value(stat).to_string(),
        }
    }

    pub fn columns(&self, stat: &StatType) -> Vec<String> {
        DisplayColumns::iter()
            .map(|c| self.column(stat, &c))
            .collect::<Vec<String>>()
    }

    pub fn iter_columns(&self) -> StatColIter {
        StatColIter::from(self)
    }
}

impl table::Tabular<DisplayField> for Stats {
    fn headers() -> Vec<String> {
        DisplayColumns::iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
    }

    fn rows(&self) -> Vec<DisplayField> {
        self.iter_columns().collect::<Vec<DisplayField>>()
    }

    fn sort_by_col(&mut self, column_index: usize, reverse: bool) {}
}

impl table::Filtration<Stats, String> for Stats {
    fn filter(self, filter: &String) -> Self {
        self
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
    Display,
    EnumIter,
    Serialize,
    Deserialize,
)]
pub enum StatType {
    #[default]
    BasicLift,
    BasicMove,
    BasicSpeed,
}

impl From<&Attributes> for Stats {
    fn from(attr: &Attributes) -> Self {
        let st = attr.st as f64;
        let basic_lift = (st * st) / 5.0;
        let basic_lift = basic_lift.floor() as usize;
        let ht = attr.ht as f64;
        let dx = attr.dx as f64;
        let basic_speed = (ht + dx) / 4.0;
        let basic_move = basic_speed.floor() as usize;
        Self {
            basic_lift,
            basic_speed,
            basic_move,
        }
    }
}

pub struct StatColIter {
    values: Stats,
    type_of: StatTypeIter,
}

impl From<&Stats> for StatColIter {
    fn from(value: &Stats) -> Self {
        Self {
            values: value.clone(),
            type_of: StatType::iter(),
        }
    }
}

impl Iterator for StatColIter {
    type Item = DisplayField;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(x) = self.type_of.next() {
            let columns = self.values.columns(&x);
            let item = DisplayField::new(&columns[0], &columns[1]);
            Some(item)
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct CombatStats {
    damage_thrust: DamageKind,
    damage_swing: DamageKind,
    dr: usize,
    parry: usize,
    block: usize,
}

impl Default for CombatStats {
    fn default() -> Self {
        Self {
            damage_thrust: DamageKind::Thrust(0),
            damage_swing: DamageKind::Swing(0),
            dr: 0,
            parry: 0,
            block: 0,
        }
    }
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Display,
    EnumIter,
    Serialize,
    Deserialize,
)]
pub enum DamageKind {
    Thrust(usize),
    Swing(usize),
}

impl Default for DamageKind {
    fn default() -> Self {
        Self::Swing(Default::default())
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
    Display,
    EnumIter,
    Serialize,
    Deserialize,
)]
pub enum EncumbranceType {
    #[default]
    Weight,
    Move,
    Dodge,
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct EncumbranceField {
    weight: String,
    enc_move: String,
    dodge: String,
    id: uuid::Uuid,
}

impl EncumbranceField {
    pub fn new(weight: &str, enc_move: &str, dodge: &str) -> Self {
        Self {
            weight: weight.to_string(),
            enc_move: enc_move.to_string(),
            dodge: dodge.to_string(),
            id: uuid::Uuid::new_v4(),
        }
    }
}

impl table::Columnar for EncumbranceField {
    fn names() -> Vec<String> {
        EncumbranceType::iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
    }

    fn values(&self) -> Vec<String> {
        vec![
            self.weight.clone(),
            self.enc_move.clone(),
            self.dodge.clone(),
        ]
    }

    fn id(&self) -> &uuid::Uuid {
        &self.id
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Encumbrance {
    weight: EncumbranceWeight,
    enc_move: EncumbranceMove,
    dodge: EncumbranceDodge,
}

impl Encumbrance {
    pub fn value(&self, type_of: &EncumbranceType, level: &EncumbranceLevel) -> usize {
        match *type_of {
            EncumbranceType::Weight => self.weight.value(level),
            EncumbranceType::Move => self.enc_move.value(level),
            EncumbranceType::Dodge => self.dodge.value(level),
        }
    }

    pub fn columns(&self, level: &EncumbranceLevel) -> Vec<String> {
        let mut values = Vec::new();
        values.push(level.name());
        let enc = EncumbranceType::iter()
            .map(|v| self.value(&v, level).to_string())
            .collect::<Vec<String>>();
        values.extend(enc);

        values
    }

    pub fn iter_columns(&self) -> EncumbranceIter {
        EncumbranceIter::from(self)
    }
}

// impl table::Tabular<EncumbranceField> for Encumbrance {
//     fn headers() -> Vec<String> {
//         EncumbranceType::iter().map(|v| v.to_string()).collect::<Vec<String>>()
//     }
//
//     fn rows(&self) -> Vec<EncumbranceField> {
//
//     }
// }

// impl Tabular for Encumbrance
// Create a character sheet that holds multiple table views in a single window

impl From<&Stats> for Encumbrance {
    fn from(stats: &Stats) -> Self {
        let weight = EncumbranceWeight::from(stats);
        let enc_move = EncumbranceMove::from(stats);
        let dodge = EncumbranceDodge::from(stats);
        Self {
            weight,
            enc_move,
            dodge,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EncumbranceIter {
    values: Encumbrance,
    type_of: EncumbranceLevelIter,
}

impl Iterator for EncumbranceIter {
    type Item = EncumbranceField;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(x) = self.type_of.next() {
            let columns = self.values.columns(&x);
            let item = EncumbranceField::new(&columns[0], &columns[1], &columns[2]);
            Some(item)
        } else {
            None
        }
    }
}

impl From<&Encumbrance> for EncumbranceIter {
    fn from(value: &Encumbrance) -> Self {
        Self {
            values: *value,
            type_of: EncumbranceLevel::iter(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct EncumbranceWeight {
    none: usize,
    light: usize,
    medium: usize,
    heavy: usize,
    extra_heavy: usize,
}

impl EncumbranceWeight {
    pub fn value(&self, level: &EncumbranceLevel) -> usize {
        match *level {
            EncumbranceLevel::None => self.none,
            EncumbranceLevel::Light => self.light,
            EncumbranceLevel::Medium => self.medium,
            EncumbranceLevel::Heavy => self.heavy,
            EncumbranceLevel::XHeavy => self.extra_heavy,
        }
    }
}

impl From<&Stats> for EncumbranceWeight {
    fn from(stats: &Stats) -> Self {
        let basic_lift = stats.basic_lift;
        let none = basic_lift;
        let light = basic_lift * 2;
        let medium = basic_lift * 3;
        let heavy = basic_lift * 6;
        let extra_heavy = basic_lift * 10;
        Self {
            none,
            light,
            medium,
            heavy,
            extra_heavy,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct EncumbranceMove {
    none: usize,
    light: usize,
    medium: usize,
    heavy: usize,
    extra_heavy: usize,
}

impl EncumbranceMove {
    pub fn value(&self, level: &EncumbranceLevel) -> usize {
        match *level {
            EncumbranceLevel::None => self.none,
            EncumbranceLevel::Light => self.light,
            EncumbranceLevel::Medium => self.medium,
            EncumbranceLevel::Heavy => self.heavy,
            EncumbranceLevel::XHeavy => self.extra_heavy,
        }
    }
}

impl From<&Stats> for EncumbranceMove {
    fn from(stats: &Stats) -> Self {
        let basic_move = stats.basic_move;
        let none = basic_move;
        let flt = basic_move as f64 * 0.8;
        let light = flt.floor() as usize;
        let flt = basic_move as f64 * 0.6;
        let medium = flt.floor() as usize;
        let flt = basic_move as f64 * 0.4;
        let heavy = flt.floor() as usize;
        let flt = basic_move as f64 * 0.2;
        let extra_heavy = flt.floor() as usize;
        Self {
            none,
            light,
            medium,
            heavy,
            extra_heavy,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct EncumbranceDodge {
    none: usize,
    light: usize,
    medium: usize,
    heavy: usize,
    extra_heavy: usize,
}

impl EncumbranceDodge {
    pub fn value(&self, level: &EncumbranceLevel) -> usize {
        match *level {
            EncumbranceLevel::None => self.none,
            EncumbranceLevel::Light => self.light,
            EncumbranceLevel::Medium => self.medium,
            EncumbranceLevel::Heavy => self.heavy,
            EncumbranceLevel::XHeavy => self.extra_heavy,
        }
    }
}

impl From<&Stats> for EncumbranceDodge {
    fn from(stats: &Stats) -> Self {
        // Since basic speed has a minimum of one, at extra heavy usize will not drop below zero.
        // Dodge is basic speed plus 3, dropping fractions [BS - 17]
        let dodge = stats.basic_speed.floor() as usize + 3;
        let none = dodge;
        let light = dodge - 1;
        let medium = dodge - 2;
        let heavy = dodge - 3;
        let extra_heavy = dodge - 4;
        Self {
            none,
            light,
            medium,
            heavy,
            extra_heavy,
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
    Display,
    EnumIter,
    Serialize,
    Deserialize,
)]
pub enum EncumbranceLevel {
    #[default]
    None,
    Light,
    Medium,
    Heavy,
    XHeavy,
}

impl EncumbranceLevel {
    pub fn name(&self) -> String {
        match *self {
            EncumbranceLevel::None => "None".to_string(),
            EncumbranceLevel::Light => "Light".to_string(),
            EncumbranceLevel::Medium => "Medium".to_string(),
            EncumbranceLevel::Heavy => "Heavy".to_string(),
            EncumbranceLevel::XHeavy => "Extra Heavy".to_string(),
        }
    }
}
