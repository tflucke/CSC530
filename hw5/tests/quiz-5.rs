#[cfg(test)]
mod tflucke_4_tests {
    use hw2::arith::build::*;
    use hw2::arith::ty::Type::*;
    use hw2::typecheck::*;
    use std::rc::Rc;

   #[test]
   fn quiz5_prob3_part1() {
       assert!(typecheck(
           &unpack_e("Counter", "counter",
                     pack_e(Int, record_e(vec![
                         ("new", num_e(0)),
                         ("inc", lam_e("v", Int, add_e(var_e("v"), num_e(1)))),
                         ("get", lam_e("v", Int, var_e("v")))
                     ]), Packed {vari: "Counter", inner: Rc::new(Record(vec![
                         ("new", Polymorphic("Counter")),
                         ("inc", Fn {param: Rc::new(Int),
                                     ret: Rc::new(Polymorphic("Counter"))}),
                         ("get", Fn {param: Rc::new(Polymorphic("Counter")),
                                     ret: Rc::new(Int)})
                     ]))}),
                     app_e(project_e(var_e("counter"), "get"),
                           app_e(project_e(var_e("counter"), "inc"),
                                 project_e(var_e("counter"), "new")))
           ), &vec![]).is_err()
       );
   }

   #[test]
   fn quiz5_prob3_part2() {
       assert!(typecheck(
           &unpack_e("Counter", "counter",
                     pack_e(Int, record_e(vec![
                         ("new", num_e(0)),
                         ("inc", lam_e("v", Int, add_e(var_e("v"), num_e(1)))),
                         ("get", lam_e("v", Int, var_e("v")))
                     ]), Packed {vari: "Counter", inner: Rc::new(Record(vec![
                         ("new", Int),
                         ("inc", Fn {param: Rc::new(Polymorphic("Counter")),
                                     ret: Rc::new(Polymorphic("Counter"))}),
                         ("get", Fn {param: Rc::new(Polymorphic("Counter")),
                                     ret: Rc::new(Int)})
                     ]))}),
                     app_e(project_e(var_e("counter"), "get"),
                           app_e(project_e(var_e("counter"), "inc"),
                                 project_e(var_e("counter"), "new")))
           ), &vec![]).is_err()
       );
   }

   #[test]
   fn quiz5_prob3_part3() {
       assert_eq!(typecheck(
           &unpack_e("Counter", "counter",
                     pack_e(Int, record_e(vec![
                         ("new", num_e(0)),
                         ("inc", lam_e("v", Int, add_e(var_e("v"), num_e(1)))),
                         ("get", lam_e("v", Int, var_e("v")))
                     ]), Packed {vari: "Counter", inner: Rc::new(Record(vec![
                         ("new", Polymorphic("Counter")),
                         ("inc", Fn {param: Rc::new(Polymorphic("Counter")),
                                     ret: Rc::new(Polymorphic("Counter"))}),
                         ("get", Fn {param: Rc::new(Polymorphic("Counter")),
                                     ret: Rc::new(Int)})
                     ]))}),
                     app_e(project_e(var_e("counter"), "get"),
                           app_e(project_e(var_e("counter"), "inc"),
                                 project_e(var_e("counter"), "new")))
           ), &vec![]), Ok(Int)
       );
   }

   #[test]
   fn quiz5_prob3_part4() {
       assert!(typecheck(
           &unpack_e("Counter", "counter",
                     pack_e(Int, record_e(vec![
                         ("new", num_e(0)),
                         ("inc", lam_e("v", Int, add_e(var_e("v"), num_e(1)))),
                         ("get", lam_e("v", Int, var_e("v")))
                     ]), Packed {vari: "Counter", inner: Rc::new(Record(vec![
                         ("new", Polymorphic("Counter")),
                         ("inc", Fn {param: Rc::new(Polymorphic("Counter")),
                                     ret: Rc::new(Polymorphic("Counter"))}),
                         ("get", Fn {param: Rc::new(Int),
                                     ret: Rc::new(Int)})
                     ]))}),
                     app_e(project_e(var_e("counter"), "get"),
                           app_e(project_e(var_e("counter"), "inc"),
                                 project_e(var_e("counter"), "new")))
           ), &vec![]).is_err()
       );
   }

   #[test]
   fn quiz5_prob3_part5() {
       assert!(typecheck(
           &unpack_e("Counter", "counter",
                     pack_e(Int, record_e(vec![
                         ("new", num_e(0)),
                         ("inc", lam_e("v", Int, add_e(var_e("v"), num_e(1)))),
                         ("get", lam_e("v", Int, var_e("v")))
                     ]), Packed {vari: "Counter", inner: Rc::new(Record(vec![
                         ("new", Polymorphic("Counter")),
                         ("inc", Fn {param: Rc::new(Polymorphic("Counter")),
                                     ret: Rc::new(Polymorphic("Counter"))}),
                         ("get", Fn {param: Rc::new(Polymorphic("Counter")),
                                     ret: Rc::new(Int)})
                     ]))}),
                     app_e(project_e(var_e("counter"), "get"),
                           app_e(project_e(var_e("counter"), "inc"),
                                 num_e(0)))
           ), &vec![]).is_err()
       );
   }
}
