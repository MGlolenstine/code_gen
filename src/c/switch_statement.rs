use super::expression::Expression;
use super::code_block::CodeBlock;

pub struct Case{
    expression: Expression,
    code_block: Box<dyn CodeBlock>,
    breaks: bool,
}

impl Clone for Case{
    fn clone(&self) -> Self{
        Case{
            expression: self.expression.clone(),
            code_block: self.code_block.to_cb(),
            breaks: self.breaks,
        }
    }
}

pub struct Switch{
    pub condition: Expression,
    pub cases: Vec<Case>,
    pub default: Option<Box<dyn CodeBlock>>,
}

impl Clone for Switch{
    fn clone(&self) -> Self{
        let default = if let Some(d) = self.default.as_ref(){
            Some(d.to_cb())
        }else{
            None
        };
        Switch{
            condition: self.condition.clone(),
            cases: self.cases.clone(),
            default,
        }
    }
}

impl std::fmt::Display for Switch{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        write!(f, "switch({}){{\n", self.condition)?;
        for c in self.cases.iter() {
            write!(f, "case {}:\n", c.expression)?;
            write!(f, "{}\n", c.code_block)?;
            if c.breaks {
                write!(f, "break;")?;
            }
        }
        if let Some(default) = self.default.as_ref(){
            write!(f, "default:\n")?;
            write!(f, "{}\n", default)?;
        }
        write!(f, "}}")
    }
}
