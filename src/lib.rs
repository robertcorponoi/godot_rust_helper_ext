/// Defines the extension methods that get applied to Nodes.
pub trait NodeExt {
	/// Returns a typed node.
	unsafe fn get_typed_node<T: gdnative::GodotObject, P: Into<gdnative::NodePath>>(&self, path: P) -> Option<T>;
}

/// Implement the `NodeExt` trait for `Node`.
impl NodeExt for gdnative::Node {
	/// Get a node from a provided path and cast it into a specified type.
	///
	/// # Arguments
	///
	/// `path` - The path to the Node in the Godot project node tree.
	unsafe fn get_typed_node<T: gdnative::GodotObject, P: Into<gdnative::NodePath>>(&self, path: P) -> Option<T> {
		self.get_node(path.into()).and_then(|node| node.cast())
	}
}