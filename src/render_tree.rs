use std::cell::RefCell;
use std::rc::Rc;

use sfml::{graphics::{CircleShape, Drawable, RcSprite, RenderTarget}, system::Vector2f};
use slab_tree::{NodeRef, NodeId};

use crate::tree::{GameTree, GameElement};

#[derive(Clone)]
pub struct CircleShapeApi<'a> {
    id: NodeId,
    tree: Rc<RefCell<RenderTree<'a>>>,
}

impl<'a> CircleShapeApi<'a> {
    pub fn new(id: NodeId, tree: Rc<RefCell<RenderTree<'a>>>) -> Self {
        CircleShapeApi { id, tree }
    }

    pub fn set_position(&mut self, _position: Vector2f) {
        self.tree.borrow_mut().tree_mut().get_mut(self.id);
        todo!()
    }
}

pub struct SpriteApi {}

#[derive(Clone)]
pub enum RenderNode<'a> {
    CircleShape {
        shape: CircleShape<'a>,
    },
    Sprite {
        shape: RcSprite,
    },
}

impl<'a> RenderNode<'a> {

    fn as_drawable(&self) -> &dyn Drawable {
        match self {
            RenderNode::CircleShape{shape} => shape,
            RenderNode::Sprite{shape} => shape,
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
