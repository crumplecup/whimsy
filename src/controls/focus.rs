use egui::Id;
use std::collections::HashMap;
use tracing::info;
use uuid::Uuid;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Tree {
    pub flags: HashMap<Uuid, bool>,
    pub leaves: HashMap<Uuid, Leaf>,
    pub nodes: HashMap<Uuid, Node>,
    pub windows: Vec<Uuid>,
    pub select: Option<Id>,
    node_index: usize,
    window_index: usize,
}

impl Tree {
    pub fn new() -> Self {
        let flags = HashMap::new();
        let leaves = HashMap::new();
        let nodes = HashMap::new();
        let windows = Vec::new();
        Self {
            flags,
            leaves,
            nodes,
            windows,
            ..Default::default()
        }
    }

    pub fn leaf(&mut self, id: Id) -> Uuid {
        Leaf::from_id(id, self)
    }

    pub fn node(&mut self) -> Uuid {
        Node::with_tree(self)
    }

    pub fn window(&mut self) -> Uuid {
        let id = Uuid::new_v4();
        self.windows.push(id);
        self.flags.insert(id, false);
        id
    }

    pub fn select(&mut self, id: Id) {
        self.select = Some(id);
    }

    pub fn selected(&self) -> Option<Id> {
        self.select
    }

    pub fn clear_selected(&mut self) {
        self.select = None;
    }

    pub fn with_leaf(&mut self, leaf: Uuid, node: Uuid) {
        Node::with_leaf(leaf, node, self);
    }

    pub fn with_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    pub fn with_window(&mut self, node: Uuid, window: Uuid) {
        let node = self.nodes.get_mut(&node);
        if let Some(n) = node {
            n.with_window(window);
        }
    }

    pub fn get_window(&self, window: Uuid) -> Vec<Uuid> {
        self.nodes
            .iter()
            .map(|(k, v)| (k, v))
            .filter(|(_, v)| v.window == Some(window))
            .map(|(k, _)| k.clone())
            .collect::<Vec<Uuid>>()
    }

    pub fn current_window(&self) -> Uuid {
        self.windows[self.window_index]
    }

    pub fn next_window(&mut self) -> Uuid {
        if self.window_index + 1 > self.windows.len() - 1 {
            self.window_index = 0;
        } else {
            self.window_index += 1;
        }
        self.windows[self.window_index]
    }

    pub fn previous_window(&mut self) -> Uuid {
        if self.window_index == 0 {
            self.window_index = self.windows.len() - 1;
        } else {
            self.window_index -= 1;
        }
        self.windows[self.window_index]
    }

    pub fn current_node(&self) -> Uuid {
        let id = self.current_window();
        let nodes = self.get_window(id);
        nodes[self.node_index]
    }

    pub fn next_node(&mut self) -> Uuid {
        let id = self.current_window();
        let nodes = self.get_window(id);
        if self.node_index == (nodes.len() - 1) {
            self.node_index = 0;
        } else {
            self.node_index += 1;
        }
        nodes[self.node_index]
    }

    pub fn previous_node(&mut self) -> Uuid {
        let id = self.current_window();
        let nodes = self.get_window(id);
        if self.node_index == 0 {
            self.node_index = nodes.len() - 1;
        } else {
            self.node_index -= 1;
        }
        nodes[self.node_index]
    }

    pub fn next_node_inner(&mut self) -> Option<Uuid> {
        if let Some(node) = self.nodes.get_mut(&self.current_node()) {
            Some(node.next_node())
        } else {
            None
        }
    }

    pub fn previous_node_inner(&mut self) -> Option<Uuid> {
        if let Some(node) = self.nodes.get_mut(&self.current_node()) {
            Some(node.previous_node())
        } else {
            None
        }
    }

    pub fn current_leaf(&self) -> Option<Uuid> {
        if let Some(node) = self.nodes.get(&self.current_node()) {
            Some(node.current_leaf())
        } else {
            None
        }
    }

    pub fn next_leaf(&mut self) -> Option<Uuid> {
        if let Some(node) = self.nodes.get_mut(&self.current_node()) {
            Some(node.next_leaf())
        } else {
            None
        }
    }

    pub fn previous_leaf(&mut self) -> Option<Uuid> {
        if let Some(node) = self.nodes.get_mut(&self.current_node()) {
            Some(node.previous_leaf())
        } else {
            None
        }
    }

    pub fn select_current(&mut self) {
        if let Some(leaf_id) = self.current_leaf() {
            if let Some(leaf) = self.leaves.get(&leaf_id) {
                tracing::info!("Setting select to {:#?}", leaf.id);
                self.select = Some(leaf.id);
            }
        }
    }

    pub fn select_next(&mut self) {
        if let Some(leaf_id) = self.next_leaf() {
            if let Some(leaf) = self.leaves.get(&leaf_id) {
                tracing::info!("Setting select to {:#?}", leaf.id);
                self.select = Some(leaf.id);
            }
        }
    }

    pub fn select_previous(&mut self) {
        if let Some(leaf_id) = self.previous_leaf() {
            if let Some(leaf) = self.leaves.get(&leaf_id) {
                tracing::info!("Setting select to {:#?}", leaf.id);
                self.select = Some(leaf.id);
            }
        }
    }

    pub fn select_next_node(&mut self) {
        let _ = self.next_node();
        self.select_current();
    }

    pub fn select_previous_node(&mut self) {
        let _ = self.previous_node();
        self.select_current();
    }

    pub fn select_next_window(&mut self) {
        let _ = self.next_window();
        self.select_current();
    }

    pub fn select_previous_window(&mut self) {
        let _ = self.previous_window();
        self.select_current();
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Node {
    pub id: Uuid,
    pub parent: Option<Uuid>,
    pub nodes: Vec<Uuid>,
    pub leaves: Vec<Uuid>,
    pub window: Option<Uuid>,
    node_index: usize,
    leaf_index: usize,
}

impl Node {
    pub fn with_tree(tree: &mut Tree) -> Uuid {
        let id = Uuid::new_v4();
        let parent = None;
        let node = Self {
            id,
            parent,
            ..Default::default()
        };
        tree.nodes.insert(id, node);
        id
    }

    pub fn with_leaf(node_id: Uuid, leaf_id: Uuid, tree: &mut Tree) {
        let leaf = tree.leaves.get_mut(&leaf_id);
        let node = tree.nodes.get_mut(&node_id);
        if let Some(l) = leaf {
            l.parent = Some(node_id);
            if let Some(n) = node {
                n.leaves.push(l.leaf_id);
            }
        }
    }

    pub fn with_branch(&mut self, node: &mut Node) {
        node.parent = Some(self.id.to_owned());
        self.nodes.push(node.id);
    }

    pub fn with_window(&mut self, window: Uuid) {
        self.window = Some(window);
    }

    pub fn current_leaf(&self) -> Uuid {
        let id = self.leaves[self.leaf_index];
        info!("Current leaf is {}", id);
        id
    }

    pub fn next_leaf(&mut self) -> Uuid {
        if self.leaf_index == (self.leaves.len() - 1) {
            self.leaf_index = 0;
        } else {
            self.leaf_index += 1;
        }
        info!("Leaf index is {}", self.leaf_index);
        let id = self.leaves[self.leaf_index];
        info!("Next leaf is {}", id);
        id
    }

    pub fn previous_leaf(&mut self) -> Uuid {
        if self.leaf_index == 0 {
            self.leaf_index = self.leaves.len() - 1;
        } else {
            self.leaf_index -= 1;
        }
        let id = self.leaves[self.leaf_index];
        info!("Previous leaf is {}", id);
        id
    }

    pub fn current_node(&self) -> Uuid {
        let id = self.nodes[self.node_index];
        info!("Current node is {}", id);
        id
    }

    pub fn next_node(&mut self) -> Uuid {
        if self.node_index + 1 > self.nodes.len() - 1 {
            self.node_index += 1;
        } else {
            self.node_index = 0;
        }
        let id = self.nodes[self.node_index];
        info!("Next node is {}", id);
        id
    }

    pub fn previous_node(&mut self) -> Uuid {
        if self.node_index == 0 {
            self.node_index = self.nodes.len() - 1;
        } else {
            self.node_index -= 1;
        }
        let id = self.nodes[self.node_index];
        info!("Previous node is {}", id);
        id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Leaf {
    pub id: Id,
    pub leaf_id: Uuid,
    pub parent: Option<Uuid>,
}

impl Leaf {
    pub fn from_id(id: Id, tree: &mut Tree) -> Uuid {
        let leaf_id = Uuid::new_v4();
        let leaf = Self {
            id,
            leaf_id,
            parent: None,
        };
        tree.leaves.insert(leaf_id, leaf);
        leaf_id
    }
}
