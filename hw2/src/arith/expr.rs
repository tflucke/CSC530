use std::rc::Rc;
use super::ty::Type;

#[derive(PartialEq,Debug)]
pub enum Expression {
   True,
   False,
   If {guard: Rc<Expression>, then_: Rc<Expression>, else_: Rc<Expression>},
   Num (i32),
   Var (&'static str),
   Add {left: Rc<Expression>, right: Rc<Expression>},
   Sub {left: Rc<Expression>, right: Rc<Expression>},
   IsZero (Rc<Expression>),
   Lambda {param: &'static str, param_type: Type, body: Rc<Expression>},
   App {f: Rc<Expression>, arg: Rc<Expression>},
}

