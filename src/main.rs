mod render_tree;
mod logic_tree;
mod scene;
mod load_scene;
mod tree;

use render_tree::draw_tree_to;
use sfml::{
	graphics::{RenderWindow, RenderTarget, Color},
	window::{Style, Event},
};

use crate::logic_tree::run_logics;

fn main() {
	let scene = load_scene::load_scene();
	println!("{scene}");
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
		
		draw_tree_to(&mut window, &scene.render_tree());
		run_logics(&scene.logic_tree());

		window.display();
	}
}
