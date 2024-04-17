use crate::prelude::{Address, Addresses, Parcels, Tabular, TableView};
use egui::{Align, Color32, Context, DragValue, Layout, ScrollArea, Sense, Slider, TextStyle, Ui};
use egui_extras::{Column, TableBuilder};
use itertools::sorted;
use std::collections::{BTreeMap, HashSet, HashMap};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct UiState {
    pub addresses: Option<Addresses>,
    pub address_table: Option<TableView<Addresses, Address>>,
    pub counter: i32,
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
            },
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
            panel,
            parcels,
        }
    }
    pub fn run(&mut self, ui: &Context) {
        egui::Window::new("Whimsy UI").show(ui, |ui| {
            ui.heading("Window");
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

        egui::Window::new("Parcels").show(ui, |ui| {
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
                        ui.label(format!("Owner: {}", name));
                        ui.label(format!("Map #: {}", &record.id));
                    }
                });
            } else {
                ui.label("None loaded.");
            }
        });

        egui::Window::new("Addresses").show(ui, |ui| {
            if let Some(panel) = &mut self.panel {
                panel.table(ui);
            }

        });

        egui::Window::new("Address Table").show(ui, |ui| {
            if let Some(values) = &mut self.address_table {
                values.table(ui);
            }

        });

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

impl<K: Eq + std::hash::Hash + Ord + Clone + std::fmt::Display + Default, V: std::fmt::Display + Clone + Default + Eq + std::hash::Hash> HashPanel<K, V> {

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
                track_item |= ui.add(Slider::new(&mut self.target, 0..=(num_rows - 1)))
                    .dragged();
                scroll_top |= ui.button("|<").clicked();
                scroll_bottom |= ui.button(">|").clicked();
            });
        }
        ui.separator();
        ScrollArea::vertical().max_height(400.)
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
                                let response =
                                    ui.selectable_value(&mut self.value, self.data[keys[item]].clone(), format!("{}: {}", keys[item], self.data[keys[item]]));
                                response.scroll_to_me(Some(Align::Center));
                                self.value = self.data[keys[item]].clone();
                            } else {
                                ui.selectable_value(&mut self.value, self.data[keys[item]].clone(), format!("{}: {}", keys[item], self.data[keys[item]]));
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
                track_item |= ui.add(Slider::new(&mut self.target, 0..=(num_rows - 1)))
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
        table
            .body(|body| {
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
        let data = data.iter().map(|v| {
            let k = Uuid::new_v4();
            (k, v.clone())
        }).collect::<HashMap<Uuid, T>>();
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
                track_item |= ui.add(Slider::new(&mut self.target, 0..=(num_rows - 1)))
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
        table
            .body(|body| {
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


