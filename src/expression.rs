use std::fmt::Display;

use super::rational::Rational;

// 式中の演算子
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Sub => write!(f, "-"),
            Operator::Mul => write!(f, "*"),
            Operator::Div => write!(f, "/"),
        }
    }
}

// リストアップ順序
impl Operator {
    pub fn list() -> Vec<Operator> {
        vec![Operator::Add, Operator::Sub, Operator::Mul, Operator::Div]
    }
}

// 式中の式もしくは変数
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExpressionNode {
    Value(Rational),
    Exp(Expression),
}

impl Display for ExpressionNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionNode::Value(r) => r.fmt(f),
            ExpressionNode::Exp(e) => e.fmt(f),
        }
    }
}

// 式
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Expression {
    pub(crate) op: Operator,
    pub(crate) lhs: Box<ExpressionNode>,
    pub(crate) rhs: Box<ExpressionNode>,
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        self.lhs.fmt(f)?;
        write!(f, " {} ", self.op)?;
        self.rhs.fmt(f)?;
        write!(f, ")")
    }
}

// 値に変換可能
pub(crate) trait RationalEvaluatable {
    fn evaluate(&self) -> Rational;
}

impl RationalEvaluatable for ExpressionNode {
    fn evaluate(&self) -> Rational {
        match self {
            ExpressionNode::Value(r) => *r,
            ExpressionNode::Exp(e) => e.evaluate(),
        }
    }
}

impl RationalEvaluatable for Expression {
    fn evaluate(&self) -> Rational {
        let lhs = self.lhs.evaluate();
        let rhs = self.rhs.evaluate();
        match self.op {
            Operator::Add => lhs + rhs,
            Operator::Sub => lhs - rhs,
            Operator::Mul => lhs * rhs,
            Operator::Div => lhs / rhs,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn evaluate() {
        let e = Expression {
            op: Operator::Add,
            lhs: Box::new(ExpressionNode::Value(Rational {
                numerator: 1,
                denominator: 2,
            })),
            rhs: Box::new(ExpressionNode::Value(Rational {
                numerator: 1,
                denominator: 3,
            })),
        };
        assert_eq!(
            e.evaluate(),
            Rational {
                numerator: 5,
                denominator: 6
            }
        );
    }
}
