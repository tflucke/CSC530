#[cfg(test)]
mod part7_tests {
   use hw1::arith::expr::Expression;
   use hw1::arith::build::*;
   use hw1::part7::gather_nums;
   use std::rc::Rc;

   #[test]
   fn gather_nums1() {
      assert_eq!(gather_nums(
            &add_e(add_e(num_e(9), num_e(10)), add_e(num_e(1), num_e(2)))),
         vec![9,10,1,2]);
   }

   #[test]
   fn gather_nums2() {
      let sub : Rc<Expression> =
            add_e(add_e(num_e(1), num_e(2)), num_e(10));

      assert_eq!(gather_nums(&sub), vec![1,2,10]);
   }

   #[test]
   fn gather_nums3() {
      let sub : Rc<Expression> =
            add_e(num_e(3), true_e());

      assert_eq!(gather_nums(&sub), vec![3]);
   }

   #[test]
   fn gather_nums4() {
      assert_eq!(gather_nums(
         &if_e(is_zero_e(
               add_e(add_e(num_e(9), num_e(10)), add_e(num_e(1), num_e(2)))),
            add_e(num_e(10), num_e(3)),
            is_zero_e(num_e(4)))),
         vec![9,10,1,2,10,3,4]);
   }

   #[test]
   fn gather_nums5() {
      let sub : Rc<Expression> =
            add_e(add_e(num_e(9), num_e(10)),
               is_zero_e(
                  sub_e(
                     add_e(num_e(9), num_e(10)),
                     sub_e(num_e(21), num_e(2)))));

      assert_eq!(gather_nums(&sub), vec![9,10,9,10,21,2]);
   }

   #[test]
   fn gather_nums6() {
      assert_eq!(gather_nums(&add_e(if_e(is_zero_e(
            add_e(add_e(num_e(9), num_e(10)), add_e(num_e(1), num_e(2)))),
            add_e(num_e(10), num_e(3)), is_zero_e(num_e(4))), num_e(10))),
         vec![9,10,1,2,10,3,4,10]);
   }
}
