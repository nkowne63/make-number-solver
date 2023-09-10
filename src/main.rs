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
    println!("calculating...");
    for e in ExpressionIter::new(Rational::from_vec(values)) {
        if e.evaluate() == target.into() {
            println!("answer >");
            println!("{}", e);
            break;
        }
    }
}
