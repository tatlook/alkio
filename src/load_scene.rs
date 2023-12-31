use rhai::{Engine, Scope};
use serde::Deserialize;
use sfml::{
    graphics::{CircleShape, IntRect, RcSprite, RcTexture, Transformable},
    system::Vector2f,
};
use slab_tree::NodeId;
use std::{cell::RefCell, fs, rc::Rc};

use crate::{
    render_tree::{CircleShapeApi, RenderNode, RenderTree},
    scene::Scene,
    script::Script,
    tree::GameElement,
};

#[derive(Debug, Deserialize)]
struct TomlScene {
    render_nodes: Vec<TomlRenderNode>,
}

#[derive(Debug, Deserialize)]
struct TomlRenderNode {
    path: String,
    position: Option<f32>,
    script: Option<String>,
    variation: TomlRenderNodeVariation,
}

#[derive(Debug, Deserialize)]
enum TomlRenderNodeVariation {
    CircleShape { radius: f32 },
    Sprite { texture_path: String },
}

fn redirect_protocol(path: &String) -> String {
    if &path[0..5] == "res://" {
        String::from(&path[6..path.len()])
    } else {
        path.clone()
    }
}

fn parse_toml() -> TomlScene {
    let mut file_path = std::env::current_exe().expect("Cannot get exe path");
    file_path.pop();
    file_path.push("example");
    file_path.push("scene.toml");
    let scene = fs::read_to_string(file_path).expect("There is no file");
    toml::from_str(scene.as_str()).expect("Toml error")
}

#[derive(Clone, Copy)]
struct Aobj {}

impl Aobj {
    pub fn hello(&mut self) {
        print!("Good!");
    }
}

fn load_render_tree<'a>(scene: &TomlScene) -> Rc<RefCell<RenderTree<'a>>> {
    let render_rhai_engine = Rc::new(RefCell::new(Engine::new()));
    render_rhai_engine
        .borrow_mut()
        .register_type_with_name::<Aobj>("Aobj")
        .register_fn("hello", Aobj::hello);
    let tree = RenderTree::from_root(
        "/render".to_string(),
        GameElement::from(RenderNode::CircleShape {
            shape: CircleShape::new(50., 30),
        }),
    );
    let tree = Rc::new(RefCell::new(tree));
    let render_tree_scope = Scope::new();
    for node in &scene.render_nodes {
        let render_node = match &node.variation {
            TomlRenderNodeVariation::CircleShape { radius } => {
                let mut shape = CircleShape::new(radius.to_owned(), 30);
                if let Some(pos) = node.position {
                    shape.set_position(Vector2f::new(pos, pos));
                }
                RenderNode::CircleShape { shape }
            }
            TomlRenderNodeVariation::Sprite { texture_path } => load_sprite(texture_path),
        };
        let elem = GameElement::from(render_node);

        let id = tree.borrow_mut().add_node(node.path.to_owned(), elem);

        let borrow_mut = &mut tree.borrow_mut();
        let borrow_mut = &mut borrow_mut.tree_mut().get_mut(id).unwrap();
        let elem = borrow_mut.data();

        if let Some(script) = &node.script {
            load_script(
                &render_tree_scope,
                elem,
                id,
                tree.clone(),
                render_rhai_engine.clone(),
                script.to_owned(),
            );
        }
    }
    tree
}

fn load_script<'a>(
    render_tree_scope: &Scope<'_>,
    elem: &mut GameElement<'a, RenderNode<'a>>,
    id: NodeId,
    tree: Rc<RefCell<RenderTree<'a>>>,
    render_rhai_engine: Rc<RefCell<Engine>>,
    script: String,
) {
    let mut scope = Scope::new();
    match elem.element_mut() {
        RenderNode::CircleShape { shape: _ } => {
            let api = CircleShapeApi::new(id, tree.clone());
            scope.push("self", api);
        }
        RenderNode::Sprite { shape: _ } => {}
    }
    let o = Aobj {};
    scope.push("o", o);
    match Script::compile(render_rhai_engine.clone(), scope, script) {
        Ok(script) => elem.set_script(script),
        Err(e) => eprintln!("Failed to compile script: {}", e),
    }
}

fn load_sprite<'a>(texture_path: &String) -> RenderNode<'a> {
    let mut sprite = RcSprite::new();
    let mut texture = RcTexture::new().expect("Can't create texture");
    texture
        .load_from_file(&texture_path, IntRect::new(50, 50, 100, 100))
        .expect("File not found");
    sprite.set_texture(&texture, true);
    RenderNode::Sprite { shape: sprite }
}

/// Load scene from example file.
pub fn load_scene<'a>() -> Scene<'a> {
    let scene = parse_toml();
    let render_tree = load_render_tree(&scene);
    Scene::with_every_tree(render_tree)
}
