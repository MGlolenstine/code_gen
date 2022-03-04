use super::expression::Expression;
use super::code_block::CodeBlock;

pub struct If{
    pub condition: Expression,
    pub code_block: Box<dyn CodeBlock>,
    pub default: Option<Box<dyn CodeBlock>>,
    pub else_if: Vec<ElseIf>,
}

impl Clone for If{
    fn clone(&self) -> Self{
        let default = if let Some(d) = self.default.as_ref(){
            Some(d.to_cb())
        }else{
            None
        };
        If{
            condition: self.condition.clone(),
            code_block: self.code_block.to_cb(),
            default,
            else_if: self.else_if.clone()
        }
    }
}

impl CodeBlock for If{
    fn to_cb(&self) -> Box<dyn CodeBlock>{
        Box::new(self.clone())
    }
}

pub struct ElseIf{
    pub condition: Expression,
    pub code_block: Box<dyn CodeBlock>,
}

impl Clone for ElseIf{
    fn clone(&self) -> Self {
        ElseIf{
            condition: self.condition.clone(),
            code_block: self.code_block.to_cb(),
        }
    }
}

impl std::fmt::Display for If{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "if({}){{\n", self.condition)?;
        write!(f, "{}\n", self.code_block)?;
        write!(f, "}}")?;

        for e_if in self.else_if.iter(){
            write!(f, " else if({}) {{\n", e_if.condition)?;
            write!(f, "{}\n", e_if.code_block)?;
            write!(f, "}}")?;
        }

        if let Some(default) = self.default.as_ref(){
            write!(f, " else{{\n")?;
            write!(f, "{default}\n")?;
            write!(f, "}}")?;
        }
        write!(f, "")
    }
}

#[test]
fn test_ifs(){
    let simple_if = If{condition: Expression::Ne{
        lhs: Box::new(Expression::Constant{lhs: "2".to_string()}),
        rhs: Box::new(Expression::Variable{lhs: "a".to_string()}),
    },
                       code_block: Box::new(Expression::Comment{text: "CodeBlocks are WIP!".to_string()}),
                       default: None,
                       else_if: vec![],
    };
    assert_eq!(simple_if.to_string(), r#"if((2 != a)){
//CodeBlocks are WIP!

}"#);
}
