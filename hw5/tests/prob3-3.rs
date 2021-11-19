#[cfg(test)]
mod pbs2_q3_tests {
    use hw2::arith::build::*;
    use hw2::arith::ty::Type::*;
    use hw2::typecheck::*;
    use std::rc::Rc;

   #[test]
   fn part1() {
       assert_eq!(typecheck(&variant_e("b", true_e(), variant_t(vec![
           ("b", Bool),
           ("i", Int),
           ("u", Unit)
       ])), &vec![]), Ok(variant_t(vec![
           ("b", Bool),
           ("i", Int),
           ("u", Unit)
       ])));
   }

   #[test]
   fn part2() {
       assert!(typecheck(&variant_e("x", num_e(7), variant_t(vec![
           ("b", Bool),
           ("i", Int),
           ("u", Unit)
       ])), &vec![]).is_err());
   }

   #[test]
   fn part3() {
       println!("{:?}", typecheck(&lam_e("a", variant_t(vec![
           ("b", Bool),
           ("i", Int),
           ("u", Unit)
       ]), case_e(var_e("a"), vec![
           ("b", "y", var_e("y")),
           ("i", "n", var_e("n")),
           ("u", "x", num_e(0))
       ])), &vec![]));
       assert!(typecheck(&lam_e("a", variant_t(vec![
           ("b", Bool),
           ("i", Int),
           ("u", Unit)
       ]), case_e(var_e("a"), vec![
           ("b", "y", var_e("y")),
           ("i", "n", var_e("n")),
           ("u", "x", num_e(0))
       ])), &vec![]).is_err());
   }

   #[test]
   fn part3_ret_bool() {
       assert_eq!(typecheck(&lam_e("a", variant_t(vec![
           ("b", Bool),
           ("i", Int),
           ("u", Unit)
       ]), case_e(var_e("a"), vec![
           ("b", "y", var_e("y")),
           ("i", "n", is_zero_e(var_e("n"))),
           ("u", "x", false_e())
       ])), &vec![]), Ok(Fn {
           param: Rc::new(variant_t(vec![
               ("b", Bool),
               ("i", Int),
               ("u", Unit)
           ])),
           ret: Rc::new(Bool)
       }));
   }

   #[test]
   fn part4() {
       assert!(typecheck(&lam_e("a", variant_t(vec![
           ("b", Bool),
           ("i", Int),
           ("u", Unit)
       ]), case_e(var_e("a"), vec![
           ("b", "y", num_e(0)),
           ("i", "n", var_e("n")),
       ])), &vec![]).is_err());
   }

   #[test]
   fn part4_ret_num() {
       assert_eq!(typecheck(&lam_e("a", variant_t(vec![
           ("b", Bool),
           ("i", Int),
           ("u", Unit)
       ]), case_e(var_e("a"), vec![
           ("b", "y", num_e(0)),
           ("u", "x", num_e(-1)),
           ("i", "n", var_e("n")),
       ])), &vec![]), Ok(Fn {
           param: Rc::new(variant_t(vec![
               ("b", Bool),
               ("i", Int),
               ("u", Unit)
           ])),
           ret: Rc::new(Int)
       }));
   }

    // Part 5 identical to prob 3-2
}
