use serde::Deserialize;
use sfml::{graphics::{CircleShape, Drawable, Transformable}, system::Vector2f};
use slab_tree::{Tree, TreeBuilder};

use crate::tree::NamedNode;

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
            path = "/"
            position = 500.0

            [[nodes]]
            path = "/shape1"
        "#;

    toml::from_str(toml_str).unwrap()
}

pub fn load_tree_from_file() -> Tree<NamedNode<dyn Drawable>> {
    let scene = parse_toml();
    println!("{:#?}", scene);

    let mut tree = TreeBuilder::<NamedNode<dyn Drawable>>::new()
        .with_root(NamedNode::new(String::from("/"), CircleShape::new(50., 30)))
        .build();
    let mut root = tree
        .get_mut(tree.root_id().expect("NO ROOT PANIC"))
        .unwrap();
    for node in scene.nodes {
        let mut shape = CircleShape::new(50., 30);
        match node.position {
            Some(pos) => {
                shape.set_position(Vector2f::new(pos, pos));

            }, None => {} 
        }
        root.append(NamedNode::new(node.path, shape));
    }
    tree as Tree<NamedNode<(dyn Drawable + 'static)>>
}
