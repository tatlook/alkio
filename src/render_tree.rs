use sfml::graphics::{Drawable, RenderTarget, CircleShape};
use slab_tree::NodeRef;

use crate::tree::GameTree;

pub enum RenderNode<'a> {
    CircleShape(CircleShape<'a>),
}

impl<'a> RenderNode<'a> {
/*     fn as_transformable(&self) -> &dyn Transformable {
        match self {
            RenderNode::CircleShape(shape) => shape,
        }
    }
 */
    fn as_drawable(&self) -> &dyn Drawable {
        match self {
            RenderNode::CircleShape(shape) => shape,
        }
    }
}

pub type RenderTree<'a> = GameTree<RenderNode<'a>>;

fn draw_tree_nodes_to(target: &mut dyn RenderTarget, node: NodeRef<'_, RenderNode>) {
    for child in node.children() {
        draw_tree_nodes_to(target, child);
    }
    match node.data() {
        RenderNode::CircleShape(shape) => {
            target.draw(shape)
        },
    }
}

pub fn draw_tree_to(target: &mut dyn RenderTarget, tree: &RenderTree) {
    let tree = tree.tree();
    let root = tree
        .get(tree.root_id().expect("This tree has no root"))
        .unwrap();
    draw_tree_nodes_to(target, root);
    RenderNode::CircleShape(sfml::graphics::CircleShape::new(2., 23));
}
