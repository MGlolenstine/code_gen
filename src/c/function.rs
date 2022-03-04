use super::code_block::CodeBlock;

#[derive(Clone)]
pub struct Argument{
    pub variable_type: String,
    pub name: String,
}


pub struct Function{
    pub return_type: String,
    pub name: String,
    pub arguments: Vec<Argument>,
    pub code_block: Box<dyn CodeBlock>,
}

impl Clone for Function{
    fn clone(&self) -> Self{
        Function{
            return_type: self.return_type.clone(),
            name: self.name.clone(),
            arguments: self.arguments.clone(),
            code_block: self.code_block.to_cb(),
        }
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        write!(f, "{} {}({}){{\n", self.return_type, self.name, self.arguments.iter().map(|a|format!("{} {}", a.variable_type, a.name)).collect::<Vec<String>>().join(", "))?;
        write!(f, "{}\n", self.code_block)?;
        write!(f, "}}")
    }
}
