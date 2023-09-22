mod render_tree;
mod render_nodes;
mod load_file;
mod tree;

use render_tree::draw_tree_to;
use sfml::{
	graphics::{RenderWindow, RenderTarget, Color},
	window::{Style, Event},
};

fn main() {
	let tree = load_file::load_tree_from_file();
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
		
		draw_tree_to(&mut window, &tree);

		window.display();
	}
}
