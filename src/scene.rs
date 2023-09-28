use std::fmt::Display;

use slab_tree::NodeId;

use crate::{logic_tree::LogicTree, render_tree::RenderTree};

pub struct Scene {
    render_tree: RenderTree,
    logic_tree: LogicTree,
    peer_table: PeerTable,
}

struct PeerTableElement {
    render_node: NodeId,
    logic_node: NodeId,
}

/// Table of node peers.
/// A node may have peer in another tree.
struct PeerTable {
    table: Vec<PeerTableElement>,
}

impl PeerTable {
    fn new() -> Self {
        PeerTable { table: vec![] }
    }

    fn find_peers_of_render(&self, id: NodeId) -> Option<&PeerTableElement> {
        for elem in &self.table {
            if elem.logic_node == id {
                return Some(&elem);
            }
        }
        None
    }
}

impl Scene {
    pub fn with_every_tree(render_tree: RenderTree, logic_tree: LogicTree) -> Self {
        Scene {
            render_tree,
            logic_tree,
            peer_table: PeerTable::new(),
        }
    }

    pub fn render_tree(&self) -> &RenderTree {
        &self.render_tree
    }

    pub fn logic_tree(&self) -> &LogicTree {
        &self.logic_tree
    }
}

impl Display for Scene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.render_tree, self.logic_tree)
    }
}
