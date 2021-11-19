use super::data::List;
use std::rc::Rc;

#[derive(PartialEq,Debug,Clone)]
pub struct RcList {
   head: Option<Rc<(i32,RcList)>>,
}

impl List<RcList> for RcList {
   fn null() -> Self {
      RcList {head: None}
   }

   fn hd(&self) -> Option<i32> {
      match &self.head {
         None => None,
         Some(rc_node) => Some(rc_node.0),
      }
   }

   fn tl(&self) -> Option<RcList> {
      match &self.head {
         None => None,
         Some(rc_node) => Some(rc_node.1.clone()), // What of this clone?
      }
   }

   fn cons(&self, value: i32) -> Self {
      RcList {head: Some(Rc::new((value, self.clone())))}
                                          // And this one?
   }

   fn append(&self, other: &Self) -> Self {
       match &self.head {
           None => other.clone(),
           Some(boxn) => RcList {
               head: Some(Rc::new((boxn.0, boxn.1.append(other))))
           },
       }
   }
}
