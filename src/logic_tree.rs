use slab_tree::NodeRef;

use crate::tree::GameTree;


pub type LogicTree = GameTree<String>;


use rhai::{Engine, EvalAltResult};

pub fn run_rhai(script: &str) -> Result<(), Box<EvalAltResult>>
{
    let engine = Engine::new();
    engine.run(script)?;

    // Done!
    Ok(())
}

fn run_logics_children(node: NodeRef<'_, String>) {
    for child in node.children() {
        run_logics_children(child);
    }
    run_rhai(node.data()).unwrap();
}

pub fn run_logics(logic_tree: &LogicTree) {
    let tree = logic_tree.tree();
    let root = tree
        .get(tree.root_id().expect("This tree has no root"))
        .unwrap();
    run_logics_children(root);
}
