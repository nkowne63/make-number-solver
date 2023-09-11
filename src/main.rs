pub(crate) mod bin_tree;
pub(crate) mod new_ops;
pub(crate) mod new_traverse;
pub(crate) mod parse;

use crate::new_traverse::TraversalableNums;
use crate::parse::prompt_input;

fn main() {
    let (values, target) = prompt_input();
    println!("calculating...");
    for expression in TraversalableNums::new(values) {
        if expression.evaluate() == Some(target.into()) {
            println!("{} = {}", expression, target);
            break;
        }
    }
}
