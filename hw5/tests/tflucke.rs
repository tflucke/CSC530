#[cfg(test)]
mod tflucke_tests {
    use hw2::arith::build::*;
    use hw2::arith::ty::Type::*;
    use hw2::typecheck::*;

   #[test]
   fn fn_app() {
       assert_eq!(typecheck(
           &app_e(lam_e("x", Int, if_e(
               is_zero_e(var_e("x")),
               num_e(0),
               add_e(var_e("x"), num_e(1)))
           ), num_e(3)),
           &vec![]), Ok(Int));
   }

   #[test]
   fn free_var() {
       assert!(typecheck(
           &app_e(lam_e("x", Int, if_e(
               is_zero_e(var_e("x")),
               num_e(0),
               add_e(var_e("y"), num_e(1)))
           ), num_e(3)),
           &vec![]).is_err());
   }

   #[test]
   fn free_var_in_env() {
       assert_eq!(typecheck(
           &app_e(lam_e("x", Int, if_e(
               is_zero_e(var_e("x")),
               num_e(0),
               add_e(var_e("y"), num_e(1)))
           ), num_e(3)),
           &vec![("y", Int)]), Ok(Int));
   }

   #[test]
   fn if_mixed_type() {
       assert!(typecheck(
           &app_e(lam_e("x", Int, if_e(
               is_zero_e(var_e("x")),
               false_e(),
               add_e(var_e("y"), num_e(1)))
           ), num_e(3)),
           &vec![("y", Int)]).is_err());
   }

   #[test]
   fn is_zero_bool() {
       assert!(typecheck(
           &app_e(lam_e("x", Int, if_e(
               is_zero_e(is_zero_e(var_e("x"))),
               num_e(0),
               add_e(var_e("x"), num_e(1)))
           ), num_e(3)),
           &vec![]).is_err());
   }

   #[test]
   fn shadowing() {
       assert_eq!(typecheck(
           &app_e(lam_e("x", Int, if_e(
               is_zero_e(var_e("x")),
               num_e(0),
               add_e(var_e("x"), num_e(1)))
           ), num_e(3)),
           &vec![("x", Bool)]), Ok(Int));
   }

   #[test]
   fn shadowing_context() {
       assert_eq!(typecheck(
           &app_e(lam_e("y", Bool,
                        app_e(lam_e("x", Int, if_e(
                            var_e("y"),
                            num_e(0),
                            add_e(var_e("x"), num_e(1)))
                        ), num_e(3))
           ), var_e("x")),
           &vec![("x", Bool)]), Ok(Int));
   }

   #[test]
   fn app_non_fn() {
       assert!(typecheck(
           &app_e(num_e(3), var_e("x")),
           &vec![("x", Bool)]).is_err());
   }

   #[test]
   fn add_left_side_errs() {
       assert!(typecheck(
           &add_e(app_e(num_e(3), var_e("x")),
                  num_e(3)),
           &vec![]).is_err());
   }

   #[test]
   fn add_right_side_errs() {
       assert!(typecheck(
           &add_e(num_e(3),
                  app_e(num_e(3), var_e("x"))),
           &vec![]).is_err());
   }

   #[test]
   fn add() {
       assert_eq!(typecheck(
           &add_e(num_e(3),
                  num_e(-34)),
           &vec![]), Ok(Int));
   }

   #[test]
   fn add_bool() {
       assert!(typecheck(
           &add_e(num_e(3),
                  true_e()),
           &vec![]).is_err());
   }
    
   #[test]
   fn sub_left_side_errs() {
       assert!(typecheck(
           &sub_e(app_e(num_e(3), var_e("x")),
                  num_e(3)),
           &vec![]).is_err());
   }

   #[test]
   fn sub_right_side_errs() {
       assert!(typecheck(
           &sub_e(num_e(3),
                  app_e(num_e(3), var_e("x"))),
           &vec![]).is_err());
   }

   #[test]
   fn sub() {
       assert_eq!(typecheck(
           &sub_e(num_e(3),
                  num_e(-34)),
           &vec![]), Ok(Int));
   }

   #[test]
   fn sub_bool() {
       assert!(typecheck(
           &sub_e(num_e(3),
                  true_e()),
           &vec![]).is_err());
   }
}
