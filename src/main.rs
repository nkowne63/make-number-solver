pub(crate) mod exp_traverse;
pub(crate) mod expression;
pub(crate) mod num_traverse;
pub(crate) mod ops_traverse;
pub(crate) mod parse;
pub(crate) mod rational;

use crate::exp_traverse::ExpressionIter;
use crate::parse::prompt_input;
use crate::rational::Rational;

use crate::expression::RationalEvaluatable;

fn main() {
    let (values, target) = prompt_input();
    let values = values
        .into_iter()
        .map(|i| Rational::new(i as i32, 1))
        .collect();
    let target = Rational::new(target as i32, 1);
    for e in ExpressionIter::new(values) {
        if e.evaluate() == target {
            println!("{}", e);
            break;
        }
    }
}
