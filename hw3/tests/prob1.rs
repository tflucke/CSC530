#[cfg(test)]
mod pbs2_q1_tests {
    use hw2::arith::build::*;
    use hw2::arith::ty::{Type,Type::*};
    use hw2::typecheck::*;

   #[test]
   fn part1() {
      assert_eq!(typecheck(&is_zero_e(num_e(0)), &vec![]), Ok(Type::Bool));
   }
    
   #[test]
   fn part2() {
       assert_eq!(typecheck(&add_e(
           sub_e(add_e(num_e(0),
                       num_e(1)),
                 num_e(1)),
           num_e(1)), &vec![]),
                  Ok(Type::Int));
   }
    
   #[test]
   fn part3() {
       assert!(typecheck(&if_e(true_e(),
                               add_e(num_e(0), num_e(1)),
                               true_e()), &vec![])
                  .is_err());
   }
    
   #[test]
   fn part4() {
       assert_eq!(typecheck(&if_e(var_e("x"),
                                  add_e(num_e(0), num_e(1)),
                                  sub_e(num_e(0), num_e(1))), &vec![("x", Bool)]),
                  Ok(Int));
   }
    
   #[test]
   fn part4_bad_gaurd() {
       assert!(typecheck(&if_e(var_e("x"),
                                  add_e(num_e(0), num_e(1)),
                                  sub_e(num_e(0), num_e(1))), &vec![("x", Int)])
                  .is_err());
   }
}

