use crate::prelude::{
    Address, Addresses, Choices, EguiAct, Leaf, Node, Parcels, TableView, Tabular, Tree,
};
use egui::{
    Align, Color32, Context, DragValue, Id, Layout, ScrollArea, Sense, Slider, TextStyle, Ui,
};
use egui_extras::{Column, TableBuilder};
use itertools::sorted;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone, Debug, Default)]
pub struct UiState {
    pub addresses: Option<Addresses>,
    pub address_table: Option<TableView<Addresses, Address>>,
    pub counter: i32,
    pub focus_tree: Tree,
    pub focus_counter: bool,
    pub focus_parcels: bool,
    pub panel: Option<Panel<Address>>,
    pub parcels: Option<Arc<Parcels>>,
}

impl UiState {
    pub fn new() -> Self {
        // let vec = include_bytes!("../data/addresses.data");
        // let addresses: Option<AddressPoints> = match bincode::deserialize(&vec[..]) {
        //     Ok(data) => Some(data),
        //     Err(e) => {
        //         tracing::info!("{:#?}", e.to_string());
        //         None
        //     }
        // };

        let mut panel = None;
        let mut address_table = None;
        let addresses = match Addresses::load("data/addresses.data") {
            Ok(data) => {
                panel = Some(Panel::new(data.records.clone()));
                address_table = Some(TableView::new(data.clone()));
                tracing::info!("Records read: {}", data.records.len());
                Some(data)
            }
            Err(e) => {
                tracing::info!("Could not read records: {}", e.to_string());
                None
            }
        };
        // let addresses = match Addresses::from_csv("data/addresses.csv") {
        //     Ok(data) => {
        //         panel = Some(Panel::new(data.records.clone()));
        //         // tracing::info!("Records read: {}", data.records.len());
        //         // let mut d = data.clone();
        //         // d.to_csv("data/addresses.csv").unwrap();
        //         // data.save("data/addresses.data").unwrap();
        //         Some(data)
        //     },
        //     Err(_) => None,
        // };

        let parcels = match Parcels::load("data/parcels.data") {
            Ok(data) => Some(Arc::new(data)),
            Err(_) => None,
        };

        Self {
            addresses,
            address_table,
            counter: Default::default(),
            focus_tree: Tree::new(),
            focus_counter: true,
            focus_parcels: true,
            panel,
            parcels,
        }
    }

    pub fn in_focus(&mut self, id: Id) -> bool {
        if let Some(focus) = self.focus_tree.select {
            if focus == id {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn act(&mut self, act: &EguiAct) {
        match *act {
            EguiAct::Right => {
                let _ = self.focus_tree.next_node();
                self.focus_tree.select_current();
            }
            EguiAct::Left => {
                let _ = self.focus_tree.previous_node();
                self.focus_tree.select_current();
            }
            EguiAct::Up => self.focus_tree.select_previous(),
            EguiAct::Down => self.focus_tree.select_next(),
            EguiAct::Next => self.focus_tree.select_next_node(),
            EguiAct::Previous => self.focus_tree.select_previous_node(),
            EguiAct::NextWindow => self.focus_tree.select_next_window(),
            EguiAct::PreviousWindow => self.focus_tree.select_previous_window(),
            EguiAct::Be => tracing::info!("Taking no action."),
        }
    }

    pub fn run(&mut self, ui: &Context) {
        let mut set_counter = None;
        let mut set_counter1 = None;
        let mut set_parcels = None;
        if self.focus_tree.flags.is_empty() {
            set_counter = Some(self.focus_tree.window());
            set_counter1 = Some(self.focus_tree.window());
            set_parcels = Some(self.focus_tree.window());
        }
        egui::Window::new("Whimsy UI").show(ui, |ui| {
            ui.heading("Window");

            let button = ui.button("Counter");
            if self.in_focus(button.id) {
                tracing::info!("Requesting focus for {:#?}", button.id);
                button.request_focus();
                tracing::info!("Clearing select.");
                self.focus_tree.select = None;
            }
            if button.clicked {
                self.counter += 1;
            }
            ui.label(format!("Counter ID: {:?}", button.id));
            // if button.has_focus() {
            //     ui.label("Focused: Click <Enter> to increment counter.");
            // }

            let inc = ui.button("Increment");
            if self.in_focus(inc.id) {
                tracing::info!("Requesting focus for {:#?}", inc.id);
                inc.request_focus();
                tracing::info!("Clearing select.");
                self.focus_tree.select = None;
            }
            if inc.has_focus() {
                ui.label("Focused: Click <Enter> to increment counter.");
            }
            ui.label(format!("{}", self.counter));

            let mut address_ct = 0;
            if let Some(data) = &self.addresses {
                address_ct = data.records.len();
            }
            ui.label(format!("Addresses: {}", address_ct));

            let mut parcel_ct = 0;
            if let Some(data) = &self.parcels {
                parcel_ct = data.records.len();
            }
            ui.label(format!("Parcels: {}", parcel_ct));

            if let Some(id) = set_counter {
                let button_id = self.focus_tree.leaf(button.id);
                let inc_id = self.focus_tree.leaf(inc.id);
                let node_id = self.focus_tree.node();
                self.focus_tree.with_leaf(node_id, button_id);
                self.focus_tree.with_leaf(node_id, inc_id);
                self.focus_tree.with_window(node_id, id);
                tracing::info!("Tree: {:#?}", self.focus_tree);
                if let Some(counter) = self.focus_tree.flags.get_mut(&id) {
                    *counter = true;
                    set_counter = None;
                }
            }
        });
        egui::Window::new("Counter").show(ui, |ui| {
            let button = ui.button("Counter");
            if self.in_focus(button.id) {
                tracing::info!("Requesting focus for {:#?}", button.id);
                button.request_focus();
                tracing::info!("Clearing select.");
                self.focus_tree.select = None;
            }
            if button.clicked {
                self.counter += 1;
            }
            ui.label(format!("Counter ID: {:?}", button.id));
            if button.has_focus() {
                ui.label("Focused: Click <Enter> to increment counter.");
            }

            let inc = ui.button("Increment");
            if self.in_focus(inc.id) {
                tracing::info!("Requesting focus for {:#?}", inc.id);
                inc.request_focus();
                tracing::info!("Clearing select.");
                self.focus_tree.select = None;
            }
            if ui.button("Increment").clicked() {
                self.counter += 1;
            }
            ui.label(format!("{}", self.counter));

            let mut address_ct = 0;
            if let Some(data) = &self.addresses {
                address_ct = data.records.len();
            }
            ui.label(format!("Addresses: {}", address_ct));

            let mut parcel_ct = 0;
            if let Some(data) = &self.parcels {
                parcel_ct = data.records.len();
            }
            ui.label(format!("Parcels: {}", parcel_ct));

            if let Some(id) = set_counter1 {
                let button_id = self.focus_tree.leaf(button.id);
                let inc_id = self.focus_tree.leaf(inc.id);
                let node_id = self.focus_tree.node();
                self.focus_tree.with_leaf(node_id, button_id);
                self.focus_tree.with_leaf(node_id, inc_id);
                self.focus_tree.with_window(node_id, id);
                tracing::info!("Tree: {:#?}", self.focus_tree);
                if let Some(counter) = self.focus_tree.flags.get_mut(&id) {
                    *counter = true;
                    set_counter1 = None;
                }
            }
        });

        let text_style = TextStyle::Body;
        // egui::SidePanel::right("Sidebar").show(ui, |ui| {
        //     ui.label("Address Info:");
        //     if let Some(data) = &self.addresses {
        //         let row_height = ui.text_style_height(&text_style);
        //         let num_rows = data.records.len();
        //         egui::ScrollArea::vertical().show_rows(
        //             ui,
        //             row_height,
        //             num_rows,
        //             |ui, row_range| {
        //             for row in row_range {
        //                 ui.label(format!("{:#?}", data.records[row]));
        //             }
        //         });
        //     } else {
        //         ui.label("None loaded.");
        //     }
        // });

        let mut table_id = None;
        if let Some(window) = set_parcels {
            let id = self.focus_tree.node();
            table_id = Some(id);
            self.focus_tree.with_window(id, window);
        }
        egui::Window::new("Parcels").show(ui, |ui| {
            let mut select = self.focus_tree.select.clone();
            if let Some(data) = &self.parcels {
                let row_height = ui.text_style_height(&text_style);
                let num_rows = data.records.len();
                egui::ScrollArea::vertical().show_rows(
                    ui,
                    row_height,
                    num_rows,
                    |ui, row_range| {
                        for row in row_range {
                            let record = &data.records[row].owner;
                            let name = if let Some(val) = &record.name {
                                val.clone()
                            } else {
                                "None".to_string()
                            };
                            let owner = ui.label(format!("Owner: {}", name));
                            if set_parcels.is_some() {
                                let leaf = self.focus_tree.leaf(owner.id);
                                if let Some(table) = table_id {
                                    self.focus_tree.with_leaf(table, leaf);
                                }
                            }
                            if let Some(id) = select {
                                if id == owner.id {
                                    tracing::info!("Requesting focus for {:#?}", owner.id);
                                    owner.request_focus();
                                    tracing::info!("Clearing select.");
                                    select = None;
                                }
                            }

                            ui.label(format!("Map #: {}", &record.id));
                        }
                        if let Some(id) = set_parcels {
                            tracing::info!("Tree: {:#?}", self.focus_tree);
                            if let Some(p) = self.focus_tree.flags.get_mut(&id) {
                                *p = true;
                                set_parcels = None;
                            }
                        }
                    },
                );
                self.focus_tree.select = select;
            } else {
                ui.label("None loaded.");
            }
        });

        // egui::Window::new("Addresses").show(ui, |ui| {
        //     if let Some(panel) = &mut self.panel {
        //         panel.table(ui);
        //     }
        //
        // });

        // egui::Window::new("Address Table").show(ui, |ui| {
        //     if let Some(values) = &mut self.address_table {
        //         values.table(ui);
        //     }
        //
        // });

        // egui::Window::new("Addresses").show(ui, |ui| {
        //     self.scroll_to.ui(ui);
        //
        // });
    }
}

#[derive(Clone, Debug, Default)]
pub struct HashPanel<K, V> {
    pub data: BTreeMap<K, V>,
    pub key: Option<K>,
    pub selected: HashSet<V>,
    pub search: String,
    pub target: usize,
    pub value: V,
}

impl<
        K: Eq + std::hash::Hash + Ord + Clone + std::fmt::Display + Default,
        V: std::fmt::Display + Clone + Default + Eq + std::hash::Hash,
    > HashPanel<K, V>
{
    pub fn new(data: BTreeMap<K, V>) -> Self {
        Self {
            data,
            ..Default::default()
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        let mut panel = self.clone();
        if !self.search.is_empty() {
            panel.contains(&self.search);
        }
        let keys: Vec<&K> = sorted(panel.data.keys().into_iter()).collect();
        let num_rows = keys.len();
        let mut track_item = false;
        let mut scroll_top = false;
        let mut scroll_bottom = false;
        ui.horizontal(|ui| {
            ui.add(egui::TextEdit::singleline(&mut self.search).hint_text("Search"));
            if ui.button("X").clicked() {
                self.search = Default::default();
            }
        });
        if num_rows == 0 {
            ui.label("Tracker disabled.");
        } else {
            ui.horizontal(|ui| {
                track_item |= ui
                    .add(Slider::new(&mut self.target, 0..=(num_rows - 1)))
                    .dragged();
                scroll_top |= ui.button("|<").clicked();
                scroll_bottom |= ui.button(">|").clicked();
            });
        }
        ui.separator();
        ScrollArea::vertical()
            .max_height(400.)
            .show(ui, |ui| {
                if scroll_top {
                    ui.scroll_to_cursor(Some(Align::TOP));
                }
                ui.vertical(|ui| {
                    if num_rows == 0 {
                        ui.label("No data to display.");
                    } else {
                        for item in 0..=(num_rows - 1) {
                            if track_item && item == self.target {
                                let response = ui.selectable_value(
                                    &mut self.value,
                                    self.data[keys[item]].clone(),
                                    format!("{}: {}", keys[item], self.data[keys[item]]),
                                );
                                response.scroll_to_me(Some(Align::Center));
                                self.value = self.data[keys[item]].clone();
                            } else {
                                ui.selectable_value(
                                    &mut self.value,
                                    self.data[keys[item]].clone(),
                                    format!("{}: {}", keys[item], self.data[keys[item]]),
                                );
                                // ui.label(format!("{}: {}", keys[item], self.data[keys[item]]));
                            }
                        }
                    }
                });

                if scroll_bottom {
                    ui.scroll_to_cursor(Some(Align::BOTTOM));
                }
            })
            .inner;

        ui.separator();
        ui.label(format!("Value selected: {}", self.value));
    }

    pub fn entry_contains(fragment: &str, entry: (&K, &mut V)) -> bool {
        let key_str = entry.0.to_string();
        let val_str = entry.1.to_string();
        if key_str.contains(fragment) | val_str.contains(fragment) {
            true
        } else {
            false
        }
    }

    pub fn contains(&mut self, fragment: &str) {
        self.data.retain(|k, v| {
            let key = k.to_string();
            let val = v.to_string();
            if key.contains(fragment) | val.contains(fragment) {
                true
            } else {
                false
            }
        });
    }

    pub fn table(&mut self, ui: &mut Ui) {
        let mut panel = self.clone();
        if !self.search.is_empty() {
            panel.contains(&self.search);
        }
        let num_rows = panel.data.len();
        let mut track_item = false;
        let mut scroll_top = false;
        let mut scroll_bottom = false;
        ui.horizontal(|ui| {
            ui.add(egui::TextEdit::singleline(&mut self.search).hint_text("Search"));
            if ui.button("X").clicked() {
                self.search = Default::default();
            }
        });
        if num_rows == 0 {
            ui.label("Tracker disabled.");
        } else {
            ui.horizontal(|ui| {
                track_item |= ui
                    .add(Slider::new(&mut self.target, 0..=(num_rows - 1)))
                    .dragged();
                scroll_top |= ui.button("|<").clicked();
                scroll_bottom |= ui.button(">|").clicked();
                if ui.button("Clear").clicked() {
                    self.selected = HashSet::new();
                }
            });
        }

        ui.separator();

        let data = panel.data.clone();
        let keys = data.keys().collect::<Vec<&K>>();
        let mut table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .sense(Sense::click())
            .cell_layout(Layout::left_to_right(Align::Center))
            .column(Column::auto().at_least(100.))
            .column(Column::auto().at_least(100.));
        if track_item {
            table = table.scroll_to_row(self.target, Some(Align::Center));
        }
        if scroll_top {
            table = table.scroll_to_row(0, Some(Align::BOTTOM));
        }
        if scroll_bottom {
            table = table.scroll_to_row(self.data.len(), Some(Align::BOTTOM));
        }
        table.body(|body| {
            body.rows(20., panel.data.len(), |mut row| {
                let row_index = row.index();
                row.set_selected(self.selected.contains(&panel.data[keys[row_index]]));
                row.col(|ui| {
                    ui.label(format!("{}", keys[row_index]));
                });
                row.col(|ui| {
                    ui.label(format!("{}", panel.data[keys[row_index]]));
                });
                self.toggle_row_selection(panel.data[keys[row_index]].clone(), &row.response());
            });
        });
    }

    pub fn toggle_row_selection(&mut self, target: V, row_response: &egui::Response) {
        if row_response.clicked() {
            if self.selected.contains(&target) {
                self.selected.remove(&target);
            } else {
                self.selected.insert(target);
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Panel<T> {
    pub data: HashMap<Uuid, T>,
    pub selected: HashSet<Uuid>,
    pub search: String,
    pub target: usize,
    pub value: Option<T>,
}

impl<T: PartialEq + Clone + std::fmt::Display + Card + Default> Panel<T> {
    pub fn new(data: Vec<T>) -> Self {
        let data = data
            .iter()
            .map(|v| {
                let k = Uuid::new_v4();
                (k, v.clone())
            })
            .collect::<HashMap<Uuid, T>>();
        Self {
            data,
            ..Default::default()
        }
    }

    pub fn table(&mut self, ui: &mut Ui) {
        let mut panel = self.clone();
        if !self.search.is_empty() {
            panel.contains(&self.search);
        }
        let num_rows = panel.data.len();
        let mut track_item = false;
        let mut scroll_top = false;
        let mut scroll_bottom = false;
        ui.horizontal(|ui| {
            ui.add(egui::TextEdit::singleline(&mut self.search).hint_text("Search"));
            if ui.button("X").clicked() {
                self.search = Default::default();
            }
        });
        if num_rows == 0 {
            ui.label("Tracker disabled.");
        } else {
            ui.horizontal(|ui| {
                track_item |= ui
                    .add(Slider::new(&mut self.target, 0..=(num_rows - 1)))
                    .dragged();
                scroll_top |= ui.button("|<").clicked();
                scroll_bottom |= ui.button(">|").clicked();
                if ui.button("Clear").clicked() {
                    self.selected = HashSet::new();
                }
            });
        }

        ui.separator();

        let data = panel.data.clone();
        let keys = data.keys().collect::<Vec<&Uuid>>();
        let mut table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .sense(Sense::click())
            .cell_layout(Layout::left_to_right(Align::Center))
            .column(Column::auto().at_least(100.));
        if track_item {
            table = table.scroll_to_row(self.target, Some(Align::Center));
        }
        if scroll_top {
            table = table.scroll_to_row(0, Some(Align::BOTTOM));
        }
        if scroll_bottom {
            table = table.scroll_to_row(self.data.len(), Some(Align::BOTTOM));
        }
        table.body(|body| {
            body.rows(20., keys.len(), |mut row| {
                let row_index = row.index();
                row.set_selected(self.selected.contains(&keys[row_index]));
                row.col(|ui| {
                    ui.label(format!("{}", panel.data[&keys[row_index]]));
                });
                self.toggle_row_selection(&keys[row_index], &row.response());
            });
        });
    }

    pub fn toggle_row_selection(&mut self, target: &Uuid, row_response: &egui::Response) {
        if row_response.clicked() {
            if self.selected.contains(target) {
                self.selected.remove(target);
            } else {
                self.selected.insert(target.clone());
            }
        }
    }

    // pub fn contains(&mut self, fragment: &str) {
    //     self.data = self.data.iter().filter(|v| v.contains(fragment, SearchConfig::default())).cloned().collect();
    // }

    pub fn contains(&mut self, fragment: &str) {
        self.data.retain(|k, v| {
            let key = k.to_string();
            let val = v.to_string();
            if key.contains(fragment) | val.contains(fragment) {
                true
            } else {
                false
            }
        });
    }
}

pub trait Card {
    fn contains(&self, fragment: &str, config: SearchConfig) -> bool;
    fn show(&self, ui: &mut Ui);
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Copy)]
pub struct SearchConfig {
    pub case_sensitive: bool,
}
