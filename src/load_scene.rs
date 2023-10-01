use serde::Deserialize;
use sfml::{
    graphics::{CircleShape, Transformable},
    system::Vector2f,
};

use crate::{logic_tree::LogicTree, render_tree::{RenderTree, RenderNode}, scene::Scene};

#[derive(Debug, Deserialize)]
struct TomlScene {
    render_nodes: Vec<TomlRenderNode>,
    logic_nodes: Vec<TomlLogicNode>,
}

#[derive(Debug, Deserialize)]
struct TomlRenderNode {
    path: String,
    position: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct TomlLogicNode {
    path: String,
    script: String,
}

fn parse_toml() -> TomlScene {
    let toml_str = r#"
            [[render_nodes]]
            path = "/render/help"
            position = 500.0

            [[render_nodes]]
            path = "/render/shape1"

            [[logic_nodes]]
            path = "/logic/hellowod" 
            script = "print(\"yeeie\")"
        "#;

    toml::from_str(toml_str).unwrap()
}

fn load_render_tree<'a>(scene: &TomlScene) -> RenderTree<'a> {
    let mut tree = RenderTree::from_root(
        "/render".to_string(),
        RenderNode::CircleShape(CircleShape::new(50., 30)),
    );
    for node in &scene.render_nodes {
        let mut shape = CircleShape::new(50., 30);
        if let Some(pos) = node.position {
            shape.set_position(Vector2f::new(pos, pos));
        }
        tree.add_node(node.path.to_owned(), RenderNode::CircleShape(shape));
    }
    tree
}

fn load_logic_tree(scene: &TomlScene) -> LogicTree {
    let mut tree = LogicTree::from_root("/logic".to_string(), "print(0)".to_string());
    for node in &scene.logic_nodes {
        tree.add_node(node.path.to_owned(), node.script.to_owned());
    }
    tree
}

pub fn load_scene<'a>() -> Scene<'a> {
    let scene = parse_toml();
    println!("{:#?}", scene);

    let render_tree = load_render_tree(&scene);
    let logic_tree = load_logic_tree(&scene);
    Scene::with_every_tree(render_tree, logic_tree)
}
