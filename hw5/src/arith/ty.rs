use std::rc::Rc;
use std::collections::BTreeMap;

#[derive(PartialEq,Debug,Clone)]
pub enum Type {
    Int,
    Bool,
    Unit,
    Polymorphic(&'static str),
    Fn {param: Rc<Type>, ret: Rc<Type>},
    TypeFn {param: &'static str, ret: Rc<Type>},
    Record(Vec<(&'static str, Type)>),
    Variant(BTreeMap<&'static str, Type>),
    Packed {vari: &'static str, inner: Rc<Type>},
}
