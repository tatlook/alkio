use std::{
    collections::HashMap,
    fmt::Display,
};

use slab_tree::{NodeId, NodeMut, Tree, TreeBuilder, NodeRef};

use crate::script::Script;

pub struct GameElement<'a, E> {
    node: E,
    script: Option<Script<'a>>,
}

impl<'a, E> From<E> for GameElement<'a, E> {
    fn from(value: E) -> Self {
        GameElement {
            node: value,
            script: None,
        }
    }
}

impl<'a, E> GameElement<'a, E> {
    pub fn set_script(&mut self, script: Script<'a>) {
        self.script = Some(script);
    }

    pub fn script_mut(&mut self) -> Option<&mut Script<'a>> {
        if let Some(s) = &mut self.script {
            Some(s)
        } else {
            None
        }
    }

    pub fn element(&self) -> &E{
        &self.node
    }

    pub fn element_mut(&mut self) -> &mut E{
        &mut self.node
    }
}

pub struct GameTree<'a, E> {
    tree: Tree<GameElement<'a, E>>,
    path_map: HashMap<String, NodeId>, // node path of node id
}

impl<'a, E> GameTree<'a, E> {
    /// Create a game tree with a root and its path
    /// ```
    /// let tree = GameTree::<Box<dyn Drawable>>::from_root(
    ///     "/render".to_string(),
    ///     Box::new(CircleShape::new(50., 30)) as Box<dyn Drawable>
    /// );
    /// ```
    pub fn from_root(path: String, root: GameElement<'a, E>) -> Self {
        let mut tree = GameTree {
            tree: TreeBuilder::new().with_root(root).build(),
            path_map: HashMap::new(),
        };
        tree.path_map.insert(path, tree.tree().root_id().unwrap());
        tree
    }

    fn collect_node_ids<'s>(node: NodeRef<'s, GameElement<E>>) -> Vec<NodeId> {
        let mut ids = Vec::<NodeId>::new();
        for child in node.children() {
            let mut child_ids = Self::collect_node_ids(child);
            ids.append(&mut child_ids);
        }
        ids.push(node.node_id());
        ids
    }
    
    pub fn collect_trees_node_ids(&self) -> Vec<NodeId> {
        Self::collect_node_ids(self.tree().root().expect("This tree as no root"))
    }

    pub fn find_node_mut(&mut self, path: String) -> Option<NodeMut<GameElement<'a, E>>> {
        if let Some(&id) = self.path_map.get(&path) {
            return self.tree.get_mut(id);
        }
        None
    }

    pub fn tree(&self) -> &Tree<GameElement<'a, E>> {
        &self.tree
    }

    pub fn tree_mut(&mut self) -> &mut Tree<GameElement<'a, E>> {
        &mut self.tree
    }
}

fn path_parent(path: &String) -> String {
    let path_split: Vec<&str> = path.split('/').collect();
    let mut parent = String::new();
    for i in 0..path_split.len() {
        let node_name = path_split[i];
        parent.push_str(node_name);
        if i == path_split.len() - 2 {
            break;
        }
        parent.push_str("/");
    }
    parent
}

impl<'a, E> GameTree<'a, E> {
    pub fn add_node(&mut self, path: String, elem: GameElement<'a, E>) {
        let parent_path = path_parent(&path);
        let mut parent = self.find_node_mut(parent_path).unwrap();
        let id = parent.append(elem).node_id();
        self.path_map.insert(path, id);
    }
}

impl<'a, E> Display for GameTree<'a, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.path_map.keys())
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_path_parent() {
        assert_eq!(super::path_parent(&String::from("/yee/gee")), "/yee")
    }
}
