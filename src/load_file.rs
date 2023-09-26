use serde::Deserialize;
use sfml::{
    graphics::{CircleShape, Drawable, Transformable},
    system::Vector2f,
};

use crate::tree::GameTree;

#[derive(Debug, Deserialize)]
struct TomlScene {
    nodes: Vec<TomlNode>,
}

#[derive(Debug, Deserialize)]
struct TomlNode {
    path: String,
    position: Option<f32>,
}

fn parse_toml() -> TomlScene {
    let toml_str = r#"
            [[nodes]]
            path = "/render/help"
            position = 500.0

            [[nodes]]
            path = "/render/shape1"
        "#;

    toml::from_str(toml_str).unwrap()
}

pub fn load_tree_from_file() -> GameTree<Box<dyn Drawable>> {
    let scene = parse_toml();
    println!("{:#?}", scene);

    let mut tree = GameTree::<Box<dyn Drawable>>::from_root(
        "/render".to_string(),
        Box::new(CircleShape::new(50., 30)) as Box<dyn Drawable>
    );
    for node in scene.nodes {
        let mut shape = CircleShape::new(50., 30);
        match node.position {
            Some(pos) => {
                shape.set_position(Vector2f::new(pos, pos));
            }
            None => {}
        }
        tree.add_node(node.path, Box::new(shape));
    }
    tree
}
