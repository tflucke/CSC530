#[cfg(test)]
mod pbs4_q1_tests {
    use hw2::arith::build::*;
    use hw2::arith::ty::Type::*;
    use hw2::typecheck::*;
    use std::rc::Rc;

   #[test]
   fn part1() {
       assert_eq!(typecheck(&app_e(lam_e("r", Record(vec![("x", Int)]), var_e("r")),
                                   record_e(vec![("x", num_e(2)), ("y", num_e(3))])
       ), &vec![]),
                  Ok(Record(vec![("x", Int)])));
   }

   #[test]
   fn part2() {
       assert!(typecheck(&app_e(lam_e("r", Record(vec![("x", Int), ("y", Int), ("z", Int)]),
                                      var_e("r")),
                                   var_e("a")
       ), &vec![("a", Record(vec![("x", Int), ("y", Int)]))]).is_err());
   }

   #[test]
   fn part3() {
       assert_eq!(typecheck(&app_e(var_e("f"),
                                   var_e("g")
       ), &vec![("f", Fn {
           param: Rc::new(Fn {
               param: Rc::new(Record(vec![("y", Int), ("x", Int)])),
               ret: Rc::new(Fn {
                   param: Rc::new(Record(vec![("a", Int), ("b", Int)])),
                   ret: Rc::new(Record(vec![("m", Int)]))
               })
           }),
           ret: Rc::new(Int)
       }), ("g", Fn {
           param: Rc::new(Record(vec![("y", Int)])),
           ret: Rc::new(Fn {
               param: Rc::new(Record(vec![("a", Int)])),
               ret: Rc::new(Record(vec![("n", Int), ("m", Int)]))
           })
       })]),
                  Ok(Int));
   }
}
