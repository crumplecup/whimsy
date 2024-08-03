use crate::controls::act;
use crate::identifier::Identifier;
use crate::observer;
use crate::rpg::character::{Attributes, Character, DisplayField};
use crate::rpg::players;
use crate::table::{Columnar, Filtration, TableView, Tabular};
use derive_more::{Deref, DerefMut};
// use egui_dock::dock_state::surface_index::SurfaceIndex;
// use egui_dock::dock_state::tree::{node_index::NodeIndex, tab_index::TabIndex};
use egui_dock::{NodeIndex, SurfaceIndex, TabIndex};
use std::collections::HashSet;

// pub type Tab = table::TableView<character::Attributes, character::DisplayField, String>;
/// The `Tab` type follows the naming convention of [`egui_dock`].
/// We could stick this definition inside the impl of [`egui_dock::TabViewer`] for [`TabViewer`],
/// but since we are constantly swapping it out with new variations in the development process, I
/// placed it top of module for high visibility and easy access.
///
/// The [`TabView`] holds a view of a [`Attributes`], currently Paeva.
/// The [`DisplayField`] defines the content of columns in the table.
/// The [`String`] is the type used for enabling search within the contents of the table.
pub type Tab = Character;
// pub type Tab = TabView<Attributes, DisplayField, String>;

/// The `TabView` struct is a wrapper around a [`TableView`] that provides a unique name for the
/// owning [`egui_dock::DockState`].
/// The `data` field holds the original data used to generate the [`TableView`].
/// The `view` field holds the possibly mutated version being displayed to the user.  The `view`
/// field depends upon the `data` field to restore information that is lost when rendering, for
/// example, filtered views.
/// The [`TableView`] contains a similar pattern, so perhpas it is unnecessary here?
#[derive(Debug, Clone, Default, PartialEq, derive_getters::Getters, derive_setters::Setters)]
#[setters(prefix = "with_")]
pub struct TabView<T: Tabular<U> + Filtration<T, V> + Clone, U: Columnar, V: Default> {
    name: String,
    data: TableView<T, U, V>,
    view: TableView<T, U, V>,
}

impl<T: Tabular<U> + Filtration<T, V> + Clone, U: Columnar + Clone, V: Default + Clone>
    TabView<T, U, V>
{
    /// The `new` method creates an instance of a `TabView` from a [`TableView`] `data`.
    pub fn new(data: TableView<T, U, V>, name: &str) -> Self {
        let view = data.clone();
        Self {
            name: name.to_string(),
            data,
            view,
        }
    }

    /// The `named` method creates a new instance of a `TabView` with the title assigned to
    /// `name`, otherwise following the same pattern as the [`Self::new`] method.
    pub fn named(data: TableView<T, U, V>, name: &str) -> Self {
        let name = name.to_string();
        let view = data.clone();
        Self { name, data, view }
    }

    /// The `view_mut` method provides a mutable reference to the [`TableView`] in the `view` field.
    pub fn view_mut(&mut self) -> &mut TableView<T, U, V> {
        &mut self.view
    }
}

/// The `ContextMenu` is the menu of user options that appears when clicking on the add button in
/// the tab ui of an [`egui_dock::DockArea`], created by the [`TabState::ui`] method.
/// Match on the `ContextMenu` when adding nodes from the `added_nodes` field of [`TabViewer`] in
/// the method [`TabState::ui`].
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
    strum_macros::EnumIter,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum ContextMenu {
    #[default]
    /// The `App` variant indicates a menu-based interface using `egui`.
    App,
    /// The `Map` variant indicates a map-based interface using `galileo`.
    Map,
}

#[derive(Debug, Clone, derive_new::new, derive_getters::Getters)]
pub struct TabContext {
    /// The `kind` field holds the [`ContextMenu`] offered to user when clicking the add tab
    /// button.
    kind: ContextMenu,
    /// The `surface` field represents the "window" id in the [`egui_dock::DockArea`].
    surface: SurfaceIndex,
    /// The `node` field represents the "panel" id in the [`egui_dock::DockArea`].
    node: NodeIndex,
}

#[derive(derive_new::new)]
/// The `TabViewer` struct implements the [`egui_dock::TabViewer`] trait.
pub struct TabViewer<'a, 'b> {
    /// The `added_nodes` field holds a reference to any new tabs created by the user.
    added_nodes: &'a mut Vec<TabContext>,
    identifier: &'b mut Identifier,
}

impl egui_dock::TabViewer for TabViewer<'_, '_> {
    type Tab = Tab;

    /// The `title` method provides a unique id for [`egui`] to track, so each tab needs a unique
    /// title.
    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        if let Some(value) = tab.identifier() {
            value.into()
        } else {
            let name = tab.name();
            let id = self.identifier.number();
            let concat = format!("{name}: {id}");
            tab.with_identifier(concat.clone());
            concat.into()
        }
    }

    /// The `ui` method presents the user with an interface inside the [`egui_dock::DockArea`].
    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        let name = tab.name();
        ui.push_id(name, |ui| {
            tab.view(ui, name);
        });
        // tab.view(ui, name);
    }

    /// The `add_popup` method is an optional implementation to in this case to provide the add button with a
    /// context menu.
    fn add_popup(&mut self, ui: &mut egui::Ui, surface: SurfaceIndex, node: NodeIndex) {
        ui.set_min_width(120.0);
        ui.style_mut().visuals.button_frame = false;

        // Popup UI for the add button in a tab.
        // Convert to a match statement using .to_string() for the button text.
        // Use strum to enumerate through the ContextMenu, making the button and handler.
        if ui.button("App").clicked() {
            self.added_nodes
                .push(TabContext::new(ContextMenu::App, surface, node));
        }

        if ui.button("Map").clicked() {
            self.added_nodes
                .push(TabContext::new(ContextMenu::Map, surface, node));
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, derive_getters::Getters, derive_new::new)]
/// The `Record` struct identifies an active tab in the [`egui::DockState`].
pub struct Record {
    /// The surface is the window area that holds panels and tabs.
    /// The [`SurfaceIndex`] serves as the unique ID for surfaces in an [`egui_dock::DockState`].
    surface_index: SurfaceIndex,
    /// The node is the panel area within a window that holds tabs.
    /// Nodes are contained within a surface and contain tabs.
    /// The [`NodeIndex`] serves as the unique ID for nodes in an [`egui_dock::DockState`].
    node_index: NodeIndex,
    /// The tab is the endpoint that provides an [`egui`] interface to the user.
    /// Each tab has a parent node and surface.
    /// The [`TabIndex`] serves as the unique ID for tabs in an [`egui_dock::DockState`].
    tab_index: TabIndex,
}

impl Record {
    /// The `from_tab` method creates a `Record` from a provided `tab`.
    /// Uses `tab` as the needle tab for the [`egui_dock::DockState::find_tab`] method.
    pub fn from_tab(tab: &Tab, tree: &egui_dock::DockState<Tab>) -> Option<Self> {
        if let Some((surface_index, node_index, tab_index)) = tree.find_tab(tab) {
            Some(Self {
                surface_index,
                node_index,
                tab_index,
            })
        } else {
            None
        }
    }
}

/// The `Records` struct is a wrapper around a vector of type [`Record`].
/// Implements [`derive_more::Deref`] and [`derive_more::DerefMut`] to provide convenient access to
/// the underlying vector.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Deref, DerefMut)]
pub struct Records(Vec<Record>);

impl Records {
    /// The `surfaces` method returns a vector of type [`SurfaceIndex`].
    /// Each [`SurfaceIndex`] refers to a valid surface in the [`egui_dock::DockState`].
    pub fn surfaces(&self) -> Vec<SurfaceIndex> {
        let mut vec = self
            .iter()
            .map(|r| r.surface_index())
            .cloned()
            .collect::<Vec<SurfaceIndex>>();
        vec.dedup();
        vec
    }

    /// The `nodes` method returns a vector of type [`NodeIndex`].
    /// Each [`NodeIndex`] refers to a valid node in the [`egui_dock::DockState`].
    pub fn nodes(&self) -> Vec<NodeIndex> {
        let mut vec = self
            .iter()
            .map(|r| r.node_index())
            .cloned()
            .collect::<Vec<NodeIndex>>();
        vec.dedup();
        vec
    }

    /// The `tabs` method returns a vector of type [`TabIndex`].
    /// Each [`TabIndex`] refers to a valid tab in the [`egui_dock::DockState`].
    pub fn tabs(&self) -> Vec<TabIndex> {
        let mut vec = self
            .iter()
            .map(|r| r.tab_index())
            .cloned()
            .collect::<Vec<TabIndex>>();
        vec.dedup();
        vec
    }

    /// Subsets the index values for `nodes` that are valid for the active surface.
    /// So if only the index values of 1,2 and 4 were valid, then the vector of node
    /// ids would be [1, 2, 4].  In this way, we can perform increment, decrement and wrapping
    /// operatings on the subset of remaining values using the vector index of node ids, but still index into the correct node in the
    /// `nodes` field using the value of node ids.
    pub fn node_ids(&self, surface: &SurfaceIndex) -> Vec<usize> {
        // Subset the nodes in the surface.
        let in_surface = self.clone().filter_surface(surface).nodes();
        // For each node index remaining, collect the index value in the `nodes` field.
        self.nodes()
            .iter()
            .enumerate()
            .filter(|(_, v)| in_surface.contains(v))
            .map(|(i, _)| i)
            .collect::<Vec<usize>>()
    }

    /// Subsets the index values for `tabs` that are valid for the active node.
    /// So if only the index values of 1,2 and 4 were valid, then the vector of tab
    /// ids would be [1, 2, 4].  In this way, we can perform increment, decrement and wrapping
    /// operatings on the subset of remaining values using the vector index of node ids, but still index into the correct tab in the
    /// `tabs` field using the value of tab ids.
    pub fn tab_ids(&self, node: &NodeIndex) -> Vec<usize> {
        // Subset the tabs in the node.
        let in_node = self.clone().filter_node(node).tabs();
        // For each tab index remaining, collect the index value in the `tabs` field.
        self.tabs()
            .iter()
            .enumerate()
            .filter(|(_, v)| in_node.contains(v))
            .map(|(i, _)| i)
            .collect::<Vec<usize>>()
    }

    /// Obtain the subset of `Records` where the [`SurfaceIndex`] of the [`Record`] matches
    /// `surface`.
    /// Called by [`Self::node_ids`].
    pub fn filter_surface(mut self, surface: &SurfaceIndex) -> Self {
        self.retain(|v| v.surface_index == *surface);
        self
    }

    /// Obtain the subset of `Records` where the [`NodeIndex`] of the [`Record`] matches
    /// `node`.
    /// Called by [`Self::tab_ids`].
    pub fn filter_node(mut self, node: &NodeIndex) -> Self {
        self.retain(|v| v.node_index == *node);
        self
    }

    /// Obtain the subset of `Records` where the [`TabIndex`] of the [`Record`] matches
    /// `tab`.
    /// Unused within library.
    pub fn filter_tab(mut self, tab: &TabIndex) -> Self {
        self.retain(|v| v.tab_index == *tab);
        self
    }
}

impl From<&egui_dock::DockState<Tab>> for Records {
    /// We frequently need to refresh the `Records`, since the user can change the record by
    /// interacting with the tabs or clicking the add tab button.
    /// Implementing [`From`] for [`egui_dock::DockState<Tab>`] allows us to use the `from` method
    /// to create a new `Record` from the current [`egui_dock::DockState`].
    fn from(tree: &egui_dock::DockState<Tab>) -> Self {
        let records = tree
            .iter_all_tabs()
            .map(|((_, _), tab)| {
                Record::from_tab(tab, tree).expect("Iter tabs only returns tabs that exist.")
            })
            .collect::<Vec<Record>>();
        Self(records)
    }
}

/// The `TabState` holds persistent data related to the [`egui_dock::DockState`].
/// The `surfaces` field tracks active surfaces, including the Central Panel and any windows in the
/// [`egui_dock::DockArea`].
/// The `nodes` field tracks active nodes, which are panels created by dragging a tab into the
/// docking animation in the GUI or calling one of the splitting methods from the library such as
/// [`egui_dock::DockState::detach_tab`], contained within an active surface.
/// The `tabs` field tracks active tabs, contained within an active node.
/// Internally, the `TabState` tracks the active tab by indexing into the `surfaces`, `nodes` and
/// `tabs` fields using the `surface`, `node`, and `tab` fields.
/// When a tab is available, the `surface_index`, `node_index` and `tab_index` fields contain the
/// [`surface_index::SurfaceIndex`], [`node_index::NodeIndex`] and [`tab_index::TabIndex`] of the
/// active tab, respectively.
/// The `tab_names` field holds a [`HashSet`] of active tab names, because the
/// [`egui_dock::TabViewer::title`] method provides the unique source for the [`egui::Id`]
/// when creating new tabs.
/// The `observer` field controls observability, including logging using the `trace` crate and
/// toast notifications using `egui-notify`.
#[derive(derive_getters::Getters, derive_setters::Setters)]
#[setters(prefix = "with_")]
pub struct TabState {
    // The dock state tree.
    tree: egui_dock::DockState<Tab>,
    // Records of valid surface, node and tab states.
    records: Records,
    // Valid surfaces from `records`.
    surfaces: Vec<SurfaceIndex>,
    // Valid nodes from `records`.
    nodes: Vec<NodeIndex>,
    // Valid tabs from `records`.
    tabs: Vec<TabIndex>,
    // Active surface.
    surface_index: Option<SurfaceIndex>,
    // Active node.
    node_index: Option<NodeIndex>,
    // Active tab.
    tab_index: Option<TabIndex>,
    // Index of active surface in `records`.
    surface: usize,
    // Index of active node in `records`.
    node: usize,
    // Index of active tab in `records`.
    tab: usize,
    // Set of unique tab names.
    tab_names: HashSet<String>,
    // Observability helper.
    observer: observer::Observer,
    identifier: Identifier,
}

impl TabState {
    pub fn new() -> Self {
        // Create a `DockState` with an initial tab "tab1" in the main `Surface`'s root node.
        let paeva = players::Players::paeva();
        let identifier: Identifier = Default::default();
        let attributes = paeva.attributes();
        let table = TableView::new(*attributes);
        let mut gen = names::Generator::with_naming(names::Name::Numbered);
        let name = gen.next().expect("Could not get name from generator.");
        let mut tab_names = HashSet::new();
        tab_names.insert(name.clone());
        let tab_view = TabView::named(table, &name);
        let tree = egui_dock::DockState::new(vec![paeva]);
        let records = Records::from(&tree);
        let surfaces = records.surfaces();
        let nodes = records.nodes();
        let tabs = records.tabs();
        let mut surface_index = None;
        let mut node_index = None;
        let mut tab_index = None;
        let mut surface = 0;
        let mut node = 0;
        let mut tab = 0;
        if !records.is_empty() {
            tracing::info!("Starting tab found.");
            surface_index = Some(records[0].surface_index);
            node_index = Some(records[0].node_index);
            tab_index = Some(records[0].tab_index);
            surface = records[0].surface_index.0;
            node = records[0].node_index.0;
            tab = records[0].tab_index.0;
        }
        let tab_names = HashSet::new();
        let config = observer::Config::default().log().notify();
        let observer = observer::Observer::with_config(config);
        Self {
            tree,
            records,
            surfaces,
            nodes,
            tabs,
            surface_index,
            node_index,
            tab_index,
            surface,
            node,
            tab,
            tab_names,
            observer,
            identifier,
        }
    }

    pub fn leaf(&mut self) -> Option<&mut Tab> {
        if let Some((_, tab)) = self.tree.find_active_focused() {
            Some(tab)
        } else {
            None
        }
    }

    pub fn new_name(&mut self) -> String {
        let mut gen = names::Generator::with_naming(names::Name::Numbered);
        loop {
            let name = gen.next().expect("Could not get name from generator.");
            if !self.tab_names.contains(&name) {
                self.tab_names.insert(name.clone());
                return name;
            }
        }
    }

    pub fn new_names(&mut self, count: usize) -> Vec<String> {
        let mut names = Vec::new();
        while names.len() < count {
            names.push(self.new_name())
        }
        names
    }

    /// Increment the `surface` field.  The value of `surface` is the index for the
    /// [`surface_index::SurfaceIndex`] in the `surfaces` field that corresponds to the current surface of the
    /// [`egui_dock::DockState`] in the `tree` field.
    /// Return value indicates success or failure of operation.
    pub fn increment_surface(&mut self) -> bool {
        // Responsibility should not be on the caller to check that the [`egui_dock::DockState`] is not empty
        // if attempting to set focus on the surface, as a surface value of zero will panic on a list
        // of length zero.
        if self.surfaces.is_empty() {
            self.observer
                .warn("Cannot increment surface on an empty tree.");
            false
        } else {
            let mut success = false;
            // Check the number of available surfaces.
            let surfaces = self.surfaces.len();
            // Number of available surfaces exceeds current index, advance to next.
            // A vector of length one will have index zero, so add one to the index.
            if surfaces > self.surface + 1 {
                // Move to the next available surface.
                self.observer.success("Incrementing surface index.");
                self.surface += 1;
                success = true;
            // List is length one and cannot be incremented.
            } else if surfaces == 1 {
                // Report back to the user if they are barking up the wrong tree and trying to
                // increment a list of length 1.
                self.observer.warn(&format!(
                    "Only one surface in tree. Already on current surface {}.",
                    self.surface
                ));
            // Index is at end of list and must wrap back to beginning.
            } else {
                self.observer.success("Wrapping surface index.");
                self.surface = 0;
                success = true;
            }
            if success {
                self.observer
                    .success(&format!("Surface index set to {}", self.surface));
            }
            success
        }
    }

    /// Decrement the `surface` field.  The value of `surface` is the index for the
    /// [`surface_index::SurfaceIndex`] in the `surfaces` field that corresponds to the current surface of the
    /// [`egui_dock::DockState`] in the `tree` field.
    /// Return value indicates success or failure of operation.
    pub fn decrement_surface(&mut self) -> bool {
        // Responsibility should not be on the caller to check that the [`egui_dock::DockState`] is not empty
        // if attempting to set focus on the surface, as a surface value of zero will panic on a list
        // of length zero.
        if self.surfaces.is_empty() {
            self.observer
                .warn("Cannot decrement surface on an empty tree.");
            false
        } else {
            let mut success = false;
            // Check the number of available surfaces.
            let surfaces = self.surfaces.len();
            if surfaces == 1 && self.surface == 0 {
                // Report back to the user if they are barking up the wrong tree and trying to
                // decrement a list of length 1.
                self.observer.info(&format!(
                    "Only one surface in tree. Already on current surface {}.",
                    self.surface
                ));
            } else if self.surface == 0 {
                // Wrap back to the end if at the beginning.
                self.observer.success("Wrapping surface index.");
                // A vector of length one will have index zero, so subtract one from length.
                self.surface = surfaces - 1;
                success = true;
            } else {
                // Else decrement the surface.
                self.observer.success("Decrementing surface index.");
                self.surface -= 1;
                success = true;
            }
            if success {
                self.observer
                    .success(&format!("Surface index set to {}", self.surface));
            }
            success
        }
    }

    /// Increment the `node` field.  The value of `node` is the index for the
    /// [`node_index::NodeIndex`] in the `nodes` field that corresponds to the current node of the
    /// [`egui_dock::DockState`] in the `tree` field.
    /// Checks that the node index is valid within the active surface.
    /// Fails if the surface index is invalid, or only one node is available.
    /// Return value indicates success or failure of operation.
    pub fn increment_node(&mut self) -> bool {
        // The number of available nodes is contingent on the surface.
        if let Some(surface) = self.surface_index {
            // Subset the nodes in the surface.
            let node_ids = self.records.node_ids(&surface);

            // Responsibility should not be on the caller to check that the [`egui_dock::DockState`] is not empty
            // if attempting to set focus on the node, as a node value of zero will panic on a list
            // of length zero.
            let mut success = false;
            if node_ids.is_empty() {
                self.observer
                    .warn("Cannot increment node on an empty tree.");
            // Is the current node index valid for the active surface?
            } else if let Some(current) = node_ids.iter().position(|v| *v == self.node) {
                // Check the number of available nodes.
                let node_len = node_ids.len();
                // Length of node ids exceeds current position.
                if node_len > current + 1 {
                    // Move to next node.
                    self.observer.success("Incrementing node index.");
                    self.node = node_ids[current + 1];
                    success = true;
                } else if node_len == 1 {
                    // Report back to the user if they are barking up the wrong tree and trying to
                    // increment a list of length 1.
                    self.observer.warn(&format!(
                        "Only one node in tree. Already on current node {}.",
                        self.node
                    ));
                } else {
                    // Else wrap back to the beginning.
                    self.observer.success("Wrapping node index.");
                    self.node = node_ids[0];
                    success = true;
                }
            } else {
                self.observer
                    .success("Current node not in surface, starting at first valid node.");
                self.node = node_ids[0];
                success = true;
            }
            // Node has updated if possible.
            // Return status of operation.
            success
        } else {
            // Invalid surface, return false.
            false
        }
    }

    /// Decrement the `node` field.  The value of `node` is the index for the
    /// [`node_index::NodeIndex`] in the `nodes` field that corresponds to the current node of the
    /// [`egui_dock::DockState`] in the `tree` field.
    /// Checks that the node index is valid within the active surface.
    /// Fails if the surface index is invalid, or only one node is available.
    /// Return value indicates success or failure of operation.
    pub fn decrement_node(&mut self) -> bool {
        // The number of available nodes is contingent on the surface.
        if let Some(surface) = self.surface_index {
            // Subset the nodes in the surface.
            let node_ids = self.records.node_ids(&surface);

            // Responsibility should not be on the caller to check that the [`egui_dock::DockState`] is not empty
            // if attempting to set focus on the node, as a node value of zero will panic on a list
            // of length zero.
            let mut success = false;
            if node_ids.is_empty() {
                self.observer
                    .warn("Cannot decrement node on an empty tree.");
            // Is the current node index valid for the active surface?
            } else if let Some(current) = node_ids.iter().position(|v| *v == self.node) {
                // Check the number of available nodes.
                let node_len = node_ids.len();
                // List has length of one and cannot decrement.
                if node_len == 1 {
                    // Report back to the user if they are barking up the wrong tree and trying to
                    // increment a list of length 1.
                    self.observer.warn(&format!(
                        "Only one node in tree. Already on current node {}.",
                        self.node
                    ));
                // Node is on first leaf and cannot decrement further, so wrap.
                } else if node_len == 0 {
                    // Wrap back to the end if at the beginning.
                    self.observer.success("Wrapping node index.");
                    self.node = node_ids[node_len - 1];
                    success = true;
                } else {
                    // Else decrement the node.
                    self.observer.success("Decrementing node index.");
                    self.node = node_ids[current - 1];
                    success = true;
                }
            } else {
                self.observer
                    .success("Current node not in surface, starting at first valid node.");
                self.node = node_ids[0];
                success = true;
            }
            // Node has updated if possible.
            // Return status of operation.
            success
        } else {
            // Invalid surface, return false.
            false
        }
    }

    /// Increment the `tab` field.  The value of `tab` is the index for the
    /// [`tab_index::TabIndex`] in the `tabs` field that corresponds to the current tab of the
    /// [`egui_dock::DockState`] in the `tree` field.
    /// Checks that the tab index is valid within the active node.
    /// Fails if the node index is invalid, or only one tab is available.
    /// Return value indicates success or failure of operation.
    pub fn increment_tab(&mut self) -> bool {
        // The number of available tabs is contingent on the active node.
        if let Some(node) = self.node_index {
            // Subset the tabs in the node.
            let tab_ids = self.records.tab_ids(&node);

            // Responsibility should not be on the caller to check that the [`egui_dock::DockState`] is not empty
            // if attempting to set focus on the node, as a node value of zero will panic on a list
            // of length zero.
            let mut success = false;
            if tab_ids.is_empty() {
                self.observer.warn("Cannot increment tab on an empty tree.");
            // Is the current node index valid for the active surface?
            } else if let Some(current) = tab_ids.iter().position(|v| *v == self.tab) {
                // Check the number of available nodes.
                let tab_len = tab_ids.len();
                // Length of tab ids exceeds current position.
                if tab_len > current + 1 {
                    // Move to next tab.
                    self.observer.success("Incrementing tab index.");
                    self.tab = tab_ids[current + 1];
                    success = true;
                } else if tab_len == 1 {
                    // Report back to the user if they are barking up the wrong tree and trying to
                    // increment a list of length 1.
                    self.observer.info(&format!(
                        "Only one tab in tree. Already on current tab {}.",
                        self.tab
                    ));
                } else {
                    // Else wrap back to the beginning.
                    self.observer.success("Wrapping tab index.");
                    self.tab = tab_ids[0];
                    success = true;
                }
            } else {
                self.observer
                    .success("Current tab not in surface, starting at first valid tab.");
                self.tab = tab_ids[0];
                success = true;
            }
            // Tab has updated if possible.
            // Return status of operation.
            success
        } else {
            // Invalid node, return false.
            false
        }
    }

    /// Decrement the `tab` field.  The value of `tab` is the index for the
    /// [`tab_index::TabIndex`] in the `tabs` field that corresponds to the current tab of the
    /// [`egui_dock::DockState`] in the `tree` field.
    /// Checks that the tab index is valid within the active node.
    /// Fails if the node index is invalid, or only one tab is available.
    /// Return value indicates success or failure of operation.
    pub fn decrement_tab(&mut self) -> bool {
        // The number of available tabs is contingent on the active node.
        if let Some(node) = self.node_index {
            // Subset the tabs in the node.
            let tab_ids = self.records.tab_ids(&node);

            // Responsibility should not be on the caller to check that the [`egui_dock::DockState`] is not empty
            // if attempting to set focus on the node, as a node value of zero will panic on a list
            // of length zero.
            let mut success = false;
            if tab_ids.is_empty() {
                self.observer.warn("Cannot decrement tab on an empty tree.");
            // Is the current node index valid for the active surface?
            } else if let Some(current) = tab_ids.iter().position(|v| *v == self.tab) {
                // Check the number of available nodes.
                let tab_len = tab_ids.len();
                // List is of length 1 and cannot be decremented.
                if tab_len == 1 {
                    // Report back to the user if they are barking up the wrong tree and trying to
                    // increment a list of length 1.
                    self.observer.info(&format!(
                        "Only one tab in tree. Already on current tab {}.",
                        self.tab
                    ));
                // Index is at first tab and cannot decrement, must be wrapped.
                } else if current == 0 {
                    // Wrap back to the end if at the beginning.
                    self.observer.success("Wrapping tab index.");
                    self.tab = tab_ids[tab_len - 1];
                    success = true;
                } else {
                    // Decrement the tab index.
                    self.observer.success("Decrementing tab index.");
                    self.tab = tab_ids[current - 1];
                    success = true;
                }
            } else {
                self.observer
                    .success("Current tab not in surface, starting at first valid tab.");
                self.tab = tab_ids[0];
                success = true;
            }
            // Tab has updated if possible.
            // Return status of operation.
            success
        } else {
            // Invalid node, return false.
            false
        }
    }

    /// When the value of the `surface` field is updated, then the value in the `surface_index`
    /// field needs to be updated by this method.
    /// Checks to ensure indexed calls are in bounds.
    /// Return value indicates whether the update was successful.
    pub fn update_active_surface(&mut self) -> bool {
        if self.surfaces.is_empty() || self.surfaces.len() <= self.surface {
            self.observer.warn("Surface index is out of bounds.");
            false
        } else {
            let value = self.surfaces[self.surface];
            self.observer
                .info(&format!("Setting surface index to {:?}", &value));
            self.surface_index = Some(value);
            true
        }
    }

    /// When the active node has changed, the active tab index may be invalided.
    /// Checks to see if the tab index points to valid value for the node before updating the value
    /// of the tab.  If the active tab is invalid, updates the active tab to the first available
    /// tab.  Fails if no node is active, or no tabs are available for the active node.
    /// Return value indicates success or failure of the operation.
    pub fn update_active_node(&mut self) -> bool {
        if let Some(surface) = self.surface_index {
            // Variable indicating success or failure of the update operation.
            let mut success = false;
            // Subset the nodes in the surface.
            let node_ids = self.records.node_ids(&surface);
            // Case: The active surface has no nodes in it.
            if node_ids.is_empty() {
                self.observer.warn("No available nodes in active surface.");
            // Case: The current node index value is a valid option, use it.
            } else if node_ids.contains(&self.node) {
                // What node value does the current node index point to?
                let value = self.nodes[self.node];
                self.observer.info(&format!(
                    "Node index {} is valid for active surface.",
                    self.node
                ));
                self.observer
                    .success(&format!("Node value set to {:#?}", &value));
                self.node_index = Some(value);
                success = true;
            // Case: The current node value is no longer valid, pick the first valid node.
            } else {
                self.observer.info(&format!(
                    "Node index {} is invalid for active surface.",
                    self.node
                ));
                let value = self.nodes[node_ids[0]];
                self.observer.success(&format!(
                    "Setting node to first value in index: {:#?}",
                    &value
                ));
                self.node_index = Some(value);
                success = true;
            }
            // Return value indicating status of operation.
            success
        } else {
            self.observer
                .warn("Active surface must be set to update the active node.");
            false
        }
    }

    /// When the active node has changed, the active tab index may be invalided.
    /// Checks to see if the tab index points to valid value for the node before updating the value
    /// of the tab.  If the active tab is invalid, updates the active tab to the first available
    /// tab.  Fails if no node is active, or no tabs are available for the active node.
    /// Return value indicates success or failure of the operation.
    pub fn update_active_tab(&mut self) -> bool {
        if let Some(node) = self.node_index {
            // Variable indicating success or failure of the update operation.
            let mut success = false;
            // Subset the tabs in the node.
            let tab_ids = self.records.tab_ids(&node);
            // Case: The active node has no tabs in it.
            if tab_ids.is_empty() {
                self.observer.info("No available tabs in active node.");
            // Case: The current tab index value is a valid option, use it.
            } else if tab_ids.contains(&self.tab) {
                // What tab value does the current tab index point to?
                let value = self.tabs[self.tab];
                self.observer
                    .trace(&format!("Tab index {} is valid for active node.", self.tab));
                self.observer
                    .success(&format!("Tab value set to {:#?}", &value));
                self.tab_index = Some(value);
                success = true;
            // Case: The current tab value is no longer valid, pick the first valid tab.
            } else {
                self.observer.trace(&format!(
                    "Tab index {} is invalid for active node.",
                    self.tab
                ));
                let value = self.tabs[tab_ids[0]];
                self.observer.success(&format!(
                    "Setting tab to first value in index: {:#?}",
                    &value
                ));
                self.tab_index = Some(value);
                success = true;
            }
            if success {
                // Move focus to the new tab.
                self.observer.success("Selecting new tab.");
                self.select_current_tab();
            }
            // Return value indicating status of operation.
            success
        } else {
            self.observer
                .success("Active node must be set to update the active tab.");
            false
        }
    }

    /// Advances to the next surface when one is available.
    /// Calls [`Self::increment_surface`] and [`Self::update_active_surface`].
    /// Wraps if the index is at the end and more than one surface is available.
    /// If the surface has updated, calls [`Self::update_active_node`].
    /// If the node has updated, calls [`Self::update_active_tab`].
    /// Return value indicates success or failure of the operation.
    pub fn next_surface(&mut self) -> bool {
        // User may have created new surfaces or nodes by dragging a tab.
        // Update the records before trying to increment.
        self.update_records();
        // Attempt to increment the surface index.
        if self.increment_surface() {
            // Succes indicates surface index has changed, update the value.
            if self.update_active_surface() {
                // The active surface has changed, so the active node may now be invalid, update it.
                if self.update_active_node() {
                    // The the active node has changed, so the active tab may now be invalid, update it.
                    // Failure or success of the tab to update is not relevent to the failure or
                    // success of the suface to udpate.
                    let _ = self.update_active_tab();
                }
                // Failure or success of the node to update is not relevent to the failure or
                // success of the surface to udpate.
                true
            } else {
                self.observer
                    .warn("Surface index has incremented but active surface has not updated.");
                false
            }
        } else {
            // Surface has not changed.
            false
        }
    }

    /// Retreats to the previous surface when one is available.
    /// Calls [`Self::decrement_surface`] and [`Self::update_active_surface`].
    /// Wraps if the index is zero and more than one surface is available.
    /// If the surface has updated, calls [`Self::update_active_node`].
    /// If the node has updated, calls [`Self::update_active_tab`].
    /// Return value indicates success or failure of the operation.
    pub fn previous_surface(&mut self) -> bool {
        // User may have created new surfaces or nodes by dragging a tab.
        // Update the records before trying to decrement.
        self.update_records();
        // Attempt to decrement the surface index.
        if self.decrement_surface() {
            // Succes indicates surface index has changed, update the value.
            if self.update_active_surface() {
                // The active surface has changed, so the active node may now be invalid, update it.
                if self.update_active_node() {
                    // The the active node has changed, so the active tab may now be invalid, update it.
                    // Failure or success of the tab to update is not relevent to the failure or
                    // success of the suface to udpate.
                    let _ = self.update_active_tab();
                }
                // Failure or success of the node to update is not relevent to the failure or
                // success of the surface to udpate.
                true
            } else {
                self.observer
                    .warn("Surface index has decremented but active surface has not updated.");
                false
            }
        } else {
            // Surface has not changed.
            false
        }
    }

    /// Advances to the next node when one is available.
    /// Calls [`Self::increment_node`] and [`Self::update_active_node`].
    /// Wraps if the index is at the end and more than one node is available.
    /// If the node has updated, calls [`Self::update_active_tab`].
    /// Return value indicates success or failure of the operation.
    pub fn next_node(&mut self) -> bool {
        // User may have created new surfaces or nodes by dragging a tab.
        // Update the records before trying to increment.
        self.update_records();
        // Attempt to increment the node index.
        if self.increment_node() {
            // Succes indicates node index has changed, update the value.
            if self.update_active_node() {
                // The the active node has changed, so the active tab may now be invalid, update it.
                // Failure or success of the tab to update is not relevent to the failure or
                // success of the node to udpate.
                let _ = self.update_active_tab();
                true
            } else {
                self.observer
                    .warn("Node index has incremented but active node has not updated.");
                false
            }
        } else {
            // Node has not changed.
            false
        }
    }

    /// Retreats to the previous node when one is available.
    /// Calls [`Self::decrement_node`] and [`Self::update_active_node`].
    /// Wraps if the index is zero and more than one node is available.
    /// If the node has updated, calls [`Self::update_active_tab`].
    /// Return value indicates success or failure of the operation.
    pub fn previous_node(&mut self) -> bool {
        // User may have created new surfaces or nodes by dragging a tab.
        // Update the records before trying to decrement.
        self.update_records();
        // Attempt to decrement the node index.
        if self.decrement_node() {
            // Succes indicates node index has changed, update the value.
            if self.update_active_node() {
                // The the active node has changed, so the active tab may now be invalid, update it.
                // Failure or success of the tab to update is not relevent to the failure or
                // success of the node to udpate.
                let _ = self.update_active_tab();
                true
            } else {
                self.observer
                    .warn("Node index has decremented but active node has not updated.");
                false
            }
        } else {
            // Node has not changed.
            false
        }
    }

    /// Advances to the next tab when one is available.
    /// Calls [`Self::increment_tab`] and [`Self::update_active_tab`].
    /// Wraps if the index is at the end and more than one tab is available.
    /// Return value indicates success or failure of the operation.
    pub fn next_tab(&mut self) -> bool {
        // User may have created new surfaces or nodes by dragging a tab.
        // Update the records before trying to increment.
        self.update_records();
        // Attempt to increment the tab index.
        if self.increment_tab() {
            // Succes indicates tab index has changed, update the value.
            self.update_active_tab()
        } else {
            // Tab has not changed.
            false
        }
    }

    /// Retreats to the previous tab when one is available.
    /// Calls [`Self::decrement_tab`] and [`Self::update_active_tab`].
    /// Wraps if the index is zero and more than one tab is available.
    /// Return value indicates success or failure of the operation.
    pub fn previous_tab(&mut self) -> bool {
        // User may have created new surfaces or nodes by dragging a tab.
        // Update the records before trying to decrement.
        self.update_records();
        // Attempt to decrement the tab index.
        if self.decrement_tab() {
            // Succes indicates tab index has changed, update the value.
            self.update_active_tab()
        } else {
            // Tab has not changed.
            false
        }
    }

    /// Set focus on the current node and surface identified in the `node_index` and `surface_index` fields.
    /// If focus is not set on the surface and node, [`egui_dock::DockState::set_active_tab`] will
    /// fail.
    pub fn select_node(&mut self) {
        // If the index surface and node values are present...
        if let Some(surface_index) = self.surface_index {
            if let Some(node_index) = self.node_index {
                // Set the focus on the surface and node.
                self.tree
                    .set_focused_node_and_surface((surface_index, node_index));
                self.observer.success("Active node set.");
            } else {
                self.observer.warn("Missing node index.");
            }
        } else {
            self.observer.warn("Missing surface index.");
        }
    }

    /// Bring focus to the tab identified by the fields `surface_index`, `node_index`, and `tab_index`.
    /// Wired to [`act::Dock::SelectCurrent`]. Calls [`Self::select_node`].
    pub fn select_current_tab(&mut self) {
        // If the index variables have valid values...
        if let Some(surface_index) = self.surface_index {
            if let Some(node_index) = self.node_index {
                if let Some(tab_index) = self.tab_index {
                    // Select the current node and surface.
                    self.select_node();
                    // Select the current tab.
                    self.tree
                        .set_active_tab((surface_index, node_index, tab_index));
                    self.observer.success("Active tab set.");
                } else {
                    self.observer.warn("Missing tab index.");
                }
            } else {
                self.observer.warn("Missing node index.");
            }
        } else {
            self.observer.warn("Missing surface index.");
        }
    }

    pub fn update_records(&mut self) {
        self.records = Records::from(&self.tree);
        self.surfaces = self.records.surfaces();
        self.nodes = self.records.nodes();
        self.tabs = self.records.tabs();
        self.observer.success("Records updated.");
    }

    /// If [`egui_dock::DockArea::show_add_buttons`] is set to `true` and
    /// [`egui_dock::DockArea::show_add_popup`] is set to `true`, then the variants of [`ContextMenu`] appear as options in a context menu.
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let mut added_nodes = Vec::new();
        // Here we just display the `DockState` using a `DockArea`.
        // This is where egui handles rendering and all the integrations.
        //
        // We can specify a custom `Style` for the `DockArea`, or just inherit
        // all of it from egui.

        egui_dock::DockArea::new(&mut self.tree)
            .show_add_buttons(true)
            .show_add_popup(true)
            .style(egui_dock::Style::from_egui(ui.style().as_ref()))
            .show_inside(
                ui,
                &mut TabViewer::new(&mut added_nodes, &mut self.identifier),
            );
        let update = !added_nodes.is_empty();
        let names = self.new_names(added_nodes.len());
        let mut name_iter = names.iter();

        // At this point we can inspect the TabContext and take different actions according the
        // variant of the ContextMenu.
        // Currently we do one action and do not match on the ContextMenu.
        added_nodes.drain(..).for_each(|tab_context| {
            self.tree
                .set_focused_node_and_surface((tab_context.surface, tab_context.node));
            self.tree.push_to_focused_leaf({
                players::Players::paeva()

                // let attr = paeva.attributes();
                // let table = TableView::new(*attr);
                // TabView::with_name(
                //     table,
                //     name_iter
                //         .next()
                //         .expect("Should be one name for each new tab.")
                //         .clone(),
                // )
            });
            // self.tab_index += 1;
            self.observer.success("Tab added.");
        });
        if update {
            self.update_records();
        }
        self.observer.show(ui.ctx());
    }

    pub fn run_ui(&mut self, ctx: &egui::Context) {
        // let id = ctx.viewport_id();
        // tracing::info!("Panel id: {:?}", id);
        // prints "FFFF"
        egui::CentralPanel::default().show(ctx, |ui| {
            // Unnecessary.
            ui.push_id(self.identifier.name(), |ui| {
                self.ui(ui);
            });
        });
    }

    pub fn act(&mut self, act: &act::Dock) {
        match *act {
            act::Dock::CurrentTab => self.select_current_tab(),
            act::Dock::NextTab => {
                let _ = self.next_tab();
            }
            act::Dock::PreviousTab => {
                let _ = self.previous_tab();
            }
            act::Dock::NextNode => {
                let _ = self.next_node();
            }
            act::Dock::PreviousNode => {
                let _ = self.previous_node();
            }
            act::Dock::NextSurface => {
                let _ = self.next_surface();
            }
            act::Dock::PreviousSurface => {
                let _ = self.previous_surface();
            }
            act::Dock::InspectRecords => {
                self.observer.info(&format!("{:#?}", self.records));
            }
            act::Dock::Be => tracing::trace!("Taking no action."),
        }
    }
}

impl Default for TabState {
    fn default() -> Self {
        Self::new()
    }
}
