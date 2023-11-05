use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Deref, DerefMut},
};

use slab_tree::{NodeId, NodeMut, Tree, TreeBuilder};

use crate::script::Script;

pub struct GameElement<E> {
    node: E,
    script: Option<Script>,
}

impl<E> From<E> for GameElement<E> {
    fn from(value: E) -> Self {
        GameElement {
            node: value,
            script: None,
        }
    }
}

impl<E> GameElement<E> {
    pub fn set_script(&mut self, script: Script) {
        self.script = Some(script);
    }

    pub fn script(&self) -> Option<&Script> {
        if let Some(s) = &self.script {
            Some(s)
        } else {
            None
        }
    }
}

impl<E> Deref for GameElement<E> {
    type Target = E;
    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

pub struct GameTree<E> {
    tree: Tree<GameElement<E>>,
    path_map: HashMap<String, NodeId>, // node path of node id
}

impl<E> GameTree<E> {
    /// Create a game tree with a root and its path
    /// ```
    /// let tree = GameTree::<Box<dyn Drawable>>::from_root(
    ///     "/render".to_string(),
    ///     Box::new(CircleShape::new(50., 30)) as Box<dyn Drawable>
    /// );
    /// ```
    pub fn from_root(path: String, root: GameElement<E>) -> Self {
        let mut tree = GameTree {
            tree: TreeBuilder::new().with_root(root).build(),
            path_map: HashMap::new(),
        };
        tree.path_map.insert(path, tree.root_id().unwrap());
        tree
    }

    pub fn find_node_mut(&mut self, path: String) -> Option<NodeMut<GameElement<E>>> {
        if let Some(&id) = self.path_map.get(&path) {
            return self.tree.get_mut(id);
        }
        None
    }
}

impl<T> Deref for GameTree<T> {
    type Target = Tree<GameElement<T>>;

    fn deref(&self) -> &Self::Target {
        &self.tree
    }
}

impl<T> DerefMut for GameTree<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
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

impl<E> GameTree<E> {
    pub fn add_node(&mut self, path: String, elem: GameElement<E>) {
        let parent_path = path_parent(&path);
        let mut parent = self.find_node_mut(parent_path).unwrap();
        let id = parent.append(elem).node_id();
        self.path_map.insert(path, id);
    }
}

impl<E> Display for GameTree<E> {
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
