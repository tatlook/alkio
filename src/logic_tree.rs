use rand::Rng;
use sfml::graphics::{glsl::Vec2, Transformable};
use slab_tree::NodeRef;

use crate::{
    render_tree::RenderNode,
    scene::{PeerTableElement, SceneRef},
    tree::GameTree,
};

pub type LogicTree = GameTree<String>;

use rhai::{Engine, EvalAltResult};
pub fn run_rhai<'a>(
    script: &str,
    peers: PeerTableElement,
    scene: SceneRef<'a>,
) -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();
    let scene = &scene;
    //engine.register_fn("set_position", move |x: f32, y: f32| -> () {
        //let mut scene = scene.lock().unwrap();
        // let mut render = scene
        //     .render_tree_mut()
        //     .tree_mut()
        //     .get_mut(peers.render_node.to_owned())
        //     .unwrap()
        //     .data();
        // match render {
        //     RenderNode::CircleShape(ref mut shape) => shape.set_position(Vec2::new(x, y)),
        //     RenderNode::Sprite(ref mut shape) => shape.set_position(Vec2::new(x, y)),
        // }
        //});

    engine.run(script)?;


    let mut rng = rand::thread_rng();
    let position = Vec2::new(rng.gen_range(1.0..100.), rng.gen_range(1.0..100.));
    print!("{:?}", position);
    let mut scene = scene.borrow_mut();
    let mut render = scene
        .render_tree_mut()
        .tree_mut()
        .get_mut(peers.render_node.to_owned())
        .unwrap();
    let render = render.data();
    match render {
        RenderNode::CircleShape(ref mut shape) => shape.set_position(position),
        RenderNode::Sprite(ref mut shape) => shape.set_position(position),
    }

    Ok(())
}

fn run_logics_children(node: NodeRef<String>, scene: SceneRef) {

    for child in node.children() {
        run_logics_children(child, scene.clone());
    }

    if let Some(peers) = scene.borrow().peer_table().find_peers_of_logic(node.node_id()) {
        if let Err(err) = run_rhai(node.data(), peers.clone(), scene.clone()) {
            println!("Error occurred: {:?}", err);
        }
    }
}

pub fn run_logics<'a>(scene: SceneRef<'a>) {
    let borrow = scene.borrow();
    let tree = borrow.logic_tree().tree();

    let root = tree
        .get(tree.root_id().expect("This tree has no root").to_owned())
        .unwrap();
    run_logics_children(root, scene.clone());
}
