use std::fmt::Display;


use std::cell::RefCell;
use std::rc::Rc;
use slab_tree::NodeId;
use crate::render_tree::RenderTree;

pub struct Scene<'a> {
    render_tree: Rc<RefCell<RenderTree<'a>>>,
    peer_table: PeerTable,
}

#[derive(Clone, Copy)]
pub struct PeerTableElement {
    pub render_node: NodeId,

    // TODO: sound_node, collison_node, etc.
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
}

impl<'a> Scene<'a> {
    pub fn with_every_tree(render_tree: Rc<RefCell<RenderTree<'a>>>) -> Self {
        Scene {
            render_tree,
            peer_table: PeerTable::new(),
        }
    }

    pub fn render_tree(&self) -> Rc<RefCell<RenderTree<'a>>> {
        self.render_tree.clone()
    }

    pub fn peer_table(&self) -> &PeerTable {
        &self.peer_table
    }
}

impl<'a> Display for Scene<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render_tree.borrow())
    }
}
