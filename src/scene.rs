use std::fmt::Display;

use slab_tree::NodeId;

use crate::{logic_tree::LogicTree, render_tree::RenderTree};

pub struct Scene<'a> {
    render_tree: RenderTree<'a>,
    logic_tree: LogicTree,
    peer_table: PeerTable,
}

pub struct PeerTableElement {
    pub render_node: NodeId,
    pub logic_node: NodeId,
}

impl Clone for PeerTableElement {
    fn clone(&self) -> Self {
        Self { render_node: self.render_node.clone(), logic_node: self.logic_node.clone() }
    }
}

/// Table of node peers.
/// A node may have peer in another tree.
pub struct PeerTable {
    table: Vec<PeerTableElement>,
}

impl PeerTable {
    fn new() -> Self {
        PeerTable { table: vec![] }
    }

    pub fn find_peers_of_logic(&self, id: NodeId) -> Option<&PeerTableElement> {
        for elem in &self.table {
            if elem.logic_node == id {
                return Some(&elem);
            }
        }
        None
    }
}

impl<'a> Scene<'a> {
    pub fn with_every_tree(render_tree: RenderTree<'a>, logic_tree: LogicTree) -> Self {
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

    pub fn peer_table(&self) -> &PeerTable {
        &self.peer_table
    }
}

impl<'a> Display for Scene<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.render_tree, self.logic_tree)
    }
}
