use sfml::graphics::{Drawable, RenderTarget};
use slab_tree::{NodeRef, Tree};
use crate::tree::NamedNode;

fn draw_tree_nodes_to(target: &mut dyn RenderTarget, node: NodeRef<'_, NamedNode<dyn Drawable>>) {
    for child in node.children() {
        draw_tree_nodes_to(target, child);
    }
    target.draw(node.data().get_box().as_ref())
}

pub fn draw_tree_to(target: &mut dyn RenderTarget, tree: &Tree<NamedNode<dyn Drawable>>) {
    let root = tree
        .get(tree.root_id().expect("This tree has no root"))
        .unwrap();
    draw_tree_nodes_to(target, root);
}
