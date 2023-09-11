use std::{fmt::Display, vec::IntoIter};

use itertools::{iproduct, ConsTuples, Itertools, MultiProduct, Permutations, Product};
use num_rational::{Ratio, Rational32};
use num_traits::{ops::checked::CheckedMul, CheckedDiv};

use crate::{bin_tree::Node, new_ops::Operator};

fn num_rational(num: i32) -> Rational32 {
    Rational32::from_integer(num)
}

fn nums_rational(nums: Vec<i32>) -> Vec<Rational32> {
    nums.into_iter().map(num_rational).collect()
}

type AllIterProduct = ConsTuples<
    Product<
        Product<Permutations<IntoIter<Ratio<i32>>>, MultiProduct<IntoIter<Operator>>>,
        IntoIter<Node>,
    >,
    ((Vec<Ratio<i32>>, Vec<Operator>), Node),
>;

pub(crate) struct AllTraverse {
    all_product: AllIterProduct,
}

pub(crate) struct TraversalableNums {
    nums: Vec<i32>,
}

impl TraversalableNums {
    pub(crate) fn new(nums: Vec<i32>) -> Self {
        Self { nums }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ExpressionInstance {
    nums: Vec<Rational32>,
    operators: Vec<Operator>,
    structure: Node,
}

impl IntoIterator for TraversalableNums {
    type Item = ExpressionInstance;
    type IntoIter = AllTraverse;

    fn into_iter(self) -> Self::IntoIter {
        let nums = self.nums;
        let length = nums.len();
        let nums_permutations = nums_rational(nums).into_iter().permutations(length);
        let operator_iter = (0..=length - 1)
            .map(|_| Operator::list())
            .multi_cartesian_product();
        let structure_iter = Node::get_all_tree(length)
            .into_iter()
            .collect::<Vec<_>>()
            .into_iter();
        let all_product = iproduct!(nums_permutations, operator_iter, structure_iter);
        AllTraverse { all_product }
    }
}

impl Iterator for AllTraverse {
    type Item = ExpressionInstance;
    fn next(&mut self) -> Option<Self::Item> {
        let (nums, operators, structure) = self.all_product.next()?;
        Some(ExpressionInstance {
            nums,
            operators,
            structure,
        })
    }
}

impl ExpressionInstance {
    pub(crate) fn evaluate(&self) -> Option<Rational32> {
        match &self.structure {
            Node::Leaf => Some(self.nums[0]),
            Node::Node { left, right } => {
                let left_leaves_count = left.count_leaf();
                let left_nums = self.nums[0..left_leaves_count].to_vec();
                let right_nums = self.nums[left_leaves_count..].to_vec();
                // 先頭は今ここで使う
                let left_ops = self.operators[1..left_leaves_count].to_vec();
                let right_ops = self.operators[left_leaves_count..].to_vec();
                let left = ExpressionInstance {
                    nums: left_nums,
                    operators: left_ops,
                    structure: *left.clone(),
                };
                let right = ExpressionInstance {
                    nums: right_nums,
                    operators: right_ops,
                    structure: *right.clone(),
                };
                let left_eval = left.evaluate()?;
                let right_eval = right.evaluate()?;
                // println!(
                //     "operator: {}, left: {}, right: {}",
                //     self.operators[0], left_eval, right_eval
                // );
                match self.operators[0] {
                    Operator::Add => Some(left_eval + right_eval),
                    Operator::Sub => Some(left_eval - right_eval),
                    Operator::Mul => left_eval.checked_mul(&right_eval),
                    Operator::Div => left_eval.checked_div(&right_eval),
                }
            }
        }
    }
}

impl Display for ExpressionInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.structure {
            Node::Leaf => write!(f, "{}", self.nums[0]),
            Node::Node { left, right } => {
                let left_leaves_count = left.count_leaf();
                let left_nums = self.nums[0..left_leaves_count].to_vec();
                let right_nums = self.nums[left_leaves_count..].to_vec();
                // 先頭は今ここで使う
                let left_ops = self.operators[1..left_leaves_count].to_vec();
                let right_ops = self.operators[left_leaves_count..].to_vec();
                let left = ExpressionInstance {
                    nums: left_nums,
                    operators: left_ops,
                    structure: *left.clone(),
                };
                let right = ExpressionInstance {
                    nums: right_nums,
                    operators: right_ops,
                    structure: *right.clone(),
                };
                write!(f, "({} {} {})", left, self.operators[0], right)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn expression_test() {
        let e = ExpressionInstance {
            nums: vec![num_rational(1), num_rational(2), num_rational(3)],
            operators: vec![Operator::Add, Operator::Mul],
            structure: Node::Node {
                left: Box::new(Node::Leaf),
                right: Box::new(Node::Node {
                    left: Box::new(Node::Leaf),
                    right: Box::new(Node::Leaf),
                }),
            },
        };
        assert_eq!(e.evaluate(), Some(num_rational(7)));
    }
}
