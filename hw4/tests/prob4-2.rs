#[cfg(test)]
mod pbs4_q2_tests {
    use hw2::arith::build::*;
    use hw2::arith::ty::Type::*;
    use hw2::typecheck::*;
    use std::rc::Rc;

   #[test]
   fn part1() {
       assert_eq!(typecheck(&app_e(var_e("f"), var_e("r")
       ), &vec![("f", Fn {
           param: Rc::new(Record(vec![("x", Int), ("y", Int)])),
           ret: Rc::new(Int)
       }), ("r", Record(vec![("y", Int), ("x", Int)]))]),
                  Ok(Int));
   }
    
   #[test]
   fn part2() {
       assert_eq!(typecheck(&app_e(var_e("f"), var_e("r")
       ), &vec![("f", Fn {
           param: Rc::new(Record(vec![("x", Int)])),
           ret: Rc::new(Int)
       }), ("r", Record(vec![("y", Int), ("x", Int)]))]),
                  Ok(Int));
   }
    
   #[test]
   fn part3() {
       assert_eq!(typecheck(&app_e(var_e("f"), var_e("r")
       ), &vec![("f", Fn {
           param: Rc::new(Record(vec![("x", Record(vec![("n", Int)]))])),
           ret: Rc::new(Int)
       }), ("r", Record(vec![("x", Record(vec![("n", Int), ("m", Int)])),
                             ("y", Int)]))]),
                  Ok(Int));
   }
    
   #[test]
   fn part4() {
       assert_eq!(typecheck(&app_e(var_e("f"), var_e("r")
       ), &vec![("f", Fn {
           param: Rc::new(Record(vec![("x", Fn {
               param: Rc::new(Record(vec![("n", Int)])),
               ret: Rc::new(Record(vec![("x", Int)]))
           })])),
           ret: Rc::new(Int)
       }), ("r", Record(vec![("x", Fn {
               param: Rc::new(Record(vec![("n", Int)])),
               ret: Rc::new(Record(vec![("x", Int)]))
       })]))]),
                  Ok(Int));
   }

   #[test]
   fn part5() {
       assert_eq!(typecheck(&app_e(var_e("f"), var_e("r")
       ), &vec![("f", Fn {
           param: Rc::new(Record(vec![("x", Fn {
               param: Rc::new(Record(vec![("n", Int)])),
               ret: Rc::new(Record(vec![("x", Int)]))
           })])),
           ret: Rc::new(Int)
       }), ("r", Record(vec![("x", Fn {
               param: Rc::new(Record(vec![("n", Int)])),
               ret: Rc::new(Record(vec![("x", Int), ("y", Int)]))
       })]))]),
                  Ok(Int));
   }    
}
