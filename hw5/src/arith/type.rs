use std::rc::Rc;

#[derive(PartialEq,Debug,Clone)]
pub enum Type {
    Int,
    Bool,
    Fn {param: Rc<Type>, ret: Rc<Type>},
}
