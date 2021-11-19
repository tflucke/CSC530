use std::rc::Rc;
use super::ty::Type;
use std::collections::BTreeMap;

#[derive(PartialEq,Debug)]
pub enum Expression {
   True,
   False,
   Unit,
   If {guard: Rc<Expression>, then_: Rc<Expression>, else_: Rc<Expression>},
   Num (i32),
   Var (&'static str),
   Add {left: Rc<Expression>, right: Rc<Expression>},
   Sub {left: Rc<Expression>, right: Rc<Expression>},
   IsZero (Rc<Expression>),
   Lambda {param: &'static str, param_type: Type, body: Rc<Expression>},
   App {f: Rc<Expression>, arg: Rc<Expression>},
   Record(Vec<(&'static str, Rc<Expression>)>),
   Project {rec: Rc<Expression>, id: &'static str},
   Variant {tag: &'static str, value: Rc<Expression>, ty: Type},
    Case {
        value: Rc<Expression>,
        tags: BTreeMap<&'static str, (&'static str, Rc<Expression>)>
    },
   PolyLambda {t: &'static str, body: Rc<Expression>},
   PolyApp {f: Rc<Expression>, t: Type},
   Pack {concrete: Type, expr: Rc<Expression>, packed: Type},
   Unpack {tid: &'static str, id: &'static str,
           packed: Rc<Expression>, expr: Rc<Expression>},
}
