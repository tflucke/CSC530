use std::rc::Rc;
use super::arith::expr::Expression;

pub fn gather_nums(expr: &Rc<Expression>) -> Vec<i32> {
    match &**expr {
        Expression::True => vec![],
        Expression::False => vec![],
        Expression::Num(i) => vec![*i],
        Expression::IsZero(e) => gather_nums(&e),
        Expression::Add {left: l, right: r} =>
            gather_nums(&l).into_iter()
            .chain(gather_nums(&r).into_iter()).collect(),
        Expression::Sub {left: l, right: r} =>
            gather_nums(&l).into_iter()
            .chain(gather_nums(&r).into_iter()).collect(),
        Expression::If {guard: g, then_: t, else_: e} =>
            gather_nums(&g).into_iter()
            .chain(gather_nums(&t).into_iter())
            .chain(gather_nums(&e).into_iter()).collect(),
    }
}
