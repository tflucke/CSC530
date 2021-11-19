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
type Env = Vec<(&'static str, Option<Type>)>;

fn push_env_var(id: &'static str, t: Type, env: &Env) -> Env {
    let mut res = Vec::clone(env);
    res.push((id, Some(t)));
    return res;
}

fn push_env_polymorphic(id: &'static str, env: &Env) -> Env {
    let mut res = Vec::clone(env);
    res.push((id, None));
    return res;
}

fn is_subtype(sub: &Type, sup: &Type) -> bool {
    match (sub, sup) {
        (Bool, Bool) | (Int, Int) | (Type::Unit, Type::Unit) => true,
        (Polymorphic(a), Polymorphic(b)) => a == b,
        (Packed {vari: v1, inner: i1}, Packed {vari: v2, inner: i2}) =>
            v1 == v2 && i1 == i2,
        (Type::Record(subfields), Type::Record(supfields)) =>
            supfields.iter().all(
                |(id, sup_ty)| match subfields.iter().find(|(j, _)| id == j) {
                    Some((_, sub_ty)) => is_subtype(sub_ty, sup_ty),
                    None => false
                }),
        (Fn {param: sub_p, ret: sub_r}, Fn {param: sup_p, ret: sup_r}) =>
            is_subtype(sup_p, sub_p) && is_subtype(sub_r, sup_r),
        /* Variant Subtyping: Not implemented for this assignment */
        (Type::Variant(sub_cases),Type::Variant(sup_cases)) =>
            sub_cases == sup_cases,
        (_, _) => false
    }
}

// Old function signature kept to preserve tests.
pub fn typecheck(expr: &Rc<Expression>, env: &Vec<(&'static str, Type)>) ->
    Result<Type,String> {
    return polymorphic_typecheck(expr, &env.iter().map(
        |(id, ty)| (*id, Some(Type::clone(ty)))
    ).collect());
}

pub fn polymorphic_typecheck(expr: &Rc<Expression>, env: &Env)
                 -> Result<Type,String> {
    let tc = |x: &Rc<Expression>| polymorphic_typecheck(x, env);
    match &**expr {
        True | False => Ok(Bool),
        Expression::Unit => Ok(Type::Unit),
        Num(_) => Ok(Int),
        Var(id) => check_var(id, env),
        Add {left: n, right: m} => check_add(tc(&n), tc(&m)),
        Sub {left: n, right: m} => check_sub(tc(&n), tc(&m)),
        IsZero(n)  => check_is_zero(tc(&n)),
        If {guard: g, then_: t, else_: e} => check_if(tc(&g), tc(&t), tc(&e)),
        Lambda {param: id, param_type: t, body: b}  =>
            check_lambda(t, polymorphic_typecheck(b, &push_env_var(id,
                                                                   Type::clone(t),
                                                                   env))),
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
        PolyLambda {t: ty, body: expr} => 
            check_polylambda(ty, polymorphic_typecheck(expr, &push_env_polymorphic(ty, env))),
        PolyApp {f: polylam, t: ref arg} => check_polyapp(tc(&polylam), Type::clone(arg)),
        Pack {concrete: conc, expr, packed} =>
            check_packed(conc, tc(expr)?, packed),
        Unpack {tid, id, packed, expr} =>
            check_unpack(tid, id, tc(packed)?, expr, env),
    }
}

fn check_var(id: &'static str, env: &Env)
             -> Result<Type,String> {
    match env.iter().rev().find(|&x| x.0 == id) {
        Some((_, Some(ty))) => Ok(Type::clone(ty)),
        Some((_, None)) => Err(format!("Variable {} is a polymorphic type [T-VAR]",
                                       id)),
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
        (Ok(Bool), Ok(ref u), Ok(ref v)) if is_subtype(u, v) => Ok(Type::clone(v)),
        (Ok(Bool), Ok(ref u), Ok(ref v)) if is_subtype(v, u) => Ok(Type::clone(u)),
        (Ok(Bool), Ok(u), Ok(v)) =>
            Err(format!("If branch types {:?} and {:?} are not compatible [T-IF]",
                        u, v)),
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

fn check_polylambda(ty: &'static str, body: Result<Type,String>)
                    -> Result<Type,String> {
    match body {
        Ok(x) => Ok(TypeFn{param: ty, ret: Rc::new(x),}),
        Err(x) => Err(x),
    }
}

fn check_packed(conc: &Type, expr: Type, packed: &Type)
                -> Result<Type,String> {
    if let Packed {vari: label, inner} = packed {
        let subbed = type_subsitution(label, Type::clone(conc), Type::clone(inner));
        if subbed == expr {
            Ok(Type::clone(packed))
        }
        else {
            Err(format!("Types {:?} and {:?} do not match. [T-PACK].",
                        subbed, expr))
        }
    }
    else {
        Err(format!("Cannot pack into type {:?}. [T-PACK]", packed))
    }
}

fn type_subsitution(label: &'static str, new_ty: Type, subject: Type) -> Type {
    match subject {
        Polymorphic(p) if p == label => new_ty,
        TypeFn {param: l, ret: ref r} if l != label => TypeFn {
            param: l,
            ret: Rc::new(type_subsitution(label, new_ty, Type::clone(&*r)))
        },
        // Packed {vari: v, inner: i} if v != label =>
        //     Packed {v, type_subsitution(label, new_ty, i)},
        Int | Bool | Type::Unit | Polymorphic(_) | 
        TypeFn {param: _, ret: _} => subject, // | Packed
        Fn {param: p, ret: r} => Fn {
            param: Rc::new(type_subsitution(label,
                                            Type::clone(&new_ty),
                                            Type::clone(&*p))),
            ret: Rc::new(type_subsitution(label,
                                          Type::clone(&new_ty),
                                          Type::clone(&*r)))
        },
        Type::Record(attrs) => Type::Record(attrs.iter().map(
            |(n, a)| (*n, type_subsitution(label,
                                           Type::clone(&new_ty),
                                           Type::clone(a)))
        ).collect()),
        Type::Variant(tags) => Type::Variant(tags.iter().map(
            |(n, t)| (*n, type_subsitution(label,
                                           Type::clone(&new_ty),
                                           Type::clone(t)))
        ).collect()),
        Packed {vari: v, inner: i} => Packed {vari: v, inner: Rc::clone(&i)}
    }
}

fn check_polyapp(lam: Result<Type,String>, arg: Type) -> Result<Type,String> {
    match lam {
        Ok(TypeFn {param: label, ret: r}) =>
            Ok(type_subsitution(label, arg, Type::clone(&*r))),
        Ok(x) => Err(format!("Type {:?} is not a polymorphic function. [T-TAPP]",
                             x)),
        Err(x) => Err(x),
    }
}

fn check_unpack(label: &'static str, id: &'static str, packed: Type,
                expr: &Rc<Expression>, env: &Env) -> Result<Type,String> {
    if let Packed {vari: _, inner} = packed {
        polymorphic_typecheck(expr,
                              &push_env_var(id, Type::clone(&inner),
                                            &push_env_polymorphic(label, env)))
    }
    else {
        Err(format!("Cannot unpack type {:?}. [T-UNPACK]", packed))
    }
}

fn check_app(lam: Result<Type,String>, arg: Result<Type,String>)
             -> Result<Type,String> {
    match (lam, arg) {
        (Ok(Fn {param: p, ret: r}), Ok(x)) => if is_subtype(&x, &p) {
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
              env: &Env)
              -> Result<Type,String> {
    match test {
        Err(x) => Err(x),
        Ok(Type::Variant(tags)) => match ts.iter().map(
            |(tag, (id, e))|
            match tags.get(tag) {
                None => Err(format!("Tag {} not found in variant. [T-CASE]", tag)),
                Some(ty) =>
                    match polymorphic_typecheck(e, &push_env_var(id, Type::clone(ty), env)) {
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
