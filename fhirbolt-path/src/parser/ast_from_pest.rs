use pest::iterators::{Pair, Pairs};

use crate::parser::{ast::Expression, pest_parser::Rule};

use super::ast::{
    AdditiveExpression, AdditiveOp, AndExpression, EqualityExpression, EqualityOp, Function,
    ImpliesExpression, IndexerExpression, InequalityExpression, InequalityOp, Invocation,
    InvocationExpression, Literal, MembershipExpression, MembershipOp, MultiplicativeExpression,
    MultiplicativeOp, OrExpression, OrOp, PolarityExpression, PolarityOp, Quantity, Term,
    TypeExpression, TypeOp, TypeSpecifier, UnionExpression,
};

impl<'i> From<Pairs<'i, Rule>> for Expression<'i> {
    fn from(mut pairs: Pairs<'i, Rule>) -> Self {
        Expression::from(pairs.next().unwrap())
    }
}

impl<'i> From<Pair<'i, Rule>> for Expression<'i> {
    fn from(pair: Pair<'i, Rule>) -> Self {
        match pair.as_rule() {
            Rule::start | Rule::expression => Expression::from(pair.into_inner().next().unwrap()),
            Rule::termExpression => Expression::Term(Term::from(pair.into_inner().next().unwrap())),
            Rule::invocationExpression => InvocationExpression::from_pair(pair),
            Rule::indexerExpression => IndexerExpression::from_pair(pair),
            Rule::polarityExpression => PolarityExpression::from_pair(pair),
            Rule::multiplicativeExpression => MultiplicativeExpression::from_pair(pair),
            Rule::additiveExpression => AdditiveExpression::from_pair(pair),
            Rule::typeExpression => TypeExpression::from_pair(pair),
            Rule::unionExpression => UnionExpression::from_pair(pair),
            Rule::inequalityExpression => InequalityExpression::from_pair(pair),
            Rule::equalityExpression => EqualityExpression::from_pair(pair),
            Rule::membershipExpression => MembershipExpression::from_pair(pair),
            Rule::andExpression => AndExpression::from_pair(pair),
            Rule::orExpression => OrExpression::from_pair(pair),
            Rule::impliesExpression => ImpliesExpression::from_pair(pair),
            _ => unreachable!("{:?}", pair),
        }
    }
}

impl<'i> From<Pair<'i, Rule>> for Box<Expression<'i>> {
    fn from(pair: Pair<'i, Rule>) -> Self {
        Box::new(Expression::from(pair))
    }
}

trait ExpressionFromPair {
    fn from_pair(value: Pair<Rule>) -> Expression;
}

macro_rules! impl_expr_from_pair {
    ( $t:ident { $v:ident($r:ty) } ) => {
        impl ExpressionFromPair for $t<'_> {
            fn from_pair(pair: Pair<Rule>) -> Expression {
                let mut pairs = pair.into_inner();
                let mut left = Expression::from(pairs.next().unwrap());

                while let Some(right) = pairs.next() {
                    left = Expression::$v($t {
                        left: Box::new(left),
                        right: <$r>::from(right),
                    });
                }

                left
            }
        }
    };
    ( $t:ident { $v:ident($r:ty, $o:ident ) } ) => {
        impl ExpressionFromPair for $t<'_> {
            fn from_pair(pair: Pair<Rule>) -> Expression {
                let mut pairs = pair.into_inner();
                let mut left = Expression::from(pairs.next().unwrap());

                while let Some(op) = pairs.next() {
                    left = Expression::$v($t {
                        left: Box::new(left),
                        op: $o::from(op),
                        right: <$r>::from(pairs.next().unwrap()),
                    });
                }

                left
            }
        }
    };
}

macro_rules! impl_op_from_pair {
    ( $t:ident { $( $s:literal => $v:ident ),* $(,)? } ) => {
        impl From<Pair<'_, Rule>> for $t {
            fn from(pair: Pair<Rule>) -> Self {
                match pair.as_str() {
                    $(
                        $s => $t::$v,
                    )*
                    _ => unreachable!(),
                }
            }
        }
    };
}

impl_expr_from_pair!(InvocationExpression { Invocation(Invocation) });

impl_expr_from_pair!(IndexerExpression { Indexer(Box<Expression>) });

impl ExpressionFromPair for PolarityExpression<'_> {
    fn from_pair(pair: Pair<Rule>) -> Expression {
        let mut pairs = pair.into_inner();

        let next = pairs.next().unwrap();

        if next.as_rule() == Rule::polarityOp {
            Expression::Polarity(PolarityExpression {
                op: PolarityOp::from(next),
                expression: Box::<Expression>::from(pairs.next().unwrap()),
            })
        } else {
            Expression::from(next)
        }
    }
}
impl_op_from_pair!(PolarityOp {
    "+" => Positive,
    "-" => Negative,
});

impl_expr_from_pair!(MultiplicativeExpression {
    Multiplicative(Box<Expression>, MultiplicativeOp)
});
impl_op_from_pair!(MultiplicativeOp {
    "*" => Multiplication,
    "/"=> Division,
    "div"=> Div,
    "mod"=> Mod,
});

impl_expr_from_pair!(AdditiveExpression { Additive(Box<Expression>, AdditiveOp) });
impl_op_from_pair!(AdditiveOp {
    "+" => Addition,
    "-"=> Substraction,
    "&"=> StringConcatenation,
});

impl_expr_from_pair!(TypeExpression { Type(TypeSpecifier, TypeOp) });
impl_op_from_pair!(TypeOp {
    "is" => Is,
    "as"=> As,
});

impl_expr_from_pair!(UnionExpression { Union(Box<Expression>) });

impl_expr_from_pair!(InequalityExpression {
    Inequality(Box<Expression>, InequalityOp)
});
impl_op_from_pair!(InequalityOp {
    "<=" => LessEqual,
    "<"=> Less,
    ">"=> Greater,
    ">="=> GreaterEqual,
});

impl_expr_from_pair!(EqualityExpression {
    Equality(Box<Expression>, EqualityOp)
});
impl_op_from_pair!(EqualityOp {
    "=" => Equal,
    "~"=> Equivalent,
    "!="=> NotEqual,
    "!~"=> NotEquivalent,
});

impl_expr_from_pair!(MembershipExpression {
    Membership(Box<Expression>, MembershipOp)
});
impl_op_from_pair!(MembershipOp {
    "in" => In,
    "contains"=> Contains,
});

impl_expr_from_pair!(AndExpression { And(Box<Expression>) });

impl_expr_from_pair!(OrExpression { Or(Box<Expression>, OrOp) });
impl_op_from_pair!(OrOp {
    "or" => Or,
    "xor"=> Xor,
});

impl_expr_from_pair!(ImpliesExpression { Implies(Box<Expression>) });

impl<'i> From<Pair<'i, Rule>> for Term<'i> {
    fn from(pair: Pair<'i, Rule>) -> Self {
        match pair.as_rule() {
            Rule::term => Term::from(pair.into_inner().next().unwrap()),
            Rule::invocation => Term::Invocation(Invocation::from(pair)),
            Rule::literal => Term::Literal(Literal::from(pair)),
            Rule::externalConstant => Term::ExternalConstant(pair.as_str()),
            Rule::expression => Term::Parenthesized(Box::<Expression>::from(pair)),
            _ => unreachable!("{:?}", pair),
        }
    }
}

impl<'i> From<Pair<'i, Rule>> for Literal<'i> {
    fn from(pair: Pair<'i, Rule>) -> Self {
        match pair.as_rule() {
            Rule::literal => Literal::from(pair.into_inner().next().unwrap()),
            Rule::BOOL => Literal::Bool(pair.as_str().parse().unwrap()),
            Rule::STRING => Literal::String(pair.as_str()),
            Rule::NUMBER => Literal::Number(pair.as_str()),
            Rule::DATE => Literal::Date(pair.as_str()),
            Rule::DATETIME => Literal::DateTime(pair.as_str()),
            Rule::TIME => Literal::Time(pair.as_str()),
            Rule::quantity => Literal::Quantity(Quantity::from(pair)),
            _ => Literal::Null,
        }
    }
}

impl<'i> From<Pair<'i, Rule>> for Invocation<'i> {
    fn from(pair: Pair<'i, Rule>) -> Self {
        match pair.as_rule() {
            Rule::invocation => Invocation::from(pair.into_inner().next().unwrap()),
            Rule::function => Invocation::Function(Function::from(pair)),
            Rule::thisInvocation => Invocation::This,
            Rule::indexInvocation => Invocation::Index,
            Rule::totalInvocation => Invocation::Total,
            Rule::identifier => Invocation::Identifier(identifier_from_pair(pair)),
            _ => unreachable!("{:?}", pair),
        }
    }
}

impl<'i> From<Pair<'i, Rule>> for Function<'i> {
    fn from(pair: Pair<'i, Rule>) -> Self {
        let mut pairs = pair.into_inner();
        Function {
            identifier: identifier_from_pair(pairs.next().unwrap()),
            parameters: pairs
                .map(|p| Box::<Expression>::from(p.into_inner().next().unwrap()))
                .collect(),
        }
    }
}

impl<'i> From<Pair<'i, Rule>> for Quantity<'i> {
    fn from(pair: Pair<'i, Rule>) -> Self {
        let mut pairs = pair.into_inner();
        Quantity {
            value: pairs.next().unwrap().as_str(),
            unit: pairs.next().unwrap().as_str(),
        }
    }
}

impl<'i> From<Pair<'i, Rule>> for TypeSpecifier<'i> {
    fn from(pair: Pair<'i, Rule>) -> Self {
        let pairs = pair.into_inner().next().unwrap().into_inner();
        TypeSpecifier {
            qualified_identifiers: pairs.map(|p| identifier_from_pair(p)).collect(),
        }
    }
}

fn identifier_from_pair<'i>(pair: Pair<'i, Rule>) -> &'i str {
    let s = pair.as_str();

    if s.starts_with('`') {
        &s[1..s.len() - 1]
    } else {
        s
    }
}
