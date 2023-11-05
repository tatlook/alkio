use sfml::graphics::{Drawable, RenderTarget, CircleShape, Sprite};
use slab_tree::NodeRef;

use crate::tree::{GameTree, GameElement};

pub enum RenderNode<'a> {
    CircleShape(CircleShape<'a>),
    Sprite(Sprite<'a>),
}

impl<'a> RenderNode<'a> {
/*     fn as_transformable(&self) -> &dyn Transformable {
        match self {
            RenderNode::CircleShape(ref shape) => shape,
            RenderNode::Sprite(ref shape) => shape,
        }
    }
*/
    fn as_drawable(&self) -> &dyn Drawable {
        match self {
            RenderNode::CircleShape(shape) => shape,
            RenderNode::Sprite(shape) => shape,
        }
    }
}

pub type RenderTree<'a> = GameTree<RenderNode<'a>>;

fn draw_tree_nodes_to(target: &mut dyn RenderTarget, node: NodeRef<'_, GameElement<RenderNode>>) {
    for child in node.children() {
        draw_tree_nodes_to(target, child);
    }
    target.draw(node.data().as_drawable())
}

pub fn draw_tree_to(target: &mut dyn RenderTarget, tree: &RenderTree) {
    let root = tree
        .get(tree.root_id().expect("This tree has no root"))
        .unwrap();
    draw_tree_nodes_to(target, root);
}
