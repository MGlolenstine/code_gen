use super::code_block::CodeBlock;

#[derive(Clone)]
pub enum Modulator{
    Static,
    Const,
    None
}

#[derive(Clone)]
pub enum Expression{
    Empty,
    Other{data: String},
    Comment{text: String},
    Constant{lhs: String},
    Variable{lhs: String},
    Initialise{modulator: Modulator, variable_type: String, lhs: String, rhs: Box<Expression>},
    Assignment{lhs: Box<Expression>, rhs: Box<Expression>},
    Add{lhs: Box<Expression>, rhs: Box<Expression>},
    Sub{lhs: Box<Expression>, rhs: Box<Expression>},
    Mult{lhs: Box<Expression>, rhs: Box<Expression>},
    Div{lhs: Box<Expression>, rhs: Box<Expression>},
    Eq{lhs: Box<Expression>, rhs: Box<Expression>},
    Ne{lhs: Box<Expression>, rhs: Box<Expression>},
    Gt{lhs: Box<Expression>, rhs: Box<Expression>},
    Lt{lhs: Box<Expression>, rhs: Box<Expression>},
    Ge{lhs: Box<Expression>, rhs: Box<Expression>},
    Le{lhs: Box<Expression>, rhs: Box<Expression>},
    Not{lhs: Box<Expression>},
}

impl CodeBlock for Expression{
    fn to_cb(&self) -> Box<dyn CodeBlock>{
        Box::new(self.clone())
    }
}

impl std::fmt::Display for Expression{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Expression::Empty => write!(f, ""),
            Expression::Other{data} => write!(f, "{data}"),
            Expression::Comment{text} => {
                if text.contains("\n"){
                    write!(f, "/*\n{text}\n*/\n")
                }else{
                    write!(f, "//{text}\n")
                }
            },
            Expression::Constant{lhs} => write!(f, "{lhs}"),
            Expression::Variable{lhs} => write!(f, "{lhs}"),
            Expression::Initialise{modulator, variable_type, lhs, rhs} => {
                match modulator{
                    Modulator::Static => {
                        write!(f, "static {variable_type} {lhs} = {rhs};")
                    }
                    Modulator::Const => {
                        write!(f, "const {variable_type} {lhs} = {rhs};")
                    }
                    Modulator::None => {
                        write!(f, "{variable_type} {lhs} = {rhs};")
                    }
                }
            },
            Expression::Assignment{lhs, rhs} => write!(f, "{lhs} = {rhs};"),
            Expression::Add{lhs, rhs} => write!(f, "({lhs} + {rhs})"),
            Expression::Sub{lhs, rhs} => write!(f, "({lhs} - {rhs})"),
            Expression::Mult{lhs, rhs} => write!(f, "({lhs} * {rhs})"),
            Expression::Div{lhs, rhs} => write!(f, "({lhs} / {rhs})"),
            Expression::Eq{lhs, rhs} => write!(f, "({lhs} == {rhs})"),
            Expression::Ne{lhs, rhs} => write!(f, "({lhs} != {rhs})"),
            Expression::Gt{lhs, rhs} => write!(f, "({lhs} > {rhs})"),
            Expression::Lt{lhs, rhs} => write!(f, "({lhs} < {rhs})"),
            Expression::Ge{lhs, rhs} => write!(f, "({lhs} >= {rhs})"),
            Expression::Le{lhs, rhs} => write!(f, "({lhs} <= {rhs})"),
            Expression::Not{lhs} => write!(f, "!{lhs}"),
        }
    }
}

#[test]
fn test_expressions(){
    let simple_expression = Expression::Initialise{modulator: Modulator::None, variable_type: "uint8_t".to_string(), lhs: "a".to_string(), rhs: Box::new(Expression::Constant{lhs: "23".to_string()})};
    assert_eq!(simple_expression.to_string(), format!("uint8_t a = 23;"));
    let multiple_comparisons = Expression::Eq{
        lhs: Box::new(Expression::Gt{
            lhs: Box::new(Expression::Constant{
                lhs: "2".to_string()
            }),
            rhs: Box::new(Expression::Variable{
                lhs: "a".to_string()
            })
        }),
        rhs: Box::new(Expression::Ge{
            lhs: Box::new(Expression::Variable{
                lhs: "b".to_string()
            }),
            rhs: Box::new(Expression::Constant{
                lhs: "5".to_string()
            })
        }),
    };
    assert_eq!(multiple_comparisons.to_string(), format!("((2 > a) == (b >= 5))"));
}
