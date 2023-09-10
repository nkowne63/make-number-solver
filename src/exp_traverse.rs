use crate::{
    expression::{Expression, ExpressionNode, Operator},
    num_traverse::{NumberReplacer, NumberReplacerIter},
    ops_traverse::OpsIter,
    rational::Rational,
};

impl NumberReplacer {
    // 最小値の部分で二つに分割する
    fn partition(&self) -> (NumberReplacer, NumberReplacer, usize) {
        let numbers = self.numbers.clone();
        // 最小値の位置を探す
        let min_index = numbers
            .iter()
            .enumerate()
            .min_by_key(|(_, n)| *n)
            .map(|(i, _)| i)
            .unwrap();
        // 最小値の位置で分割する
        let (left, right) = numbers.split_at(min_index);
        // rightの最初の要素は削除する
        let right = &right[1..];
        (
            NumberReplacer {
                numbers: left.to_vec(),
            },
            NumberReplacer {
                numbers: right.to_vec(),
            },
            min_index,
        )
    }
}

#[cfg(test)]
mod number_replacer_partition_test {
    use super::*;

    #[test]
    fn partition() {
        let replacer = NumberReplacer::new(vec![3, 1, 2]);
        let (left, right, min_index) = replacer.partition();
        assert_eq!(left.numbers, vec![3]);
        assert_eq!(right.numbers, vec![2]);
        assert_eq!(min_index, 1);
        let replacer2 = NumberReplacer::new(vec![3, 1, 2, 4]);
        let (left, right, min_index) = replacer2.partition();
        assert_eq!(left.numbers, vec![3]);
        assert_eq!(right.numbers, vec![2, 4]);
        assert_eq!(min_index, 1);
        let replacer3 = NumberReplacer::new(vec![1, 2, 4, 3]);
        let (left, right, min_index) = replacer3.partition();
        assert_eq!(left.numbers, vec![]);
        assert_eq!(right.numbers, vec![2, 4, 3]);
        assert_eq!(min_index, 0);
    }
}

// 構造、演算子、値の組み合わせから式を作成する
fn build_expression(
    structure: NumberReplacer,
    ops: Vec<Operator>,
    values: Vec<Rational>,
) -> ExpressionNode {
    // println!("structure: {:?}", structure);
    // println!("ops: {:?}", ops);
    assert!(ops.len() == structure.numbers.len());
    assert!(values.len() == structure.numbers.len() + 1);
    // structureを再帰的に分割して構文を作る
    // structureが空の場合はvaluesから値を取り出して式を作る
    if structure.numbers.is_empty() {
        assert!(values.len() == 1);
        assert!(ops.is_empty());
        return ExpressionNode::Value(values[0]);
    }
    // structureが複数の場合はstructureを分割して再帰的に式を作る
    let (left, right, min_index) = structure.partition();
    let op = ops[min_index];
    let lhs = Box::new(build_expression(
        left,
        ops[..min_index].to_vec(),
        // 1つ余分に数字を取る
        values[..min_index + 1].to_vec(),
    ));
    let rhs = Box::new(build_expression(
        right,
        ops[min_index + 1..].to_vec(),
        values[min_index + 1..].to_vec(),
    ));
    ExpressionNode::Exp(Expression { op, lhs, rhs })
}

#[cfg(test)]
mod build_expression_test {
    use super::*;
    #[test]
    fn build_expression_test() {
        let structure = NumberReplacer::new(vec![1, 3, 2]);
        let ops = vec![Operator::Add, Operator::Div, Operator::Mul];
        let values = vec![
            Rational {
                numerator: 1,
                denominator: 2,
            },
            Rational {
                numerator: 1,
                denominator: 3,
            },
            Rational {
                numerator: 1,
                denominator: 4,
            },
            Rational {
                numerator: 1,
                denominator: 5,
            },
        ];
        let expression = build_expression(structure, ops, values);
        assert_eq!(
            expression,
            ExpressionNode::Exp(Expression {
                op: Operator::Add,
                lhs: Box::new(ExpressionNode::Value(Rational {
                    numerator: 1,
                    denominator: 2,
                })),
                rhs: Box::new(ExpressionNode::Exp(Expression {
                    op: Operator::Mul,
                    lhs: Box::new(ExpressionNode::Exp(Expression {
                        op: Operator::Div,
                        lhs: Box::new(ExpressionNode::Value(Rational {
                            numerator: 1,
                            denominator: 3,
                        })),
                        rhs: Box::new(ExpressionNode::Value(Rational {
                            numerator: 1,
                            denominator: 4,
                        }))
                    })),
                    rhs: Box::new(ExpressionNode::Value(Rational {
                        numerator: 1,
                        denominator: 5,
                    })),
                })),
            })
        );
    }
}

pub(crate) struct ExpressionIter {
    structure_iter: NumberReplacerIter,
    current_structure: Option<NumberReplacer>,
    values_replacer_iter: NumberReplacerIter,
    current_values_replacer: Option<NumberReplacer>,
    ops_iter: OpsIter,
    values: Vec<Rational>,
}

impl ExpressionIter {
    pub(crate) fn new(values: Vec<Rational>) -> Self {
        let length = values.len();
        let mut structure_iter = NumberReplacerIter::new((1..length as u32).collect());
        let current_structure = structure_iter.next();
        let mut values_replacer_iter = NumberReplacerIter::new((1..=length as u32).collect());
        let current_values_replacer = values_replacer_iter.next();
        ExpressionIter {
            structure_iter,
            current_structure,
            values_replacer_iter,
            current_values_replacer,
            ops_iter: OpsIter::new(length - 1),
            values,
        }
    }
}

impl Iterator for ExpressionIter {
    type Item = ExpressionNode;
    fn next(&mut self) -> Option<Self::Item> {
        let next_ops = self.ops_iter.next();
        // 次に試すべき演算子の配置がない場合
        if next_ops.is_none() {
            // ops_iterをリセット
            self.ops_iter = OpsIter::new(self.values.len() - 1);
            // 値の置換を進める
            self.current_values_replacer = self.values_replacer_iter.next();
            // 次に試すべき値の置換がない場合
            if self.current_values_replacer.is_none() {
                // values_replacer_iterをリセット
                self.values_replacer_iter =
                    NumberReplacerIter::new((1..=self.values.len() as u32).collect());
                self.current_values_replacer = self.values_replacer_iter.next();
                // 構造を進める
                self.current_structure = self.structure_iter.next();
                // 次に試すべき構造がない場合、試すものはもうない
                self.current_structure.as_ref()?;
            }
        }
        // この時点で全てnoneではない
        let next_ops = match next_ops {
            Some(ops) => ops,
            None => self.ops_iter.next().unwrap(),
        };
        let current_values_replacer = self.current_values_replacer.as_ref().unwrap();
        let current_structure = self.current_structure.clone().unwrap();
        let values = current_values_replacer.apply(self.values.clone());
        Some(build_expression(current_structure, next_ops, values))
    }
}
