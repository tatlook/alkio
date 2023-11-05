mod load_scene;
mod render_tree;
mod scene;
mod script;
mod tree;

use render_tree::draw_tree_to;
use script::run_tree_script;
use sfml::{
	graphics::{RenderWindow, RenderTarget, Color},
	window::{Style, Event},
};
use std::{rc::Rc, cell::RefCell};


fn main() {
	println!("{:?}", std::env::current_dir().unwrap());
	let scene = Rc::new(RefCell::new(load_scene::load_scene()));
	let mut window = RenderWindow::new((1000, 1000), 
			"Oh My God",
			Style::CLOSE,
			&Default::default());
	while window.is_open() {
		while let Some(e) = window.poll_event() {
			match e {
			    Event::Closed => window.close(),
			    _ => {}
			}
		}
		window.clear(Color::GREEN);
		
		draw_tree_to(&mut window, &scene.borrow().render_tree());
		run_tree_script(scene.borrow().render_tree());

		window.display();
	}
}
