use sfml::graphics::{CircleShape, Drawable, RcSprite, RcTexture, RenderTarget};
use slab_tree::NodeRef;

use crate::tree::{GameElement, GameTree};

pub struct CircleShapeApi {}

pub struct SpriteApi {}

pub enum RenderNode<'a> {
    CircleShape {
        shape: CircleShape<'a>,
    },
    Sprite {
        shape: RcSprite,
        texture: RcTexture,
    },
}

impl<'a> RenderNode<'a> {

    fn as_drawable(&self) -> &dyn Drawable {
        match self {
            RenderNode::CircleShape{shape} => shape,
            RenderNode::Sprite{shape, texture: _} => shape,
        }
    }
}

pub type RenderTree<'a> = GameTree<'a, RenderNode<'a>>;

fn draw_tree_nodes_to(target: &mut dyn RenderTarget, node: NodeRef<'_, GameElement<RenderNode>>) {
    for child in node.children() {
        draw_tree_nodes_to(target, child);
    }
    target.draw(node.data().element().as_drawable())
}

pub fn draw_tree_to(target: &mut dyn RenderTarget, tree: &RenderTree) {
    let root = tree.tree()
        .get(tree.tree().root_id().expect("This tree has no root"))
        .unwrap();
    draw_tree_nodes_to(target, root);
}
