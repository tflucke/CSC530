use super::data::List;

#[derive(PartialEq,Debug,Clone)]
pub struct BoxList {
   pub head: Option<Box<(i32,BoxList)>>,
}

impl List<BoxList> for BoxList {
   fn null() -> Self {
      BoxList {head: None}
   }

   fn hd(&self) -> Option<i32> {
      match &self.head {
         None => None,
         Some(box_node) => Some(box_node.0),
      }
   }

   fn tl(&self) -> Option<BoxList> {
      match &self.head {
         None => None,
         Some(box_node) => Some(box_node.1.clone()), // !!!! CLONE?!?!
                                                     // -- single owner
      }
   }

   fn cons(&self, value: i32) -> Self {
      BoxList {head: Some(Box::new((value, self.clone())))}
                                          // !! ^^^^^ AGAIN !!
   }

    fn append(&self, other: &Self) -> Self {
       match &self.head {
           None => other.clone(),
           Some(boxn) => BoxList {
               head: Some(Box::new((boxn.0, boxn.1.append(other))))
           },
       }
   }
}
