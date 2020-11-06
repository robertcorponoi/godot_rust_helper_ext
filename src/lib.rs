use gdnative::prelude::{GodotObject, Node, NodePath, Shared, SubClass, TRef};

/// Defines the extension methods that get applied to Nodes.
pub trait NodeExt {
    /// Returns a typed node with the specified path.
    unsafe fn get_typed_node<T, P>(&self, path: P) -> TRef<'_, T, Shared>
    where
        T: GodotObject + SubClass<Node>,
        P: Into<NodePath>;
}

// /// Implement the `NodeExt` trait for `Node`.
// impl NodeExt for gdnative::api::Node {
//     /// Get a node from a provided path and cast it into a specified type.
//     ///
//     /// # Arguments
//     ///
//     /// `path` - The path to the Node in the Godot project node tree.
//     unsafe fn get_typed_node<T: GodotObject, P: Into<NodePath>>(&self, path: P) -> Option<T> {
//         self.get_node(path.into()).and_then(|node| {
//             node.assume_safe();
//             node.cast::<GodotObject>();
//         });
//     }
// }

// use gdnative::prelude::*;

impl NodeExt for Node {
    /// Get a node from a provided path and cast it into a specified type.
    ///
    /// # Arguments
    ///
    /// `path` - The path to the Node in the Godot project node tree.
    unsafe fn get_typed_node<T, P>(&self, path: P) -> TRef<'_, T, Shared>
    where
        T: GodotObject + SubClass<Node>,
        P: Into<NodePath>,
    {
        self.get_node(path.into())
            .expect("Unable to find node")
            .assume_safe()
            .cast()
            .expect("Unable to cast node to specified type")
    }
}
