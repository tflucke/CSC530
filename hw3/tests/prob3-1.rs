#[cfg(test)]
mod pbs3_q1_tests {
    use hw2::arith::build::*;
    use hw2::arith::ty::{Type,Type::*};
    use hw2::typecheck::*;
    use std::rc::Rc;

   #[test]
   fn part1() {
       let color = Record(vec![("r", Int), ("g", Int), ("b", Int)]);
       assert_eq!(typecheck(&record_e(vec![
           ("x", num_e(2)),
           ("y", num_e(3)),
           ("color", var_e("c"))
       ]), &vec![("c", Type::clone(&color))]),
                  Ok(Record(vec![
                      ("x", Int),
                      ("y", Int),
                      ("color", color)])));
   }
    
   #[test]
   fn part2() {
       let color = Record(vec![("r", Int), ("g", Int), ("b", Int)]);
       assert_eq!(typecheck(&project_e(record_e(vec![
           ("x", num_e(2)),
           ("y", num_e(3)),
           ("color", var_e("c"))
       ]), "color"), &vec![("c", Type::clone(&color))]),
                  Ok(color));
   }
    
   #[test]
   fn part3() {
       assert_eq!(typecheck(
           &lam_e("obj", Record(vec![
               ("move", Fn {
                   param: Rc::new(Record(vec![
                       ("x", Int),
                       ("y", Int)
                   ])),
                   ret: Rc::new(Record(vec![
                       ("x", Int),
                       ("y", Int)
                   ]))
               }),
               ("point", Record(vec![
                   ("x", Int),
                   ("y", Int)
               ]))
           ]), project_e(project_e(var_e("obj"), "point"), "x")),
           &vec![]), Ok(Fn {
           param: Rc::new(Record(vec![
               ("move", Fn {
                   param: Rc::new(Record(vec![
                       ("x", Int),
                       ("y", Int)
                   ])),
                   ret: Rc::new(Record(vec![
                       ("x", Int),
                       ("y", Int)
                   ]))
               }),
               ("point", Record(vec![
                   ("x", Int),
                   ("y", Int)
               ]))
           ])),
           ret: Rc::new(Int)
       }));
   }
}
