use rhai::Engine;
use serde::Deserialize;
use sfml::{
    graphics::{CircleShape, Transformable},
    system::Vector2f,
};
use std::{rc::Rc, cell::RefCell, fs::{self, File}};

use crate::{render_tree::{RenderTree, RenderNode}, scene::Scene, tree::GameElement, script::Script};

#[derive(Debug, Deserialize)]
struct TomlScene {
    render_nodes: Vec<TomlRenderNode>,
}

#[derive(Debug, Deserialize)]
struct TomlRenderNode {
    path: String,
    position: Option<f32>,
    script: Option<String>,
}

fn parse_toml() -> TomlScene {
    let mut file_path = std::env::current_exe().expect("Cannot get exe path");
    file_path.pop();
    file_path.push("example");
    file_path.push("scene.toml");
    println!("Path {:?}", file_path.as_path());
    let scene = fs::read_to_string(file_path).expect("There is no file");
    toml::from_str(scene.as_str()).unwrap()
}

fn load_render_tree<'a>(scene: &TomlScene) -> RenderTree<'a> {
    let render_rhai_engine = Rc::new(RefCell::new(Engine::new()));
    let mut tree = RenderTree::from_root(
        "/render".to_string(),
        GameElement::from(RenderNode::CircleShape(CircleShape::new(50., 30))),
    );
    for node in &scene.render_nodes {
        let mut shape = CircleShape::new(50., 30);
        if let Some(pos) = node.position {
            shape.set_position(Vector2f::new(pos, pos));
        }
        let mut elem = GameElement::from(RenderNode::CircleShape(shape));
        if let Some(script) = &node.script {
            match Script::compile(render_rhai_engine.clone(), script.to_owned()) {
                Ok(script) => elem.set_script(script),
                Err(e) => eprintln!("Failed to compile script: {}", e)
            }
        }
        tree.add_node(node.path.to_owned(), elem);
    }
    tree
}

pub fn load_scene<'a>() -> Scene<'a> {
    let scene = parse_toml();
    println!("{:#?}", scene);
    let render_tree = load_render_tree(&scene);
    Scene::with_every_tree(render_tree)
}
