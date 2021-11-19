use std::rc::Rc;
use std::collections::BTreeMap;

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
        Expression::Unit => Ok(Type::Unit),
        Num(_) => Ok(Int),
        Var(id) => check_var(id, env),
        Add {left: n, right: m} => check_add(tc(&n), tc(&m)),
        Sub {left: n, right: m} => check_sub(tc(&n), tc(&m)),
        IsZero(n)  => check_is_zero(tc(&n)),
        If {guard: g, then_: t, else_: e} => check_if(tc(&g), tc(&t), tc(&e)),
        Lambda {param: id, param_type: ty, body: b}  =>
            check_lambda(ty, typecheck(b, &push_env(id, Type::clone(ty), env))),
        App {f: lam, arg: a}  => check_app(tc(&lam), tc(&a)),
        Expression::Record(fs)  => check_record(fs.iter().map(
            |(id, e)| match tc(&e) {
                Ok(ty) => Ok((*id, ty)),
                Err(x) => Err(x)
            }
        ).collect()),
        Project {rec: p, id: f}  => check_project(tc(&p), f),
        Expression::Variant {tag: ta, value: val, ty: t} =>
            check_variant(ta, tc(&val), Type::clone(t)),
        Case {value: val, tags: ts} => check_case(tc(&val), ts, env),
    }
}

fn check_var(id: &'static str, env: &Vec<(&'static str, Type)>)
             -> Result<Type,String> {
    match env.iter().rev().find(|&x| x.0 == id) {
        Some((_, ty)) => Ok(Type::clone(ty)),
        None => Err(format!("Unbound variable {} [T-VAR]", id))
    }
}

fn check_add(l: Result<Type,String>, r: Result<Type,String>)
             -> Result<Type,String> {
    match (l, r) {
        (Ok(Int), Ok(Int)) => Ok(Int),
        (Ok(x), Ok(y)) => Err(format!("Cannot add types {:?} and {:?} \
                                       [T-ADD]", x, y)),
        (Err(x), _) | (_, Err(x)) => Err(x),
    }
}

fn check_sub(l: Result<Type,String>, r: Result<Type,String>)
             -> Result<Type,String> {
    match (l, r) {
        (Ok(Int), Ok(Int)) => Ok(Int),
        (Ok(x), Ok(y)) => Err(format!("Cannot subtract types {:?} and {:?} \
                                       [T-SUB]", x, y)),
        (Err(x), _) | (_, Err(x)) => Err(x),
    }
}

fn check_is_zero(b: Result<Type,String>) -> Result<Type,String> {
    match b {
        Ok(Int) => Ok(Bool),
        Err(x) => Err(x),
        Ok(x) => Err(format!("Cannot check zero-state of expression type {:?} \
                              [T-ISZERO]", x)),
    }
}

fn check_if(g: Result<Type,String>, t: Result<Type,String>, e: Result<Type,String>)
            -> Result<Type,String> {
    match (g, t, e) {
        (Ok(Bool), Ok(u), Ok(v)) => if u == v {
            Ok(u)
        }
        else {
            Err(format!("If branch types {:?} and {:?} do not match [T-IF]", u, v))
        },
        (Err(x), _, _) | (_, Err(x), _) | (_, _, Err(x)) => Err(x),
        (Ok(x), _, _) => Err(format!("Invalid if guard type {:?} [T-IF]", x)),
    }
}

fn check_lambda(ty: &Type, body: Result<Type,String>) -> Result<Type,String> {
    match body {
        Ok(x) => Ok(Fn{param: Rc::new(Type::clone(ty)), ret: Rc::new(x),}),
        Err(x) => Err(x),
    }
}

fn check_app(lam: Result<Type,String>, arg: Result<Type,String>)
             -> Result<Type,String> {
    match (lam, arg) {
        (Ok(Fn {param: p, ret: r}), Ok(x)) => if x == *p {
            Ok(Type::clone(&*r))
        }
        else {
            Err(format!("Type {:?} does not match argument type \
                         {:?} [T-APP]", x, p))
        },
        (Ok(x), Ok(_)) => Err(format!("Cannot apply {:?} type [T-APP]", x)),
        (Err(x), _) | (_, Err(x)) => Err(x),
    }
}

fn check_record(fields: Result<Vec<(&'static str, Type)>, String>)
                -> Result<Type,String> {
    match fields {
        Ok(types) => Ok(Type::Record(types)),
        Err(x) => Err(x),
    }
}

fn check_project(rec: Result<Type,String>, id: &'static str)
                 -> Result<Type,String> {
    match rec {
        Ok(Type::Record(vec)) => match vec.iter().find_map(
            |(f, ty)| if *f == id { Some(ty) } else { None }
        ) {
            Some(ty) => Ok(Type::clone(ty)),
            None => Err(format!("Record type {:?} does not have a field {} \
                                 [T-PROJ]", Type::Record(vec), id))
        },
        Ok(x) => Err(format!("Cannot project into type {:?} [T-PROJ]", x)),
        Err(x) => Err(x)
    }
}

fn check_variant(act_tag: &'static str, val: Result<Type,String>, ty: Type)
                 -> Result<Type,String> {
    match (&ty, val) {
        (Type::Variant(types), Ok(ref act_ty)) => match types.iter().find_map(
            |(tag, typ)| if *tag == act_tag { Some(typ) } else { None }
        ) {
            Some(exp_ty) if *exp_ty == *act_ty => Ok(ty),
            Some(exp_ty) => Err(format!("Type {:?} did not match expected type \
                                         {:?} [T-VARIANT]", act_ty, exp_ty)),
            None => Err(format!("Tag {} not found in variant {:?} [T-VARIANT]",
                                act_tag, ty))
        }
        (Type::Variant(_), Err(x)) => Err(x),
        (x, _) => Err(format!("Type {:?} is not a variant [T-VARIANT]", x)),
    }
}

fn check_case(test: Result<Type,String>,
              ts: &BTreeMap<&'static str, (&'static str, Rc<Expression>)>,
              env: &Vec<(&'static str, Type)>)
              -> Result<Type,String> {
    match test {
        Err(x) => Err(x),
        Ok(Type::Variant(tags)) => match ts.iter().map(
            |(tag, (id, e))|
            match tags.get(tag) {
                None => Err(format!("Tag {} not found in variant. [T-CASE]", tag)),
                Some(ty) =>
                    match typecheck(e, &push_env(id, Type::clone(ty), env)) {
                        Ok(res_ty) => Ok((*tag, res_ty)),
                        Err(x) => Err(x),
                    }
            }).collect::<Result<BTreeMap<&'static str,Type>,String>>() {
            Ok(cases) => {
                let (_, res) = &cases.iter().next().unwrap();
                let not_covered: Vec<&'static str> = tags.iter()
                    .filter(|(t, _)| !cases.contains_key(*t))
                    .map(|(x, _)| *x).collect();
                if !cases.iter().all(|(_, x)| x == *res) {
                    Err("Cases do not have the same type. [T-CASE]".to_string())
                }
                else if !not_covered.is_empty() {
                    Err(format!("Variant case {} not covered. [T-CASE]",
                                not_covered[0]))
                }
                else {
                    Ok(Type::clone(*res))
                }
            },
            Err(x) => Err(x),
        },
        Ok(x) => Err(format!("Type {:?} is not a variant. [T-CASE]", x)),
    }
}
