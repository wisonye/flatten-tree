/// All node structs should apply the `derive` marco like this:
///
/// `#[derive(SimpleFastNode)]`
///
/// After that, any struct will become a tree node struct which implement this trait which is
/// the key to make this tree behavior works.
pub trait FlattenTreeNodeExt {
    fn generate_tree_node_hashmap_key(&self) -> String;

    fn get_title(&self) -> String;

    fn print_debug(&self) {
        println!("'print_debug' get called >>>>>>>>>>>>>>>>>");
    }
}
