mod load_scene;
mod render_tree;
mod scene;
mod script;
mod tree;

use rand::Rng;
use render_tree::{draw_tree_to, RenderNode, RenderTree};
use script::run_tree_script;
use sfml::{
    graphics::{Color, RenderTarget, RenderWindow, Transformable},
    window::{Event, Style}, system::Vector2f,
};
use slab_tree::NodeId;
use std::{cell::RefCell, rc::Rc};


fn main() {
    println!("{:?}", std::env::current_dir().unwrap());
    let scene = Rc::new(RefCell::new(load_scene::load_scene()));
    let mut window =
        RenderWindow::new((1000, 1000), "Oh My God", Style::CLOSE, &Default::default());
   
    while window.is_open() {
        while let Some(e) = window.poll_event() {
            match e {
                Event::Closed => window.close(),
                _ => {}
            }
        }
        window.clear(Color::GREEN);

        draw_tree_to(&mut window, &scene.borrow().render_tree().borrow());
        run_tree_script(&mut scene.borrow_mut().render_tree().borrow_mut());
        make_random(&mut scene.borrow_mut().render_tree().borrow_mut());

        window.display();
    }
}

fn make_subrandom<'a>(tree: &mut RenderTree, ids: Vec<NodeId>) {
    let mut rng = rand::thread_rng();
    
    for id in ids {
        let mut node = tree.tree_mut().get_mut(id).unwrap();
        let j = node.data().element_mut();
        let xmove = rng.gen_range(-6..7) as f32;
        let ymove = rng.gen_range(-6..7) as f32;
        match j {
            RenderNode::CircleShape { shape } => {
                let mut pos = shape.position();
                pos += Vector2f::new(xmove, ymove);
                shape.set_position(pos);
            }
            RenderNode::Sprite { shape } => {
                let mut pos = shape.position();
                pos += Vector2f::new(xmove, ymove);
                shape.set_position(pos);
            },
        }
    }
}

fn make_random<'a>(tree: &mut RenderTree) {
    let ids = tree.collect_trees_node_ids();
    make_subrandom(tree, ids);
}
