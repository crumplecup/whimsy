use crate::prelude::{Address, Addresses, Parcels};
use egui::{Align, Color32, Context, DragValue, ScrollArea, Slider, TextStyle, Ui};
use itertools::sorted;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct UiState {
    pub addresses: Option<Addresses>,
    pub counter: i32,
    pub panel: Option<Panel<Address>>,
    pub parcels: Option<Arc<Parcels>>,
    pub scroll_to: ScrollTo,
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
        // let addresses = match Addresses::load("data/addresses.data") {
        //     Ok(data) => {
        //         panel = Some(Panel::new(data.records.clone()));
        //         Some(data)
        //     },
        //     Err(_) => None,
        // };
        let addresses = match Addresses::from_csv("/home/erik/geojson/addresses.csv") {
            Ok(data) => {
                panel = Some(Panel::new(data.records.clone()));
                Some(data)
            },
            Err(_) => None,
        };


        let parcels = match Parcels::load("data/parcels.data") {
            Ok(data) => Some(Arc::new(data)),
            Err(_) => None,
        };

        Self {
            addresses,
            counter: Default::default(),
            panel,
            parcels,
            scroll_to: Default::default(),
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
                panel.show(ui);
            }

        });

        // egui::Window::new("Addresses").show(ui, |ui| {
        //     self.scroll_to.ui(ui);
        //
        // });
    }

}

#[derive(Clone, Debug, PartialEq)]
pub struct ScrollTo {
    pub track_item: usize,
    pub tack_item_align: Option<Align>,
    pub offset: f32,
}

impl Default for ScrollTo {
    fn default() -> Self {
        Self {
            track_item: 25,
            tack_item_align: Some(Align::Center),
            offset: 0.0,
        }
    }
}

impl ScrollTo {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.label("This shows how you can scroll to a specific item or pixel offset");

        let num_items = 500;

        let mut track_item = false;
        let mut go_to_scroll_offset = false;
        let mut scroll_top = false;
        let mut scroll_bottom = false;

        ui.horizontal(|ui| {
            ui.label("Scroll to a specific item index:");
            track_item |= ui
                .add(Slider::new(&mut self.track_item, 1..=num_items).text("Track Item"))
                .dragged();
        });

        ui.horizontal(|ui| {
            ui.label("Item align:");
            track_item |= ui
                .radio_value(&mut self.tack_item_align, Some(Align::Min), "Top")
                .clicked();
            track_item |= ui
                .radio_value(&mut self.tack_item_align, Some(Align::Center), "Center")
                .clicked();
            track_item |= ui
                .radio_value(&mut self.tack_item_align, Some(Align::Max), "Bottom")
                .clicked();
            track_item |= ui
                .radio_value(&mut self.tack_item_align, None, "None (Bring into view)")
                .clicked();
        });

        ui.horizontal(|ui| {
            ui.label("Scroll to a specific offset:");
            go_to_scroll_offset |= ui
                .add(DragValue::new(&mut self.offset).speed(1.0).suffix("px"))
                .dragged();
        });

        ui.horizontal(|ui| {
            scroll_top |= ui.button("Scroll to top").clicked();
            scroll_bottom |= ui.button("Scroll to bottom").clicked();
        });

        let mut scroll_area = ScrollArea::vertical().max_height(200.0).auto_shrink(false);
        if go_to_scroll_offset {
            scroll_area = scroll_area.vertical_scroll_offset(self.offset);
        }

        ui.separator();
        let (current_scroll, max_scroll) = scroll_area
            .show(ui, |ui| {
                if scroll_top {
                    ui.scroll_to_cursor(Some(Align::TOP));
                }
                ui.vertical(|ui| {
                    for item in 1..=num_items {
                        if track_item && item == self.track_item {
                            let response =
                                ui.colored_label(Color32::YELLOW, format!("This is item {item}"));
                            response.scroll_to_me(self.tack_item_align);
                        } else {
                            ui.label(format!("This is item {item}"));
                        }
                    }
                });

                if scroll_bottom {
                    ui.scroll_to_cursor(Some(Align::BOTTOM));
                }

                let margin = ui.visuals().clip_rect_margin;

                let current_scroll = ui.clip_rect().top() - ui.min_rect().top() + margin;
                let max_scroll = ui.min_rect().height() - ui.clip_rect().height() + 2.0 * margin;
                (current_scroll, max_scroll)
            })
            .inner;
        ui.separator();

        ui.label(format!(
            "Scroll offset: {current_scroll:.0}/{max_scroll:.0} px"
        ));

        ui.separator();
    }
}

pub fn runner(state: &mut UiState, ui: &Context) {
    egui::Window::new("Whimsy UI").show(ui, |ui| {
        ui.heading("Window");
        if ui.button("Increment").clicked() {
            state.counter += 1;
        }
        ui.label(format!("{}", state.counter));
    });
}

#[derive(Clone, Debug)]
pub struct HashPanel<K, V> {
    pub data: HashMap<K, V>,
    pub selected: usize,
    pub search: String,
    pub value: V,
}

impl<K: Eq + std::hash::Hash + Ord + Clone + std::fmt::Display, V: std::fmt::Display + Clone + Default + Eq> HashPanel<K, V> {

    pub fn new(data: HashMap<K, V>) -> Self {
        let selected = 0;
        let search = String::new();
        let value = Default::default();
        Self {
            data,
            selected,
            search,
            value,
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
        if num_rows == 0 {
            ui.label("Tracker disabled.");
        } else {
            ui.horizontal(|ui| {
                track_item |= ui.add(Slider::new(&mut self.selected, 0..=(num_rows - 1)).text("Track Item"))
                    .dragged();
            });
        }

        let mut scroll_top = false;
        let mut scroll_bottom = false;
        ui.horizontal(|ui| {
            scroll_top |= ui.button("|<").clicked();
            scroll_bottom |= ui.button(">|").clicked();
        });

        ui.horizontal(|ui| {
            ui.add(egui::TextEdit::singleline(&mut self.search).hint_text("Search"));
            if ui.button("X").clicked() {
                self.search = Default::default();
            }

        });
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
                            if track_item && item == self.selected {
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

}

#[derive(Clone, Debug)]
pub struct Panel<T> {
    pub data: Vec<T>,
    pub selected: usize,
    pub search: String,
    pub value: Option<T>,
}

impl<T: PartialEq + Clone + std::fmt::Display + Card> Panel<T> {

    pub fn new(data: Vec<T>) -> Self {
        let selected = 0;
        let search = String::new();
        let value = Default::default();
        Self {
            data,
            selected,
            search,
            value,
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        let mut panel = self.clone();
        if !self.search.is_empty() {
            panel.contains(&self.search);
        }
        let num_rows = panel.data.len();
        let mut track_item = false;
        if num_rows == 0 {
            ui.label("Tracker disabled.");
        } else {
            ui.horizontal(|ui| {
                track_item |= ui.add(Slider::new(&mut self.selected, 0..=(num_rows - 1)).text("Track Item"))
                    .dragged();
            });
        }

        let mut scroll_top = false;
        let mut scroll_bottom = false;
        ui.horizontal(|ui| {
            scroll_top |= ui.button("|<").clicked();
            scroll_bottom |= ui.button(">|").clicked();
        });

        ui.horizontal(|ui| {
            ui.add(egui::TextEdit::singleline(&mut self.search).hint_text("Search"));
            if ui.button("X").clicked() {
                self.search = Default::default();
            }

        });
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
                            if track_item && item == self.selected {
                                let response =
                                    ui.selectable_value(&mut self.value, Some(panel.data[item].clone()), format!("{}", panel.data[item]));
                                response.scroll_to_me(Some(Align::Center));
                                self.value = Some(panel.data[item].clone());
                            } else {
                                ui.selectable_value(&mut self.value, Some(panel.data[item].clone()), format!("{}", panel.data[item]));
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
        ui.label(
            if let Some(value) = &self.value {
                format!("Value selected: {}", value)
            } else {
                format!("No value selected.")
            });
    }

    pub fn contains(&mut self, fragment: &str) {
        self.data = self.data.iter().filter(|v| v.contains(fragment, SearchConfig::default())).cloned().collect();
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


