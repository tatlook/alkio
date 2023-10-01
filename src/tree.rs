
use std::{collections::HashMap, fmt::Display};

use slab_tree::{Tree, NodeId, TreeBuilder, NodeMut};

pub struct GameTree<E> {
    tree: Tree<E>,
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
    pub fn from_root(path: String, root: E) -> Self {
        let mut tree = GameTree {
            tree: TreeBuilder::new().with_root(root).build(),
            path_map: HashMap::new()
        };
        tree.path_map.insert(path, tree.tree().root_id().unwrap());
        tree
    }

    pub fn tree(&self) -> &Tree<E> {
        return &self.tree;
    }
    
    pub fn find_node_mut(&mut self, path: String) -> Option<NodeMut<E>> {
        if let Some(&id) = self.path_map.get(&path) {
            return self.tree.get_mut(id)
        }
        None
    }
}

fn path_parent(path: &String) -> String {
    let path_split: Vec<&str> = path.split('/').collect();
    let mut parent = String::new();
    for i in 0 .. path_split.len() {
        let node_name = path_split[i];
        parent.push_str(node_name);
        if i == path_split.len() - 2 {
            break;
        }
        parent.push_str( "/");
    }
    parent
}

impl<E> GameTree<E> {
    pub fn add_node(&mut self, path: String, elem: E) {
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

mod test {
    use super::path_parent;

    #[test]
    fn test_path_parent() {
        assert_eq!(path_parent(&String::from("/yee/gee")), "/yee")
    }
}
