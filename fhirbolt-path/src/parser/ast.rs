use std::fmt::{self, Debug, Formatter};

#[derive(Clone, PartialEq)]
pub enum Expression<'i> {
    Term(Term<'i>),
    Invocation(InvocationExpression<'i>),
    Indexer(IndexerExpression<'i>),
    Polarity(PolarityExpression<'i>),
    Multiplicative(MultiplicativeExpression<'i>),
    Additive(AdditiveExpression<'i>),
    Type(TypeExpression<'i>),
    Union(UnionExpression<'i>),
    Inequality(InequalityExpression<'i>),
    Equality(EqualityExpression<'i>),
    Membership(MembershipExpression<'i>),
    And(AndExpression<'i>),
    Or(OrExpression<'i>),
    Implies(ImpliesExpression<'i>),
}

impl Debug for Expression<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Term(e) => e.fmt(f),
            Self::Invocation(e) => e.fmt(f),
            Self::Indexer(e) => e.fmt(f),
            Self::Polarity(e) => e.fmt(f),
            Self::Multiplicative(e) => e.fmt(f),
            Self::Additive(e) => e.fmt(f),
            Self::Type(e) => e.fmt(f),
            Self::Union(e) => e.fmt(f),
            Self::Inequality(e) => e.fmt(f),
            Self::Equality(e) => e.fmt(f),
            Self::Membership(e) => e.fmt(f),
            Self::And(e) => e.fmt(f),
            Self::Or(e) => e.fmt(f),
            Self::Implies(e) => e.fmt(f),
        }
    }
}

macro_rules! expr_struct {
    ( $n:ident<'i> { $l:ty, $r:ty } ) => {
        #[derive(Clone, PartialEq, Debug)]
        pub struct $n<'i> {
            pub left: $l,
            pub right: $r,
        }
    };
    ( $n:ident<'i> { $l:ty, $r:ty } $o:ident { $( $i:ident ),* $(,)? } ) => {
        #[derive(Clone, PartialEq, Debug)]
        pub struct $n<'i> {
            pub left: $l,
            pub op: $o,
            pub right: $r,
        }

        #[derive(Clone, PartialEq, Debug)]
        pub enum $o {
            $(
                $i,
            )*
        }
    };
}

expr_struct!(InvocationExpression<'i> {
    Box<Expression<'i>>, Invocation<'i>
});

expr_struct!(IndexerExpression<'i> {
    Box<Expression<'i>>, Box<Expression<'i>>
});

#[derive(Clone, PartialEq, Debug)]
pub struct PolarityExpression<'i> {
    pub op: PolarityOp,
    pub expression: Box<Expression<'i>>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum PolarityOp {
    Positive,
    Negative,
}

expr_struct!(
    MultiplicativeExpression<'i> {
        Box<Expression<'i>>, Box<Expression<'i>>
    }
    MultiplicativeOp {
        Multiplication, Division, Div, Mod
    }
);

expr_struct!(
    AdditiveExpression<'i> {
        Box<Expression<'i>>, Box<Expression<'i>>
    }
    AdditiveOp {
        Addition, Substraction, StringConcatenation
    }
);

expr_struct!(
    TypeExpression<'i> {
        Box<Expression<'i>>, TypeSpecifier<'i>
    }
    TypeOp {
        Is, As
    }
);

expr_struct!(UnionExpression<'i> {
    Box<Expression<'i>>, Box<Expression<'i>>
});

expr_struct!(
    InequalityExpression<'i> {
        Box<Expression<'i>>, Box<Expression<'i>>
    }
    InequalityOp {
        LessEqual, Less, Greater, GreaterEqual
    }
);

expr_struct!(
    EqualityExpression<'i> {
        Box<Expression<'i>>, Box<Expression<'i>>
    }
    EqualityOp {
        Equal, Equivalent, NotEqual, NotEquivalent
    }
);

expr_struct!(
    MembershipExpression<'i> {
        Box<Expression<'i>>, Box<Expression<'i>>
    }
    MembershipOp {
        In, Contains
    }
);

expr_struct!(AndExpression<'i> {
    Box<Expression<'i>>, Box<Expression<'i>>
});

expr_struct!(
    OrExpression<'i> {
        Box<Expression<'i>>, Box<Expression<'i>>
    }
    OrOp {
        Or, Xor
    }
);

expr_struct!(ImpliesExpression<'i> {
    Box<Expression<'i>>, Box<Expression<'i>>
});

#[derive(Clone, PartialEq, Debug)]
pub enum Term<'i> {
    Invocation(Invocation<'i>),
    Literal(Literal<'i>),
    ExternalConstant(&'i str),
    Parenthesized(Box<Expression<'i>>),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Literal<'i> {
    Null,
    Bool(bool),
    String(&'i str),
    Number(&'i str),
    Date(&'i str),
    DateTime(&'i str),
    Time(&'i str),
    Quantity(Quantity<'i>),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Invocation<'i> {
    Identifier(&'i str),
    Function(Function<'i>),
    This,
    Index,
    Total,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Function<'i> {
    pub identifier: &'i str,
    pub parameters: Vec<Box<Expression<'i>>>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Quantity<'i> {
    pub value: &'i str,
    pub unit: &'i str,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TypeSpecifier<'i> {
    pub qualified_identifiers: Vec<&'i str>,
}
