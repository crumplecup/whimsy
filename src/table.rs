use crate::prelude::Tree;
use egui::{Align, Layout, Sense, Slider, Ui};
use egui_extras::{Column, TableBuilder};
use names::Generator;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use uuid::Uuid;

/// The `TableView` struct contains data fields to implement GUI functionality on tabular data.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct TableView<T: Tabular<U> + Filtration<T, V> + Clone, U: Columnar, V: Default> {
    /// Title to display for the table.
    pub name: String,
    /// Data source for the table.
    pub data: T,
    /// View of the data source created on the fly.
    pub view: T,
    /// Packages the active view for delivery to the GIS.
    pub package: Option<T>,
    /// Configuration parameters for table creation.
    pub config: TableConfig,
    /// Focus tree for navigation.
    pub tree: Tree,
    /// Holds user input for the search widget.
    pub search: String,
    /// Tracks rows selected by the user in the table.
    pub selection: HashSet<Uuid>,
    /// The `enter` field tracks use of the enter key.
    pub enter: Option<()>,
    /// Tracks checked boxes for rows using `row_ids`.
    pub checks: HashMap<Uuid, bool>,
    /// Tracks ordering button state in headings.
    pub ord_flags: Vec<bool>,
    /// Set to index of ord flags to refresh ordering of rows.
    pub set_ord: Option<usize>,
    /// Holds filter selection for the filter widget.
    pub filter: Option<V>,
    /// Row target for the slider widget.
    pub target: usize,
    /// The current row in focus.  Used to hold the current row id in the focus tree.
    pub row_select: Option<Uuid>,
    /// The `row_focus` field signals a change in row focus.
    pub row_focus: Option<Uuid>,
    // Current index associated with the id in `row_select`.
    row_index: Option<usize>,
    // The uuid associated with each row.
    row_ids: Vec<Uuid>,
    // Indicates if the focus tree has been loaded.
    loaded: bool,
    // Index of leaf ids for the data in `view`.
    leaves: Vec<egui::Id>,
    // Marker to appease the type checker.
    phantom: PhantomData<U>,
}

impl<T: Tabular<U> + Default + Filtration<T, V> + Clone, U: Columnar + Default, V: Default>
    TableView<T, U, V>
{
    /// Creates a new table view of data `data` with the default configuration.
    pub fn new(data: T) -> Self {
        // The initial view is a clone of the source data.
        // We keep the source data unmodified.
        let view = data.clone();
        // Each time we create a new view, package a clone for the GIS.
        let package = Some(data.clone());
        let cols = T::headers().len();
        let ord_flags = vec![false; cols];
        Self {
            name: String::new(),
            data,
            view,
            package,
            ord_flags,
            ..Default::default()
        }
    }

    pub fn with_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    /// The `view` method provides a reference to the `view` field.
    pub fn view(&self) -> &T {
        &self.view
    }

    /// The `view_mut` method provides a mutable reference to the `view` field.
    pub fn view_mut(&mut self) -> &mut T {
        &mut self.view
    }

    pub fn checks(&self) -> &HashMap<Uuid, bool> {
        &self.checks
    }

    pub fn checks_mut(&mut self) -> &mut HashMap<Uuid, bool> {
        &mut self.checks
    }

    pub fn row_ids(&self) -> &Vec<Uuid> {
        &self.row_ids
    }

    /// Creates a new `TableView` from `data` with configuration parameters `config`.
    pub fn with_config(data: T, config: TableConfig) -> Self {
        let view = data.clone();
        let package = Some(data.clone());
        let mut cols = T::headers().len();
        if config.checked {
            cols += 1;
        }
        let ord_flags = vec![false; cols];
        Self {
            data,
            view,
            package,
            config,
            ord_flags,
            ..Default::default()
        }
    }

    /// Inserts the row index into the `selection` hash set if not present, removes it if present.
    fn toggle_row_selection(&mut self, row_id: &Uuid, row_response: &egui::Response) {
        if row_response.clicked() {
            if self.selection.contains(row_id) {
                self.selection.remove(row_id);
            } else {
                self.selection.insert(*row_id);
            }
        }
    }

    /// Add search widget to table.
    pub fn search_panel(&mut self, ui: &mut Ui) {
        if self.config.search {
            ui.horizontal(|ui| {
                let entry =
                    ui.add(egui::TextEdit::singleline(&mut self.search).hint_text("Search"));
                let clear = ui.button("X");
                if clear.clicked() {
                    self.search = Default::default();
                };

                // if !self.loaded {
                //     let entry_id = self.tree.leaf(entry.id);
                //     let clear_id = self.tree.leaf(clear.id);
                //     let node_id = self.tree.node();
                //     self.tree.with_leaf(node_id, entry_id);
                //     self.tree.with_leaf(node_id, clear_id);
                // }
            });
        }
    }

    /// Change table configuration to enable the search widget.
    pub fn searchable(&mut self) -> &mut Self {
        self.config.search = true;
        self
    }

    /// The `slider` method creates the slider widget for navigating tables with large numbers of
    /// rows.  The `num_size` parameter represents the number of rows in the table.
    pub fn slider(&mut self, ui: &mut Ui, num_rows: usize) -> bool {
        let mut track_item = false;
        if self.config.slider {
            // bounds check
            if num_rows == 0 {
                ui.label("Tracker disabled.");
            } else {
                ui.horizontal(|ui| {
                    // if the slider is being dragged, engage tracking
                    track_item |= ui
                        .add(Slider::new(&mut self.target, 0..=(num_rows - 1)))
                        .dragged();
                    // return to start button
                    let beginning = ui.button("|<");
                    if beginning.clicked() {
                        self.target = 0;
                        track_item = true;
                    };
                    // jump to end button
                    let end = ui.button(">|");
                    if end.clicked() {
                        self.target = num_rows - 1;
                        track_item = true;
                    };
                    // If loaded is false, the focus tree is new or has changed.
                    if !self.loaded {
                        let _ = self.tree.leaf(beginning.id);
                        let _ = self.tree.leaf(end.id);
                        let node_id = self.tree.node();
                        self.tree.with_leaf(node_id, beginning.id);
                        self.tree.with_leaf(node_id, end.id);
                    }
                });
            }
        }
        track_item
    }

    /// Enable the slider widget in the table view.  Sets the `slider` field in the [`TableConfig`]
    /// within `config` to `true`.
    pub fn with_slider(&mut self) -> &mut Self {
        self.config.slider = true;
        self
    }

    /// The `leaves` method creates a [`Leaf`] for each row in the table, and tracks their [`Uuid`]
    /// in the field `leaves`.
    pub fn leaves(&mut self, len: usize) {
        let mut names = Generator::default();
        let mut leaves = Vec::new();
        let node_id = self.tree.node();
        for _ in 0..(len - 1) {
            let egui_id = egui::Id::new(names.next().unwrap());
            let leaf = self.tree.leaf(egui_id);
            leaves.push(leaf.id);
            self.tree.with_leaf(node_id, egui_id);
        }
        self.leaves = leaves;
    }

    /// UI display for the table view.
    pub fn table(&mut self, ui: &mut Ui) {
        // Each row contains a string value for each column in the table.
        let mut rows = self.view.rows();
        if !self.search.is_empty() {
            // the subset of rows containing the search term in any column
            rows = self.contains(&self.search);
        }
        if let Some(column) = self.set_ord.take() {
            tracing::info!("Column ordering requested for {}", column);
            let flag = self.ord_flags[column];
            self.view_mut().sort_by_col(column, flag);
        }
        // Collect the ids of each row.
        self.row_ids = rows.iter().map(|v| *v.id()).collect::<Vec<Uuid>>();
        if !self.loaded {
            self.leaves(rows.len());
        }

        if !self.row_ids.is_empty() {
            // Start selected at the first row.
            if let Some(index) = self.row_index {
                self.row_select = Some(self.row_ids[index]);
            } else {
                self.row_index = Some(0);
            }
        }
        // Creates a slider.  If slider turns true, snap focus to the target row.
        let track_item = self.slider(ui, rows.len());
        // Column headers for the table display.
        let mut headers = T::headers();
        if self.config.checked {
            headers.insert(0, "Show".to_string());
        }
        // Create the search panel widget.
        self.search_panel(ui);
        // Construct the table.
        let mut table = TableBuilder::new(ui)
            .striped(self.config.striped)
            .resizable(self.config.resizable)
            .sense(Sense::click())
            .cell_layout(Layout::left_to_right(Align::Center))
            .columns(Column::auto(), headers.len());
        // Enable row tracking on the slider.
        if track_item {
            table = table.scroll_to_row(self.target, Some(Align::Center));
        }
        // Enable row tracking on keyboard entry.
        if self.row_focus.take().is_some() {
            if let Some(index) = self.row_index {
                table = table.scroll_to_row(index, Some(Align::Center));
            }
        }

        let mut id = crate::identifier::Identifier::default();
        // ui.push_id(id.name(), |ui: &mut Ui| {
        // Populate the table.
        table
            // iterate through the headers and print them in bold as the header of each column.
            .header(20.0, |mut header| {
                headers
                    .iter()
                    .enumerate()
                    .map(|(i, v)| {
                        header.col(|ui| {
                            ui.push_id(id.name(), |ui| {
                                ui.horizontal(|ui| {
                                    ui.strong(v);
                                    // Offset the column index if the checked column is not there.
                                    // Checked is the first column, so subtract index numbers greater
                                    // than one by one.
                                    // Since the "order by" check box for row zero is not visible when
                                    // the config for checked is false, the input from the user cannot
                                    // be zero.
                                    let flag = if self.config.checked && i > 0 {
                                        i - 1
                                    } else {
                                        // If config is checked, pass i normally.
                                        i
                                    };
                                    // Flag indicates the column, while ord flag indicates the ordering
                                    // at the column.
                                    let symbol = match self.ord_flags[flag] {
                                        true => "⏷",
                                        false => "⏶",
                                    };
                                    let ord_button = ui.button(symbol);
                                    if ui.button(symbol).clicked {
                                        if self.config.checked && i > 0 {
                                            self.set_ord = Some(i - 1);
                                            self.ord_flags[i - 1] = !self.ord_flags[i - 1];
                                        } else {
                                            self.set_ord = Some(i);
                                            self.ord_flags[i] = !self.ord_flags[i];
                                        }
                                        tracing::info!("Ord flags set.");
                                    };
                                });
                            });
                        })
                    })
                    .for_each(drop);
            })
            .body(|body| {
                body.rows(20., rows.len(), |mut row| {
                    let row_index = row.index();
                    let row_data = &rows[row_index];
                    let row_id = row_data.id();
                    row.set_selected(self.selection.contains(row_id));
                    let columns = row_data.values();

                    if self.config.checked {
                        // Adds a checkbox column linked to the `checks` field.
                        if !self.checks.contains_key(row_id) {
                            self.checks.insert(*row_id, false);
                        }
                        let checked = self.checks.get_mut(row_id);
                        if let Some(check) = checked {
                            row.col(|ui| {
                                ui.push_id(id.name(), |ui| {
                                    ui.checkbox(check, "");
                                });
                            });
                        } else {
                            tracing::info!("Bad checkbox reference.");
                            row.col(|ui| {
                                ui.push_id(id.name(), |ui| {
                                    ui.label("No box");
                                });
                            });
                        }
                    }

                    columns
                        .iter()
                        .map(|v| {
                            row.col(|ui| {
                                ui.push_id(id.name(), |ui: &mut Ui| {
                                    ui.label(v);
                                });
                                // ui.label(v);
                            });
                        })
                        .for_each(drop);
                    self.toggle_row_selection(row_id, &row.response());
                });
            });
        // });
    }

    pub fn contains(&self, fragment: &str) -> Vec<U> {
        let mut data = Vec::new();
        let rows = self.view.rows();
        for row in rows {
            let mut contains = false;
            let cols = row.values();
            for col in cols {
                let mut value = col;
                let mut frag = fragment.to_string();
                if !self.config.case_sensitive {
                    value = value.to_lowercase();
                    frag = frag.to_lowercase();
                }
                if value.contains(&frag) {
                    contains = true;
                }
            }
            if contains {
                data.push(row);
            }
        }
        data
    }

    /// Returns the [`Uuid`] of the current row in focus.
    pub fn current_row(&self) -> Option<Uuid> {
        self.row_select
    }

    /// Sets the focus to the current row.
    pub fn select_current(&mut self) {
        self.row_focus = self.current_row();
    }

    /// Advances focus to the next row and returns the new row [`Uuid`].
    pub fn next_row(&mut self) -> Option<Uuid> {
        // take a mutable reference to the index of the row
        if let Some(index) = &mut self.row_index {
            tracing::info!("Current index: {}", index);
            tracing::info!("Advancing row index.");
            // Wraps to beginning if at the end
            if (*index + 1) > (self.row_ids.len() - 1) {
                *index = 0;
                tracing::info!("Wrapped row index to 0.");
            } else {
                *index += 1;
                tracing::info!("Adding one: {}", index);
            }
            // match the selected row id to the updated index.
            self.row_select = Some(self.row_ids[*index]);
            if let Some(id) = self.row_select {
                tracing::info!("Row id: {}", id);
            }
        }
        self.row_select
    }

    /// Sets the focus to the next row.
    pub fn select_next(&mut self) {
        tracing::info!("Setting row focus.");
        let next = self.next_row();
        tracing::info!("Next focus: {:?}", next);
        self.row_focus = next;
    }

    /// Moves focus to the previous row and returns the new row [`Uuid`].
    pub fn previous_row(&mut self) -> Option<Uuid> {
        if let Some(mut index) = self.row_index {
            tracing::info!("Decrementing row index.");
            if index == 0 {
                index = self.row_ids.len() - 1;
            } else {
                index -= 1;
                tracing::info!("Minus one: {}", index);
            }
            self.row_index = Some(index);
            tracing::info!("Row index: {}", index);
            self.row_select = Some(self.row_ids[index]);
            if let Some(id) = self.row_select {
                tracing::info!("Row id: {}", id);
            }
        }
        self.row_select
    }

    /// Sets the focus to the
    pub fn select_previous(&mut self) {
        tracing::info!("Setting row focus.");
        self.row_focus = self.previous_row();
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct TableConfig {
    pub case_sensitive: bool,
    pub checked: bool,
    pub resizable: bool,
    pub search: bool,
    pub slider: bool,
    pub striped: bool,
}

impl TableConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn checked(mut self) -> Self {
        self.checked = true;
        self
    }

    pub fn resizable(mut self) -> Self {
        self.resizable = true;
        self
    }

    pub fn with_search(mut self) -> Self {
        self.search = true;
        self
    }

    pub fn with_slider(mut self) -> Self {
        self.slider = true;
        self
    }

    pub fn striped(mut self) -> Self {
        self.striped = true;
        self
    }

    pub fn case_sensitive(mut self) -> Self {
        self.case_sensitive = true;
        self
    }
}

pub trait Tabular<T: Columnar> {
    fn headers() -> Vec<String>;
    fn rows(&self) -> Vec<T>;
    fn sort_by_col(&mut self, column_index: usize, reverse: bool);
    fn len(&self) -> usize {
        self.rows().len()
    }
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    // fn leaves(&self) -> Node {
    //     let mut node = Node::new();
    //     let rows = self.rows();
    //     rows.iter().map(|v| node.with_leaf(v)).for_each(drop);
    //     node
    // }
}

pub trait Columnar {
    fn names() -> Vec<String>;
    fn values(&self) -> Vec<String>;
    fn id(&self) -> &Uuid;
}

pub trait Filtration<T, U> {
    fn filter(self, filter: &U) -> T;
}
