use sfml::graphics::{Drawable, RenderTarget};
use slab_tree::NodeRef;

use crate::tree::GameTree;

pub type RenderTree = GameTree<Box<dyn Drawable>>;

fn draw_tree_nodes_to(target: &mut dyn RenderTarget, node: NodeRef<'_, Box<dyn Drawable>>) {
    for child in node.children() {
        draw_tree_nodes_to(target, child);
    }
    target.draw(node.data().as_ref())
}

pub fn draw_tree_to(target: &mut dyn RenderTarget, tree: &RenderTree) {
    let tree = tree.tree();
    let root = tree
        .get(tree.root_id().expect("This tree has no root"))
        .unwrap();
    draw_tree_nodes_to(target, root);
}
