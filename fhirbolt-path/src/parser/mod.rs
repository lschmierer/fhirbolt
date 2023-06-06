pub mod ast;

mod ast_from_pest;
mod pest_parser;

use pest::{error::Error as PestError, Parser};

use ast::Expression;
use pest_parser::{PestParser, Rule};

pub fn parse_fhir_path(s: &str) -> Result<Expression<'_>, PestError<Rule>> {
    Ok(Expression::from(PestParser::parse(Rule::start, s)?))
}

#[cfg(test)]
mod tests {
    use super::{ast::*, *};

    #[test]
    fn test_expression() {
        assert_eq!(
            parse_fhir_path("x.`y`(a = 3)").unwrap(),
            Expression::Invocation(InvocationExpression {
                left: Box::new(Expression::Term(Term::Invocation(Invocation::Identifier(
                    "x"
                )))),
                right: Invocation::Function(Function {
                    identifier: "y",
                    parameters: vec![Box::new(Expression::Equality(EqualityExpression {
                        left: Box::new(Expression::Term(Term::Invocation(Invocation::Identifier(
                            "a"
                        )))),
                        op: EqualityOp::Equal,
                        right: Box::new(Expression::Term(Term::Literal(Literal::Number("3")))),
                    }))]
                }),
            })
        );
    }
}
