use rhai::{Engine, ParseError, AST};
use slab_tree::NodeRef;
use std::cell::RefCell;
use std::rc::Rc;

use crate::tree::{GameTree, GameElement};

type EngineRef = Rc<RefCell<Engine>>;

pub struct Script {
    engine: EngineRef,
    ast: AST,
}

impl Script {
    pub fn compile(engine: EngineRef, script: String) -> Result<Self, ParseError> {
        let ast = engine.borrow().compile(script)?;
        Ok(Script { engine, ast })
    }

    pub fn run(&self) {
        if let Err(e) = self.engine.borrow().run_ast(&self.ast) {
            eprintln!("Script's runtime error: {e}");
        }
    }
}

fn run_node_script<T>(node: NodeRef<'_, GameElement<T>>) {
    for child in node.children() {
        run_node_script(child);
    }
    if let Some(script) = node.data().script() {
        script.run();
    }
}

pub fn run_tree_script<T>(tree: &GameTree<T>) {
    let root = tree
        .get(tree.root_id().expect("This tree has no root"))
        .unwrap();
    run_node_script(root);
}
