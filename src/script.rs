use rhai::{Engine, ParseError, AST, Scope};
use slab_tree::NodeId;
use std::cell::RefCell;
use std::rc::Rc;

use crate::tree::GameTree;

type EngineRef = Rc<RefCell<Engine>>;

pub struct Script<'a> {
    engine: EngineRef,
    scope: Scope<'a>,
    ast: AST,
}

impl<'a> Script<'a> {
    pub fn compile(engine: EngineRef, scope: Scope<'a>, script: String) -> Result<Self, ParseError> {
        let ast = engine.borrow().compile_with_scope(&scope, script)?;
        Ok(Script { engine, scope, ast })
    }

    pub fn run(&mut self) {
        if let Err(e) = self.engine.borrow().run_ast_with_scope(&mut self.scope, &self.ast) {
            eprintln!("Script's runtime error: {e}");
        }
    }
}

fn run_node_script<'a, T>(tree: &mut GameTree<'a, T>, ids: Vec<NodeId>) {
    for id in ids {
        let mut node = tree.tree_mut().get_mut(id).unwrap();
        if let Some(script) = node.data().script_mut() {
            script.run();
        }
    }
}

pub fn run_tree_script<'a, T>(mut tree: &'a mut GameTree<T>) {
    let all_ids = tree.collect_trees_node_ids();
    run_node_script(&mut tree, all_ids);
}
