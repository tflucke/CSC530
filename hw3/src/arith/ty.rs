use std::rc::Rc;
use std::collections::BTreeMap;

#[derive(PartialEq,Debug,Clone)]
pub enum Type {
    Int,
    Bool,
    Unit,
    Fn {param: Rc<Type>, ret: Rc<Type>},
    Record(Vec<(&'static str, Type)>),
    Variant(BTreeMap<&'static str, Type>),
}
