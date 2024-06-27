use crate::prelude::{
    load_bin, save, AddressPoint, AddressPoints, CommandMode, CommandTable, CommandView, EguiAct,
    Panel, Parcels, TableConfig, TableView, Tree,
};
use derive_more::{Deref, DerefMut};
use egui::{Context, Id, TextStyle};
use polite::Polite;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Lens {
    pub addresses: Option<AddressPoints>,
    pub address_table: Option<TableView<AddressPoints, AddressPoint, String>>,
    pub counter: i32,
    /// Command view window.
    pub command_view: CommandView,
    pub focus_tree: Tree,
    pub focus_counter: bool,
    pub focus_parcels: bool,
    pub panel: Option<Panel<AddressPoint>>,
    pub parcels: Option<Arc<Parcels>>,
    pub enter: Option<()>,
}

impl Lens {
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
        let addresses = match AddressPoints::load("data/addresses.data") {
            Ok(data) => {
                // panel = Some(Panel::new(data.records.clone()));
                let config = TableConfig::new()
                    .checked()
                    .resizable()
                    .with_search()
                    .striped()
                    .with_slider();
                address_table = Some(TableView::with_config(data.clone(), config));
                tracing::info!("Records read: {}", data.records.len());
                Some(data)
            }
            Err(e) => {
                tracing::info!("Could not read records: {}", e.to_string());
                None
            }
        };

        let parcels = match Parcels::load("data/parcels.data") {
            Ok(data) => Some(Arc::new(data)),
            Err(_) => None,
        };

        let command_tree = CommandMode::new();
        let command_table = CommandTable::from(&command_tree);
        let command_view = CommandView::from(&command_table);

        Self {
            addresses,
            address_table,
            counter: Default::default(),
            command_view,
            focus_tree: Tree::new(),
            focus_counter: true,
            focus_parcels: true,
            panel,
            parcels,
            enter: None,
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
            EguiAct::NextRow => {
                if let Some(table) = &mut self.address_table {
                    tracing::info!("Selecting next row.");
                    table.select_next();
                }
            }
            EguiAct::PreviousRow => {
                if let Some(table) = &mut self.address_table {
                    tracing::info!("Selecting previous row.");
                    table.select_previous();
                }
            }
            EguiAct::Be => tracing::trace!("Taking no action."),
        }
    }

    /// Receiver for an ['Act'] sent from the main event loop.
    pub fn enter(&mut self) {
        tracing::trace!("State for Enter set.");
        self.enter = Some(());
    }

    pub fn run(&mut self, ui: &Context) {
        // let mut set_address = None;
        let mut set_counter = None;
        let mut set_counter1 = None;
        let mut set_parcels = None;
        if self.focus_tree.flags.is_empty() {
            // set_address = Some(self.focus_tree.window());
            set_counter = Some(self.focus_tree.window());
            set_counter1 = Some(self.focus_tree.window());
            set_parcels = Some(self.focus_tree.window());
        }
        egui::Window::new("Whimsy UI").show(ui, |ui| {
            let heading = ui.heading("Window");
            if self.in_focus(heading.id) {
                tracing::trace!("Requesting focus for {:#?}", heading.id);
                heading.request_focus();
                tracing::trace!("Clearing select.");
                self.focus_tree.select = None;
            }

            let button = ui.button("Counter");
            if self.in_focus(button.id) {
                tracing::trace!("Requesting focus for {:#?}", button.id);
                button.request_focus();
                tracing::trace!("Clearing select.");
                self.focus_tree.select = None;
            }
            if button.clicked {
                self.enter();
            }
            if let Some(_) = self.enter.take() {
                self.counter += 1;
            }
            ui.label(format!("Counter ID: {:?}", button.id));
            // if button.has_focus() {
            //     ui.label("Focused: Click <Enter> to increment counter.");
            // }

            let inc = ui.button("Increment");
            if self.in_focus(inc.id) {
                tracing::trace!("Requesting focus for {:#?}", inc.id);
                inc.request_focus();
                tracing::trace!("Clearing select.");
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
                tracing::trace!("Requesting focus for {:#?}", button.id);
                button.request_focus();
                tracing::trace!("Clearing select.");
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
                tracing::trace!("Requesting focus for {:#?}", inc.id);
                inc.request_focus();
                tracing::trace!("Clearing select.");
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
                                    tracing::trace!("Requesting focus for {:#?}", owner.id);
                                    owner.request_focus();
                                    tracing::trace!("Clearing select.");
                                    select = None;
                                }
                            }

                            ui.label(format!("Map #: {}", &record.id));
                        }
                        if let Some(id) = set_parcels {
                            tracing::trace!("Tree: {:#?}", self.focus_tree);
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

        let address_table = egui::Window::new("Address Table").show(ui, |ui| {
            if let Some(values) = &mut self.address_table {
                values.table(ui);
            }
        });
        // if let Some(res) = address_table {
        //     tracing::info!("Window id: {:?}", res.response.id);
        // }

        // if let Some(id) = set_address {
        //     tracing::info!("Window id is {}", id);
        //
        // }

        // egui::Window::new("Addresses").show(ui, |ui| {
        //     self.scroll_to.ui(ui);
        //
        // });

        egui::Window::new("Commands").show(ui, |ui| self.command_view.show(ui));
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Polite<()> {
        save(&self, path)?;
        Ok(())
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Polite<Self> {
        let records = load_bin(path)?;
        let decode: Self = bincode::deserialize(&records[..])?;
        Ok(decode)
    }
}

#[derive(Debug, Clone, Default, Deref, DerefMut)]
pub struct Tab(Lens);

impl Tab {
    pub fn new(lens: Lens) -> Self {
        Self(lens)
    }

    pub fn run_ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello egui!");
        });
    }
}

impl egui_dock::TabViewer for Tab {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        "TabbaT".into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        let ctx = ui.ctx();
        tab.run(ctx);
    }
}
