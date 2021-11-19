#[cfg(test)]
mod tflucke_4_tests {
    use hw2::arith::build::*;
    use hw2::arith::ty::Type::*;
    use hw2::typecheck::*;
    use std::rc::Rc;

   #[test]
   fn type_mismatch() {
       assert!(typecheck(&app_e(lam_e("r", Record(vec![("x", Int)]), var_e("r")),
                                   record_e(vec![("x", true_e()), ("y", num_e(3))])
       ), &vec![]).is_err());
   }

   #[test]
   fn fn_type_mismatch() {
       assert!(typecheck(&app_e(var_e("f"),
                                   var_e("g")
       ), &vec![("f", Fn {
           param: Rc::new(Fn {
               param: Rc::new(Record(vec![("y", Int)])),
               ret: Rc::new(Fn {
                   param: Rc::new(Record(vec![("a", Int), ("b", Int)])),
                   ret: Rc::new(Record(vec![("m", Int)]))
               })
           }),
           ret: Rc::new(Int)
       }), ("g", Fn {
           param: Rc::new(Record(vec![("y", Int), ("x", Int)])),
           ret: Rc::new(Fn {
               param: Rc::new(Record(vec![("a", Int)])),
               ret: Rc::new(Record(vec![("n", Int), ("m", Int)]))
           })
       })]).is_err());
   }
}
