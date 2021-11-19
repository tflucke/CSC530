#[macro_use]
extern crate time_test;

#[cfg(test)]
mod part5_tests {
   
   use hw1::data::List;
   use hw1::part5a::BoxList;
   use hw1::part5b::RcList;

   const MAX: i32 = 1000;

   fn check_list<T>(input: fn () -> T) where T: List<T> {
      let mut list = input();
      for i in (1..MAX).rev() {
         list = list.cons(i);
      }
      assert_eq!(list.hd(), Some(1));

      let mut smaller = list.tl().expect("empty list!");
      for _ in (1..MAX - 2).rev() {
         smaller = smaller.tl().expect("empty list!");
      }

      assert_eq!(list.hd(), Some(1));
      assert_eq!(smaller.hd(), Some(MAX - 1));
   }

   #[test]
   fn check_box_list1() {
      time_test!();
      check_list(BoxList::null);
   }

   #[test]
   fn check_rc_list1() {
      time_test!();
      check_list(RcList::null);
   }
}

