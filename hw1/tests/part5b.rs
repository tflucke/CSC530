#[cfg(test)]
mod part5b_tests {
   use hw1::data::List;
   use hw1::part5b::RcList;

   const MAX: i32 = 500;

   fn create_list(size: i32, f: fn (i32) -> i32) -> RcList {
      let mut list = RcList::null();
      for i in (0..size).rev() {
         list = list.cons(f(i));
      }

      list
   }

   #[test]
   fn check_append() {
      let list1 = create_list(MAX, |x| x);
      let list2 = create_list(MAX, |x| x + MAX);

      let expected = create_list(MAX * 2, |x| x);

      assert_eq!(list1.append(&list2), expected);
   }
}

