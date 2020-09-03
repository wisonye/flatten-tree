//!
//!

/// All node structs should apply the `derive` marco like this:
///
/// `#[derive(SimpleFastNode)]`
///
/// After that, any struct will become a tree node struct which implement this trait which is
/// the key to make this tree behavior works.
pub trait SimpleFastTreeNode {
    fn generate_tree_node_hashmap_key(&self) -> String;

    fn get_data(&self) {
        println!("'get_data' get called >>>>>>>>>>>>>>>>>");
    }
}
