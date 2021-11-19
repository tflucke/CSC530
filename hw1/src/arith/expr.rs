use std::rc::Rc;

#[derive(PartialEq,Debug)]
pub enum Expression {
   True,
   False,
   If {guard: Rc<Expression>, then_: Rc<Expression>, else_: Rc<Expression>},
   Num (i32),
   Add {left: Rc<Expression>, right: Rc<Expression>},
   Sub {left: Rc<Expression>, right: Rc<Expression>},
   IsZero (Rc<Expression>),
}

