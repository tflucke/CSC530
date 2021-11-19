#[cfg(test)]
mod tests {
   use hw1::arith::expr::Expression;
   use hw1::arith::build::*;
   use hw1::eval::*;
   use std::rc::Rc;

   #[test]
   fn single1() {
      assert_eq!(single_step(
            add_e(add_e(num_e(9), num_e(10)), add_e(num_e(1), num_e(2)))),
         add_e(num_e(19), add_e(num_e(1), num_e(2))));
   }

   #[test]
   fn single2() {
      let sub : Rc<Expression> =
            add_e(add_e(num_e(9), num_e(10)), add_e(num_e(1), num_e(2)));

      assert_eq!(single_step(single_step(sub)), add_e(num_e(19), num_e(3)));
   }

   #[test]
   fn single3() {
      let sub : Rc<Expression> =
            add_e(add_e(num_e(1), num_e(2)), num_e(10));

      assert_eq!(single_step(sub), add_e(num_e(3), num_e(10)));
   }

   #[test]
   fn single4() {
      let sub : Rc<Expression> =
            add_e(num_e(3), true_e());

      assert_eq!(single_step(sub), add_e(num_e(3), true_e()));
   }

   #[test]
   fn single5() {
      assert_eq!(single_step(
         if_e(is_zero_e(
               add_e(add_e(num_e(9), num_e(10)), add_e(num_e(1), num_e(2)))),
            add_e(num_e(10), num_e(3)),
            is_zero_e(num_e(4)))),
         if_e(is_zero_e(add_e(num_e(19), add_e(num_e(1), num_e(2)))),
            add_e(num_e(10), num_e(3)),
            is_zero_e(num_e(4)))
         );
   }

   #[test]
   fn eval1() {
      assert_eq!(eval(
            add_e(add_e(num_e(9), num_e(10)), add_e(num_e(1), num_e(2)))),
         Ok(num_e(22)));
   }

   #[test]
   fn eval2() {
      assert_eq!(eval(is_zero_e(
            add_e(add_e(num_e(9), num_e(10)), add_e(num_e(1), num_e(2))))),
         Ok(false_e()));
   }

   #[test]
   fn eval3() {
      let sub : Rc<Expression> =
            add_e(add_e(num_e(9), num_e(10)),
               is_zero_e(
                  sub_e(
                     add_e(num_e(9), num_e(10)),
                     sub_e(num_e(21), num_e(2)))));

      assert_eq!(eval(sub), Err(add_e(num_e(19), true_e())));
   }

   #[test]
   fn if1() {
      assert_eq!(eval(if_e(is_zero_e(
            add_e(add_e(num_e(9), num_e(10)), add_e(num_e(1), num_e(2)))),
            add_e(num_e(10), num_e(3)), is_zero_e(num_e(4)))),
         Ok(false_e()));
   }

   #[test]
   fn if2() {
      assert_eq!(eval(add_e(if_e(is_zero_e(
            add_e(add_e(num_e(9), num_e(10)), add_e(num_e(1), num_e(2)))),
            add_e(num_e(10), num_e(3)), is_zero_e(num_e(4))), num_e(10))),
         Err(add_e(false_e(), num_e(10))));
   }
}
