//!
//!

use std::collections::HashMap;

// Re-export
pub use crate::tree_common::FlattenTreeNodeExt;


///
pub struct FlattenTree {
    internal_hash_map: HashMap<String, Box<dyn FlattenTreeNodeExt>>
}


// impl<T> SimpleFastTree<T> {
// 
    // pub from_vec(root_nodes_vec: Vec<T>) -> Self {
        // SimpleFastTree {
            // internal_hash_map: HashMap::new()
        // }
    // }
// }
