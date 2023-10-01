use sfml::graphics::{Drawable, Transformable, glsl::Vec2, CircleShape};
use slab_tree::NodeRef;

use crate::{
    scene::{PeerTableElement, Scene},
    tree::GameTree, render_tree::RenderNode,
};

pub type LogicTree = GameTree<String>;
use std::rc::Rc;

use rhai::{Engine, EvalAltResult};

pub fn run_rhai<'a>(
    script: &str,
    peers: PeerTableElement,
    scene: Rc<Scene<'a>>,
) -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();
    engine.register_fn("set_position", move |x: f32, y: f32| -> () {
        let mut scene  = scene;
        let mut render: &RenderNode<'a> = scene
            .render_tree()
            .tree()
            .get(peers.render_node.to_owned())
            .unwrap()
            .data();
        match render {
            RenderNode::CircleShape(ref mut shape) => shape.set_position(Vec2::new(x, y)),
        }
    });
    engine.run(script)?;

    Ok(())
}

fn run_logics_children<'a>(node: NodeRef<'_, String>, scene: Rc<Scene<'a>>) {
    for child in node.children() {
        run_logics_children(child, scene.clone());
    }
    let peers = scene
        .as_ref()
        .peer_table()
        .find_peers_of_logic(node.node_id())
        .unwrap()
        .clone();
    run_rhai(node.data(), peers, scene.clone()).unwrap();
}

pub fn run_logics<'a>(scene: Rc<Scene<'a>>) {
    let tree = &mut scene.logic_tree().tree();

    let root = tree
        .get(tree.root_id().expect("This tree has no root").to_owned())
        .unwrap();
    run_logics_children(root, scene.clone());
}
