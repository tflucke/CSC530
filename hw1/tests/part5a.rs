#[cfg(test)]
mod part5a_tests {
   use hw1::data::List;
   use hw1::part5a::BoxList;

   const MAX: i32 = 500;

   fn create_list(size: i32, f: fn (i32) -> i32) -> BoxList {
      let mut list = BoxList::null();
      for i in (0..size).rev() {
         list = BoxList {head: Some(Box::new((f(i),list)))};
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

