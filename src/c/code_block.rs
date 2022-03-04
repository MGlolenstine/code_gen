pub trait CodeBlock: std::fmt::Display {
    fn to_cb(&self) -> Box<dyn CodeBlock>;
}

// impl std::fmt::Display for CodeBlock{
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>)-> std::fmt::Result{
//         write!(f, "//CodeBlocks are WIP!")
//     }
// }
