use std::rc::Rc;

use super::arith::expr::{Expression,Expression::*};
use super::arith::ty::{Type,Type::*};

/* Better implementation:
 * Use an immutable linked list that appends to the list rather than creates a
 * new vector.
 * Reason not used:
 * Standard library didn't seem to support it well and I didn't want to rewrite an
 * entire linked list structure for this assignment.
 */
fn push_env(id: &'static str, t: Type, env: &Vec<(&'static str, Type)>)
            -> Vec<(&'static str, Type)> {
    let mut res = Vec::clone(env);
    res.push((id, t));
    return res;
}

pub fn typecheck(expr: &Rc<Expression>, env: &Vec<(&'static str, Type)>)
                 -> Result<Type,String> {
    let tc = |x: &Rc<Expression>| typecheck(x, env);
    match &**expr {
        True | False => Ok(Bool),
        Num(_) => Ok(Int),
        Var(id) => match env.iter().rev().find(|&x| x.0 == *id) {
            Some((_, ty)) => Ok(Type::clone(ty)),
            None => Err(format!("Unbound variable {} [T-VAR]", id))
        },
        Add {left: n, right: m} => match (tc(&n), tc(&m)) {
            (Ok(Int), Ok(Int)) => Ok(Int),
            (Ok(x), Ok(y)) => Err(format!("Cannot add types {:?} and {:?} \
                                          [T-ADD]", x, y)),
            (Err(x), _) | (_, Err(x)) => Err(x),
        },
        Sub {left: n, right: m} => match (tc(&n), tc(&m)) {
            (Ok(Int), Ok(Int)) => Ok(Int),
            (Ok(x), Ok(y)) => Err(format!("Cannot subtract types {:?} and {:?} \
                                          [T-SUB]", x, y)),
            (Err(x), _) | (_, Err(x)) => Err(x),
        },
        IsZero(n)  => match tc(&n) {
            Ok(Int) => Ok(Bool),
            Err(x) => Err(x),
            Ok(_) => Err(format!("Cannot check zero-state of expression {:?} \
                                 [T-ISZERO]", n)),
        },
        If {guard: g, then_: t, else_: e} => match tc(&g) {
            Ok(Bool) => match tc(&t) {
                Err(x) => Err(x),
                Ok(u) => match tc(&e) {
                    Err(x) => Err(x),
                    Ok(ref v) if u == *v => Ok(u),
                    Ok(v) => Err(
                        format!("If branch types {:?} and {:?} do not match \
                                [T-IF]", u, v)
                    )
                },
            },
            Err(x) => Err(x),
            Ok(_) => Err(format!("Invalid if guard {:?} [T-IF]", g)),
        },
        Lambda {param: id, param_type: ty, body: b}  =>
            match typecheck(b, &push_env(id, Type::clone(ty), env)) {
                Ok(x) => Ok(Fn{param: Rc::new(Type::clone(ty)), ret: Rc::new(x),}),
                Err(x) => Err(x),
            },
        App {f: lam, arg: a}  =>
            match tc(lam) {
                Ok(Fn {param: p, ret: r}) => match tc(a) {
                    Ok(x) => if x == *p {
                        Ok(Type::clone(&*r))
                    }
                    else {
                        Err(format!("Type {:?} does not match argument type \
                                          {:?} [T-APP]", x, p))
                    },
                    Err(x) => Err(x),
                },
                Ok(x) => Err(format!("Cannot apply {:?} type [T-APP]", x)),
                Err(x) => Err(x),
            },
    }
}
