#[cfg(test)]
mod pbs2_q1_tests {
    use hw2::arith::build::*;
    use hw2::arith::ty::{Type,Type::*};
    use hw2::typecheck::*;
    use std::rc::Rc;

   #[test]
   fn part1() {
      assert_eq!(typecheck(&app_e(lam_e("x", Int, add_e(var_e("x"), num_e(1))),
                                  num_e(0)),
                           &vec![]), Ok(Type::Int));
   }
    
   #[test]
   fn part2() {
      assert!(typecheck(&app_e(lam_e("x", Int, add_e(var_e("x"), num_e(1))),
                                  true_e()),
                           &vec![]).is_err());
   }
    
   #[test]
   fn part3() {
       assert_eq!(typecheck(
           &app_e(
               lam_e("x", Int, app_e(app_e(var_e("times"),
                                           var_e("x")),
                                     num_e(0))
               ),
               add_e(num_e(0), num_e(1))),
           &vec![("times", Fn {
               param: Rc::new(Int),
               ret: Rc::new(Fn {param: Rc::new(Int), ret: Rc::new(Int),}),})
           ]), Ok(Type::Int));
   }
    
   #[test]
   fn part4() {
       assert!(typecheck(
           &app_e(
               lam_e("x", Bool,
                     if_e(var_e("x"),
                          app_e(lam_e("x", Int, add_e(var_e("x"), num_e(1))),
                                num_e(0)),
                          app_e(lam_e("x", Fn {
                              param: Rc::new(Int),
                              ret: Rc::new(Bool),},
                                      app_e(var_e("x"), num_e(0))),
                                lam_e("y", Int, var_e("x"))
                          ))
               ),true_e()
           ), &vec![("x", Int)]).is_err());
   }
    
   #[test]
   fn part4_noerror() {
       assert_eq!(typecheck(
           &app_e(
               lam_e("x", Bool,
                     if_e(var_e("x"),
                          app_e(lam_e("x", Int, false_e()), num_e(0)),
                          app_e(lam_e("x", Fn {
                              param: Rc::new(Int),
                              ret: Rc::new(Bool),},
                                      app_e(var_e("x"), num_e(0))),
                                lam_e("y", Int, var_e("x"))
                          ))
               ),true_e()
           ), &vec![("x", Int)]), Ok(Type::Bool));
   }
}
