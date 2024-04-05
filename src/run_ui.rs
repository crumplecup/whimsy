use crate::prelude::{Addresses, Parcels};
use egui::{Align, Color32, Context, DragValue, ScrollArea, Slider, TextStyle, Ui};
use egui::containers::panel::Side;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct UiState {
    pub addresses: Option<Addresses>,
    pub counter: i32,
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
        let addresses = match Addresses::load("data/addresses.data") {
            Ok(data) => Some(data),
            Err(_) => None,
        };

        let parcels = match Parcels::load("data/parcels.data") {
            Ok(data) => Some(Arc::new(data)),
            Err(_) => None,
        };

        Self {
            addresses,
            counter: Default::default(),
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
        egui::SidePanel::right("Sidebar").show(ui, |ui| {
            ui.label("Address Info:");
            if let Some(data) = &self.addresses {
                let row_height = ui.text_style_height(&text_style);
                let num_rows = data.records.len();
                egui::ScrollArea::vertical().show_rows(
                    ui,
                    row_height,
                    num_rows, 
                    |ui, row_range| {
                    for row in row_range {
                        ui.label(format!("{:#?}", data.records[row]));
                    }
                });
            } else {
                ui.label("None loaded.");
            }
        });

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
            self.scroll_to.ui(ui);

        });
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
