use crate::{
    rpg::players::tab::TabView,
    table::{Columnar, Filtration, TableView, Tabular},
};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::{EnumIter, IntoEnumIterator};

/// The `Character` struct holds data related to a player character.
#[derive(
    Debug,
    Clone,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
    derive_getters::Getters,
    derive_setters::Setters,
)]
#[setters(prefix = "with_", borrow_self)]
pub struct Character {
    /// Physical attributes defining the capabilities of the character.
    attributes: Attributes,
    /// Id Source for attribute table.
    attribute_id: String,
    /// Biographical data about a character.
    biography: Biography,
    /// Id Source for biography table.
    biography_id: String,
    /// Level of encumbrance the character is currently operating under.
    encumbrance: Encumbrance,
    /// Id Source for encumbrance table.
    encumbrance_id: String,
    /// Unique identifier assigned to the hosting tab.
    #[setters(strip_option)]
    identifier: Option<String>,
    /// Basic statistics used for combat and movement calculations.
    stats: Stats,
    /// Id Source for basic stats table.
    stats_id: String,
}

impl Character {
    /// The `new` method creates an instance of `Character` from the provided `attributes`.
    /// Derives [`Stats`] from [`Attributes`].
    /// Derives [`Encumbrance`] from [`Stats`].
    pub fn new(attributes: Attributes) -> Self {
        let stats = Stats::from(&attributes);
        let encumbrance = Encumbrance::from(&stats);
        let biography = Default::default();
        let mut id = crate::identifier::Identifier::default();
        let identifier = None;
        Self {
            attributes,
            attribute_id: id.name(),
            biography,
            biography_id: id.name(),
            encumbrance,
            encumbrance_id: id.name(),
            identifier,
            stats,
            stats_id: id.name(),
        }
    }

    /// The `view` method displays the data in `Character` within an [`egui::Ui`].
    pub fn view(&self, ui: &mut egui::Ui, name: &str) {
        ui.label(format!("Character Name: {}", self.biography.name()));
        ui.label(format!("Player Name: {}", self.biography.player()));
        self.attributes.view(ui, name, &self.attribute_id);
        self.stats.view(ui, name, &self.stats_id);
        self.encumbrance.view(ui, name, &self.encumbrance_id);
    }

    pub fn name(&self) -> &String {
        self.biography.name()
    }

    /// Update the name of the character to `name`.
    /// Wraps [`Biography::with_name`].
    pub fn with_name(mut self, name: &str) -> Self {
        self.biography = self.biography.with_name(name.to_string());
        self
    }

    /// Update the player name associated with the character to `player`.
    /// Wraps [`Biography::with_player`].
    pub fn with_player(mut self, player: &str) -> Self {
        self.biography = self.biography.with_player(player.to_string());
        self
    }
}

/// The `Biography` struct holds personal details about the character that do not have a
/// corresponding game mechanic, but contain information relevant to the player.
/// Uses [`derive_getters::Getters`] to return values from private fields in the struct.
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    derive_getters::Getters,
    derive_setters::Setters,
    derive_builder::Builder,
)]
#[setters(prefix = "with_")]
#[builder(setter(into), default)]
pub struct Biography {
    /// The character name.
    name: String,
    /// The player name.
    player: String,
}

/// The `AttributeType` enum classifies attributes into variants.
/// Corresponds to field values in the [`Attributes`] struct.
/// We use [`AttributeType::Strength`] as an aribtrary default variant in case the user wants to
/// create the type first and set the value at a later time.
/// Uses [`derive_more::Display`] to implement the [`Display`] trait.
/// Uses [`strum_macros::EnumIter`] to implement the [`Iterator`] trait.
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
    Serialize,
    Deserialize,
)]
pub enum AttributeType {
    /// Physical strength.
    #[default]
    Strength,
    /// Speed and flexibility.
    Dexterity,
    /// Capacity of thought.
    Intelligence,
    /// Integrity of constitution.
    Health,
    /// Integer representation of life essence.
    HitPoints,
    /// Capacity for focus and determination.
    Willpower,
    /// Ability to sense and interpret external events.
    Perception,
    /// Integer representation of remaining stamina.
    Fatigue,
}

impl fmt::Display for AttributeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match *self {
            Self::Strength => "Strength",
            Self::Dexterity => "Dexterity",
            Self::Intelligence => "Intelligence",
            Self::Health => "Health",
            Self::HitPoints => "Hit Points",
            Self::Willpower => "Willpower",
            Self::Perception => "Perception",
            Self::Fatigue => "Fatigue",
        };
        write!(f, "{}", value)
    }
}

/// The `Attributes` struct holds the attribute values for a [`Character`].
/// The fields of `Attributes` correspond to the variants of [`AttributeType`].
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
    Serialize,
    Deserialize,
    derive_getters::Getters,
    derive_setters::Setters,
    derive_builder::Builder,
)]
#[setters(prefix = "with_")]
#[builder(setter(into), default)]
pub struct Attributes {
    /// Corresponds to [`AttributeType::Strength`].
    st: usize,
    /// Corresponds to [`AttributeType::Dexterity`].
    dx: usize,
    /// Corresponds to [`AttributeType::Intelligence`].
    iq: usize,
    /// Corresponds to [`AttributeType::Health`].
    ht: usize,
    /// Corresponds to [`AttributeType::HitPoints`].
    hp: usize,
    /// Corresponds to [`AttributeType::Willpower`].
    will: usize,
    /// Corresponds to [`AttributeType::Perception`].
    per: usize,
    /// Corresponds to [`AttributeType::Fatigue`].
    fp: usize,
}

impl Attributes {
    /// The `from_vec` is a convenience function for creating a new instance of `Attributes`.
    /// The order of values in `vec` corresponds to the field order in `Attributes`.
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
        attribute.to_string()
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
    /// Called by [`Self::columns`].
    pub fn column(&self, attribute: &AttributeType, column: &DisplayColumns) -> String {
        match *column {
            DisplayColumns::Name => self.name(attribute),
            DisplayColumns::Value => self.value(attribute).to_string(),
        }
    }

    /// The `columns` method creates a table row corresponding to given attribute.
    /// Contains a column for each variant of [`DisplayColumns`].
    /// Iterates on [`DisplayColumns`] and calls [`Self::column`] using the given `attribute`.
    /// Called by [`ColumnIterator::next`] and in turn by [`Self::iter_columns`].
    pub fn columns(&self, attribute: &AttributeType) -> Vec<String> {
        DisplayColumns::iter()
            .map(|c| self.column(attribute, &c))
            .collect::<Vec<String>>()
    }

    pub fn iter_columns(&self) -> ColumnIterator {
        ColumnIterator::from(self)
    }

    /// Passing a `table_id` is necessary to ensure that multiple tables can inhabit the name tab.
    pub fn view(&self, ui: &mut egui::Ui, name: &str, table_id: &str) {
        ui.label("Attributes");
        let mut tab = TabView::new(TableView::new(*self), name);
        ui.push_id(table_id, |ui| {
            tab.view_mut().table(ui);
        });
    }
}

/// We implement the [`Tabular`] trait on [`DisplayField`] for `Attributes` because
impl Tabular<DisplayField> for Attributes {
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

impl Filtration<Attributes, String> for Attributes {
    fn filter(self, filter: &String) -> Self {
        self
    }
}

/// Describes a single attribute for display in a table.
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    derive_getters::Getters,
    derive_setters::Setters,
)]
#[setters(prefix = "with_")]
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

impl Columnar for DisplayField {
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

/// The `ColumnIterator` struct implements the [`Iterator`] trait over [`Attributes`], returning
/// the [`DisplayField`] associated with each attribute.
/// Uses the [`AttributeTypeIter`] implementation to drive the iterator under the hood by stepping
/// through the variants of [`AttributeType`], calling [`Attributes::columns`] on the attribute
/// type, and creating a [`DisplayField`] from the results.
#[derive(Debug, Clone)]
pub struct ColumnIterator {
    /// The `values` field hold the data over which the iterator will step to derive the resulting
    /// item, the [`DisplayField`].
    values: Attributes,
    /// The `kind` field holds an [`AttributeTypeIter`], which iterates over [`AttributeType`].
    /// We call next on this iterator to drive state in our own implementation of [`Iterator::next`].
    kind: AttributeTypeIter,
}

/// We implement the [`From`] trait on [`Attributes`] for `ColumnIterator` as the preferred method
/// of obtaining a new instance.
impl From<&Attributes> for ColumnIterator {
    /// [`Attributes`] are [`Copy`], so we can dereference it to obtain `values`.
    /// We create an [`AttributeTypeIter`] for the `kind` field by calling [`AttributeType::iter`].
    fn from(value: &Attributes) -> Self {
        Self {
            values: *value,
            kind: AttributeType::iter(),
        }
    }
}

/// We want to access the fields of [`Attributes`] one at a time, making sure to visit each field.
/// From each field, we want to derive the [`DisplayField`], for use in generating a table row.
impl Iterator for ColumnIterator {
    /// The type for `Item` is [`DisplayField`], which is formatted for generating a table row.
    type Item = DisplayField;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(x) = self.kind.next() {
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

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
    derive_new::new,
    derive_getters::Getters,
    derive_setters::Setters,
    derive_builder::Builder,
)]
#[setters(prefix = "with_")]
#[builder(setter(into), default)]
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

    pub fn view(&self, ui: &mut egui::Ui, name: &str, table_id: &str) {
        ui.label("Basic Stats");
        let mut tab = TabView::new(TableView::new(self.clone()), name);
        ui.push_id(table_id, |ui| {
            tab.view_mut().table(ui);
        });
    }
}

impl Tabular<DisplayField> for Stats {
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

impl Filtration<Stats, String> for Stats {
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

#[derive(Debug, Clone)]
pub struct StatColIter {
    values: Stats,
    kind: StatTypeIter,
}

impl From<&Stats> for StatColIter {
    fn from(value: &Stats) -> Self {
        Self {
            values: value.clone(),
            kind: StatType::iter(),
        }
    }
}

impl Iterator for StatColIter {
    type Item = DisplayField;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(x) = self.kind.next() {
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
    Copy,
    Clone,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    derive_new::new,
    derive_getters::Getters,
    derive_setters::Setters,
    derive_builder::Builder,
)]
#[setters(prefix = "with_")]
#[builder(setter(into), default)]
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
    derive_new::new,
    smart_default::SmartDefault,
)]
pub enum DamageKind {
    #[default]
    Thrust(usize),
    Swing(usize),
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
    derive_new::new,
)]
pub enum EncumbranceType {
    #[default]
    Weight,
    Move,
    Dodge,
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    derive_getters::Getters,
    derive_setters::Setters,
)]
#[setters(prefix = "with_")]
pub struct EncumbranceField {
    name: String,
    weight: String,
    enc_move: String,
    dodge: String,
    id: uuid::Uuid,
}

impl EncumbranceField {
    pub fn new(name: &str, weight: &str, enc_move: &str, dodge: &str) -> Self {
        Self {
            name: name.to_string(),
            weight: weight.to_string(),
            enc_move: enc_move.to_string(),
            dodge: dodge.to_string(),
            id: uuid::Uuid::new_v4(),
        }
    }
}

impl Columnar for EncumbranceField {
    fn names() -> Vec<String> {
        let mut values = EncumbranceType::iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>();
        values.insert(0, "Level".to_string());
        values
    }

    fn values(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            self.weight.clone(),
            self.enc_move.clone(),
            self.dodge.clone(),
        ]
    }

    fn id(&self) -> &uuid::Uuid {
        &self.id
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
    Serialize,
    Deserialize,
    derive_new::new,
    derive_getters::Getters,
    derive_setters::Setters,
)]
#[setters(prefix = "with_")]
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

    pub fn view(&self, ui: &mut egui::Ui, name: &str, table_id: &str) {
        ui.label("Encumbrance");
        let mut tab = TabView::new(TableView::new(*self), name);
        ui.push_id(table_id, |ui| {
            tab.view_mut().table(ui);
        });
    }
}

impl Tabular<EncumbranceField> for Encumbrance {
    fn headers() -> Vec<String> {
        EncumbranceField::names()
    }

    fn rows(&self) -> Vec<EncumbranceField> {
        self.iter_columns().collect::<Vec<EncumbranceField>>()
    }

    fn sort_by_col(&mut self, column_index: usize, reverse: bool) {}
}

impl Filtration<Encumbrance, String> for Encumbrance {
    fn filter(self, filter: &String) -> Self {
        self
    }
}

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
            let item = EncumbranceField::new(&columns[0], &columns[1], &columns[2], &columns[3]);
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
    Serialize,
    Deserialize,
    derive_getters::Getters,
)]
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
    Serialize,
    Deserialize,
    derive_getters::Getters,
)]
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

/// The `EncumbranceDodge` struct holds the dodge value for a [`Character`] at different levels of
/// encumbrance.
/// Fields in `EncumbranceDodge` correspond to the variants of [`EncumbranceLevel`].
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
    Serialize,
    Deserialize,
    derive_getters::Getters,
)]
pub struct EncumbranceDodge {
    /// Corresponds to [`EncumbranceLevel::None`].
    none: usize,
    /// Corresponds to [`EncumbranceLevel::Light`].
    light: usize,
    /// Corresponds to [`EncumbranceLevel::Medium`].
    medium: usize,
    /// Corresponds to [`EncumbranceLevel::Heavy`].
    heavy: usize,
    /// Corresponds to [`EncumbranceLevel::XHeavy`].
    extra_heavy: usize,
}

impl EncumbranceDodge {
    /// The `value` method returns the value of the field correpsonding to the [`EncumbranceLevel`]
    /// provided in the `level` argument.
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

/// # Safety
/// Since basic speed has a minimum of one, at extra heavy usize will not drop below zero.
impl From<&Stats> for EncumbranceDodge {
    fn from(stats: &Stats) -> Self {
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
            Self::None => "None".to_string(),
            Self::Light => "Light".to_string(),
            Self::Medium => "Medium".to_string(),
            Self::Heavy => "Heavy".to_string(),
            Self::XHeavy => "Extra Heavy".to_string(),
        }
    }
}
